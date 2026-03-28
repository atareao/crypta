use anyhow::{anyhow, Context, Result};
use git2::{IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature};
use tracing::{debug, info};

pub fn sync(secrets_dir: &str, message: Option<&str>) -> Result<()> {
    info!("Iniciando sincronización Git");
    debug!("Directorio: {}", secrets_dir);
    println!("🔄 Sincronizando con el remoto...");

    let repo = Repository::open(secrets_dir).context("No se pudo abrir el repositorio git")?;

    // Verificar si hay cambios locales y hacer commit primero
    debug!("Verificando cambios locales");
    let statuses = repo.statuses(None)?;

    if !statuses.is_empty() {
        let msg = message.unwrap_or("Sync secrets");
        info!(
            "Detectados {} cambios, creando commit antes de pull",
            statuses.len()
        );
        debug!("Mensaje de commit: {}", msg);

        // Add
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;

        // Commit
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let sig = Signature::now("crypta", "crypta@local")?;
        let parent = repo.head()?.peel_to_commit()?;

        repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&parent])?;
        debug!("Commit creado exitosamente");
    }

    // Pull con rebase
    info!("Ejecutando pull con rebase");
    pull_rebase(&repo).context("Error durante pull/rebase. Verifica que tu clave SSH esté disponible y que el agente SSH esté corriendo (comprueba `SSH_AUTH_SOCK`). Si usas un puerto no estándar, asegúrate de que la URL remota y el host sean resolvibles.")?;

    // Push
    info!("Realizando push al remoto");
    push(&repo).context("Error al hacer push. Si ves errores de autenticación, añade tu clave al agente (`ssh-add`) o usa la URL HTTPS del remoto. Comprueba también que `SSH_AUTH_SOCK` esté exportada en el entorno donde ejecutas `crypta`.")?;

    println!("🚀 Sincronización completada.");
    info!("Sincronización completada exitosamente");

    Ok(())
}

fn pull_rebase(repo: &Repository) -> Result<()> {
    debug!("Iniciando pull con rebase");

    // Fetch desde origin con callbacks SSH
    let mut remote = repo.find_remote("origin")?;
    let mut callbacks = RemoteCallbacks::new();

    // Configurar autenticación SSH
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        debug!("Solicitando credenciales SSH");
        let username = username_from_url.unwrap_or("git");
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

        // Primero intentar con claves SSH del directorio .ssh
        for key_name in ["id_ed25519", "id_rsa", "id_ecdsa"] {
            let key_path_str = format!("{}/.ssh/{}", home, key_name);
            let key_path = std::path::Path::new(&key_path_str);
            if key_path.exists() {
                debug!("Intentando con clave SSH: {}", key_path.display());
                if let Ok(cred) = git2::Cred::ssh_key(username, None, key_path, None) {
                    return Ok(cred);
                }
            }
        }

        // Si no hay claves en disco, intentar con ssh-agent
        debug!("Intentando con ssh-agent");
        git2::Cred::ssh_key_from_agent(username)
    });

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    debug!("Fetching desde origin");
    let fetch_rebase_res: Result<(), git2::Error> = (|| {
        remote.fetch(&["main"], Some(&mut fetch_options), None)?;

        // Obtener referencias
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

        // Rebase
        debug!("Ejecutando rebase");
        let mut rebase = repo.rebase(
            Some(&repo.head()?.peel_to_annotated_commit()?),
            Some(&fetch_commit),
            None,
            None,
        )?;

        let mut ops = 0;
        while let Some(_op) = rebase.next() {
            ops += 1;
            rebase.commit(None, &Signature::now("crypta", "crypta@local")?, None)?;
        }

        debug!("Aplicadas {} operaciones de rebase", ops);
        rebase.finish(None)?;
        Ok(())
    })();

    if let Err(e) = fetch_rebase_res {
        debug!("Libgit2 fetch/rebase failed: {}", e);

        // Fallback: try system `git pull --rebase` in the working directory.
        let git_dir = repo.path();
        let workdir = git_dir.parent().unwrap_or(git_dir);
        let workdir_str = workdir.to_str().unwrap_or(".");
        debug!(
            "Attempting fallback using system git pull --rebase in {}",
            workdir_str
        );

        use std::process::Command;
        match Command::new("git")
            .args(["-C", workdir_str, "pull", "--rebase", "origin", "main"])
            .status()
        {
            Ok(s) if s.success() => {
                debug!("Fallback git pull --rebase succeeded");
                info!("Rebase completado exitosamente (fallback)");
            }
            Ok(s) => {
                return Err(anyhow!("Fallback git pull failed with exit code: {}", s));
            }
            Err(err) => {
                return Err(anyhow!("Failed to execute fallback git: {}", err));
            }
        }
    } else {
        info!("Rebase completado exitosamente");
    }

    // Obtener referencias
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    // Rebase
    debug!("Ejecutando rebase");
    let mut rebase = repo.rebase(None, Some(&fetch_commit), None, None)?;

    let mut ops = 0;
    while let Some(_op) = rebase.next() {
        ops += 1;
        rebase.commit(None, &Signature::now("crypta", "crypta@local")?, None)?;
    }

    debug!("Aplicadas {} operaciones de rebase", ops);
    rebase.finish(None)?;
    info!("Rebase completado exitosamente");

    Ok(())
}

fn push(repo: &Repository) -> Result<()> {
    let mut remote = repo.find_remote("origin")?;
    let mut callbacks = RemoteCallbacks::new();

    // Configurar autenticación SSH
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        debug!("Solicitando credenciales SSH para push");
        let username = username_from_url.unwrap_or("git");
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

        // Primero intentar con claves SSH del directorio .ssh
        for key_name in ["id_ed25519", "id_rsa", "id_ecdsa"] {
            let key_path_str = format!("{}/.ssh/{}", home, key_name);
            let key_path = std::path::Path::new(&key_path_str);
            if key_path.exists() {
                debug!("Intentando push con clave SSH: {}", key_path.display());
                if let Ok(cred) = git2::Cred::ssh_key(username, None, key_path, None) {
                    return Ok(cred);
                }
            }
        }

        // Si no hay claves en disco, intentar con ssh-agent
        debug!("Intentando push con ssh-agent");
        git2::Cred::ssh_key_from_agent(username)
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    let result = remote.push(
        &["refs/heads/main:refs/heads/main"],
        Some(&mut push_options),
    );

    if let Err(e) = result {
        debug!("Libgit2 push failed: {}", e);

        // Fallback: try system `git` in the working directory. This often succeeds
        // when libgit2/ssh-agent integration fails but the system ssh/git setup works.
        let git_dir = repo.path();
        let workdir = git_dir.parent().unwrap_or(git_dir);
        let workdir_str = workdir.to_str().unwrap_or(".");
        debug!("Attempting fallback using system git in {}", workdir_str);

        use std::process::Command;
        match Command::new("git")
            .args(["-C", workdir_str, "push", "origin", "main"])
            .status()
        {
            Ok(s) if s.success() => {
                debug!("Fallback git push succeeded");
                return Ok(());
            }
            Ok(s) => {
                return Err(anyhow!("Fallback git push failed with exit code: {}", s));
            }
            Err(err) => {
                return Err(anyhow!("Failed to execute fallback git: {}", err));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use git2::{Repository, Signature};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_repository_operations() {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();

        assert!(!repo.is_bare());
        assert!(repo.is_empty().unwrap());
    }

    #[test]
    fn test_signature_creation() {
        let sig = Signature::now("crypta", "crypta@local").unwrap();
        assert_eq!(sig.name().unwrap(), "crypta");
        assert_eq!(sig.email().unwrap(), "crypta@local");
    }

    #[test]
    fn test_git_status() {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();

        let statuses = repo.statuses(None).unwrap();
        assert!(statuses.is_empty());

        // Crear un archivo
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();

        let statuses = repo.statuses(None).unwrap();
        assert!(!statuses.is_empty());
    }

    #[test]
    fn test_git_index() {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();

        let index = repo.index().unwrap();
        assert_eq!(index.len(), 0);
    }
}
