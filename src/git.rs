use anyhow::{Result, Context};
use git2::{Repository, Signature, IndexAddOption, PushOptions, RemoteCallbacks};
use tracing::{debug, info};

pub fn sync(secrets_dir: &str, message: Option<&str>) -> Result<()> {
    info!("Iniciando sincronización Git");
    debug!("Directorio: {}", secrets_dir);
    println!("🔄 Sincronizando con el remoto...");
    
    let repo = Repository::open(secrets_dir)
        .context("No se pudo abrir el repositorio git")?;
    
    // Pull con rebase
    info!("Ejecutando pull con rebase");
    pull_rebase(&repo)?;
    
    // Verificar si hay cambios
    debug!("Verificando cambios locales");
    let statuses = repo.statuses(None)?;
    
    if !statuses.is_empty() {
        let msg = message.unwrap_or("Sync secrets");
        info!("Detectados {} cambios, creando commit", statuses.len());
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
        
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            msg,
            &tree,
            &[&parent],
        )?;
        
        // Push
        info!("Realizando push al remoto");
        push(&repo)?;
        
        println!("🚀 Sincronización completada.");
        info!("Sincronización completada exitosamente");
    } else {
        info!("No hay cambios para sincronizar");
        println!("✅ Todo al día.");
    }
    
    Ok(())
}

fn pull_rebase(repo: &Repository) -> Result<()> {
    debug!("Iniciando pull con rebase");
    
    // Fetch desde origin
    let mut remote = repo.find_remote("origin")?;
    debug!("Fetching desde origin");
    remote.fetch(&["main"], None, None)?;
    
    // Obtener referencias
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    
    // Rebase
    debug!("Ejecutando rebase");
    let mut rebase = repo.rebase(
        None,
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
    info!("Rebase completado exitosamente");
    
    Ok(())
}

fn push(repo: &Repository) -> Result<()> {
    let mut remote = repo.find_remote("origin")?;
    let callbacks = RemoteCallbacks::new();
    
    // Configurar callbacks vacíos (usa las credenciales del sistema)
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);
    
    remote.push(&["refs/heads/main:refs/heads/main"], Some(&mut push_options))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use git2::{Repository, Signature};
    use tempfile::TempDir;
    use std::fs;

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
