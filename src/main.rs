use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use crypta::{git, secrets, templates};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::Path;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "crypta")]
#[command(about = "Gestor de secretos con SOPS y Git", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Almacena o actualiza un secreto (valor desde stdin)
    #[command(alias = "s")]
    Store {
        /// Clave del secreto (o usa variable de entorno SECRET_ID)
        key: Option<String>,
    },
    /// Almacena o actualiza un secreto
    #[command(alias = "se")]
    Set {
        /// Clave del secreto (o usa variable de entorno SECRET_ID)
        #[arg(short, long)]
        key: Option<String>,
        /// Valor del secreto
        #[arg(short, long)]
        value: String,
    },
    /// Obtiene un valor y lo copia al portapapeles
    #[command(alias = "g")]
    Get {
        /// Clave del secreto (o usa variable de entorno SECRET_ID)
        key: Option<String>,
        /// Registrar el acceso en el log de auditoría
        #[arg(long, default_value_t = false)]
        log_access: bool,
        /// Muestra solo la longitud del secreto, sin copiarlo ni revelarlo
        #[arg(long, default_value_t = false)]
        mask: bool,
    },
    /// Muestra un valor por stdout
    #[command(alias = "l")]
    Lookup {
        /// Clave del secreto (o usa variable de entorno SECRET_ID)
        key: Option<String>,
        /// Registrar el acceso en el log de auditoría
        #[arg(long, default_value_t = false)]
        log_access: bool,
        /// Muestra solo la longitud del secreto, sin revelar su valor
        #[arg(long, default_value_t = false)]
        mask: bool,
    },
    /// Lista todas las claves
    #[command(alias = "ls")]
    List,
    /// Elimina una clave
    #[command(alias = "rm")]
    Delete {
        /// Clave del secreto (o usa variable de entorno SECRET_ID)
        key: Option<String>,
    },
    /// Inicializa el directorio y archivo de secretos
    #[command(alias = "i")]
    Init {
        /// URL del repositorio git remoto (opcional)
        #[arg(short = 'r', long)]
        remote: Option<String>,
    },
    /// Sincroniza cambios con el remoto
    #[command(alias = "sy")]
    Sync { message: Option<String> },
    /// Genera una contraseña aleatoria
    #[command(alias = "pwd")]
    Password {
        /// Longitud de la contraseña
        #[arg(short = 'l', long, default_value_t = 32)]
        length: usize,
        /// Incluir caracteres especiales
        #[arg(long, default_value_t = false)]
        special: bool,
    },
    /// Importa secretos desde un archivo .env, JSON o YAML
    #[command(alias = "im")]
    Import {
        /// Formato de entrada: env, json o yaml
        #[arg(short, long, default_value_t = String::from("env"))]
        format: String,
        /// Prefijo opcional para todas las claves importadas
        #[arg(short, long, default_value_t = String::new())]
        prefix: String,
        /// Simulación: muestra lo que se importaría sin escribir
        #[arg(long, default_value_t = false)]
        dry_run: bool,
        /// Archivo de entrada (opcional, por defecto stdin)
        file: Option<String>,
    },
    /// Exporta todos los secretos en formato .env, JSON o YAML
    #[command(alias = "ex")]
    Export {
        /// Formato de salida: env, json o yaml
        #[arg(short, long, default_value_t = String::from("env"))]
        format: String,
        /// Archivo de salida (opcional, por defecto stdout)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Genera script de autocompletado para el shell
    #[command(alias = "com")]
    Completion {
        /// Shell: bash, zsh, fish, powershell, elvish
        #[arg(short, long, default_value_t = String::from("bash"))]
        shell: String,
    },
    /// Renderiza una plantilla Jinja2 con valores del store de secretos
    #[command(alias = "r")]
    Render {
        /// Archivo de plantilla (opcional, por defecto stdin)
        file: Option<String>,
        /// Escribe el resultado en el fichero indicado en lugar de stdout
        #[arg(long)]
        out: Option<String>,
    },
}

fn main() {
    // Configurar tracing - usa RUST_LOG=debug para ver más detalles.
    // Los logs van siempre a stderr para no contaminar el stdout de los
    // comandos que devuelven datos (render, lookup, export...).
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error")),
        )
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        error!("Error ejecutando comando: {}", e);
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }

    info!("Comando ejecutado exitosamente");
}

/// Ejecuta la CLI con las rutas de secretos derivadas de `$HOME`.
fn run(cli: Cli) -> Result<()> {
    let home =
        std::env::var("HOME").context("No se pudo encontrar la variable de entorno $HOME")?;
    let secrets_dir = format!("{}/.secrets", home);
    let secrets_file = format!("{}/secrets.yml", secrets_dir);

    info!("Crypta iniciado");

    run_command(&cli.command, &secrets_dir, &secrets_file)
}

/// Resuelve la clave del secreto desde parámetro o variable de entorno SECRET_ID
fn resolve_key(key_param: Option<&str>) -> Result<String> {
    match key_param {
        Some(key) => Ok(key.to_string()),
        None => {
            std::env::var("SECRET_ID")
                .map_err(|_| anyhow::anyhow!(
                    "No se proporcionó clave. Usa el parámetro KEY o define la variable de entorno SECRET_ID"
                ))
        }
    }
}

fn run_command(command: &Commands, secrets_dir: &str, secrets_file: &str) -> Result<()> {
    match command {
        Commands::Store { key } => {
            let key = resolve_key(key.as_deref())?;
            // Leer valor desde stdin
            use std::io::{self, Read};
            let mut value = String::new();
            io::stdin().read_to_string(&mut value)?;
            let value = value.trim(); // Remover whitespace al final
            secrets::add(secrets_dir, secrets_file, &key, value)
        }
        Commands::Set { key, value } => {
            let key = resolve_key(key.as_deref())?;
            secrets::add(secrets_dir, secrets_file, &key, value)
        }
        Commands::Get {
            key,
            log_access,
            mask,
        } => {
            let key = resolve_key(key.as_deref())?;
            if *mask {
                secrets::mask(secrets_file, &key, *log_access, "get")
            } else {
                secrets::get(secrets_file, &key, *log_access)
            }
        }
        Commands::Lookup {
            key,
            log_access,
            mask,
        } => {
            let key = resolve_key(key.as_deref())?;
            if *mask {
                secrets::mask(secrets_file, &key, *log_access, "lookup")
            } else {
                secrets::show(secrets_file, &key, *log_access)
            }
        }
        Commands::List => secrets::list(secrets_file),
        Commands::Delete { key } => {
            let key = resolve_key(key.as_deref())?;
            secrets::remove(secrets_file, &key)
        }
        Commands::Init { remote } => secrets::init(secrets_dir, secrets_file, remote.as_deref()),
        Commands::Sync { message } => git::sync(secrets_dir, message.as_deref()),
        Commands::Password { length, special } => secrets::generate_password(*length, *special),
        Commands::Import {
            format,
            prefix,
            dry_run,
            file,
        } => {
            let input = match file {
                Some(path) => std::fs::read_to_string(path)
                    .context(format!("No se pudo leer el archivo: {}", path))?,
                None => {
                    let mut buf = String::new();
                    io::stdin().read_to_string(&mut buf)?;
                    buf
                }
            };
            secrets::import_secrets(secrets_dir, secrets_file, format, &input, prefix, *dry_run)
        }
        Commands::Export { format, output } => {
            let result = secrets::export_secrets(secrets_file, format)?;
            match output {
                Some(path) => std::fs::write(path, &result)
                    .context(format!("No se pudo escribir el archivo: {}", path))?,
                None => print!("{}", result),
            }
            Ok(())
        }
        Commands::Render { file, out } => {
            let result = templates::render_file(secrets_file, file.as_deref())?;
            match out {
                Some(path) => write_private_output(path, secrets_file, &result)?,
                None => print!("{}", result),
            }
            Ok(())
        }
        Commands::Completion { shell } => {
            let shell: Shell = shell.parse().map_err(|_| {
                anyhow::anyhow!(
                    "Shell no soportado: '{}'. Usa: bash, zsh, fish, powershell, elvish",
                    shell
                )
            })?;
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "crypta", &mut io::stdout());
            Ok(())
        }
    }
}

/// Escribe `content` en `path` como fichero privado (0600).
///
/// Rechaza enlaces simbólicos, ficheros no regulares y cualquier destino que
/// sea el propio archivo de secretos, para no sobrescribir el store cifrado.
fn write_private_output(path: &str, secrets_file: &str, content: &str) -> Result<()> {
    let path = Path::new(path);
    let secrets_path = Path::new(secrets_file);

    match std::fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                anyhow::bail!(
                    "No se permite escribir sobre un enlace simbólico: {}",
                    path.display()
                );
            }
            if !metadata.is_file() {
                anyhow::bail!(
                    "No se puede escribir en el destino: no es un fichero regular: {}",
                    path.display()
                );
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error).with_context(|| {
                format!("No se pudo inspeccionar el destino: {}", path.display())
            });
        }
    }

    // Se abre SIN truncar: primero hay que comprobar el inodo del descriptor
    // abierto. Así una sustitución por hardlink a `secrets.yml` entre la
    // comprobación previa y el `open` no destruye el store.
    let mut options = OpenOptions::new();
    options.write(true).create(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
        #[cfg(target_os = "linux")]
        options.custom_flags(O_NOFOLLOW);
    }

    let mut file = options
        .open(path)
        .with_context(|| format!("No se pudo escribir el archivo: {}", path.display()))?;

    let opened_metadata = file
        .metadata()
        .with_context(|| format!("No se pudo inspeccionar el archivo: {}", path.display()))?;

    if !opened_metadata.is_file() {
        anyhow::bail!(
            "No se puede escribir en el destino: no es un fichero regular: {}",
            path.display()
        );
    }

    #[cfg(unix)]
    {
        let secrets_metadata = std::fs::metadata(secrets_path).with_context(|| {
            format!(
                "No se pudo inspeccionar el archivo de secretos: {}",
                secrets_path.display()
            )
        })?;
        if same_file_metadata(&opened_metadata, &secrets_metadata) {
            anyhow::bail!(
                "No se puede sobrescribir el archivo de secretos: {}",
                path.display()
            );
        }

        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))
            .with_context(|| {
                format!(
                    "No se pudo ajustar los permisos del archivo: {}",
                    path.display()
                )
            })?;
    }

    #[cfg(not(unix))]
    {
        if same_file(path, secrets_path) {
            anyhow::bail!(
                "No se puede sobrescribir el archivo de secretos: {}",
                path.display()
            );
        }
    }

    file.set_len(0)
        .with_context(|| format!("No se pudo truncar el archivo: {}", path.display()))?;
    file.write_all(content.as_bytes())
        .with_context(|| format!("No se pudo escribir el archivo: {}", path.display()))?;
    Ok(())
}

/// `O_NOFOLLOW` en Linux para las arquitecturas cuyo valor es `0o100000`
/// (`0x8000`): arm, aarch64, powerpc, powerpc64 y m68k.
///
/// No se añade `libc` como dependencia directa, así que el valor se fija por
/// arquitectura siguiendo las definiciones de `libc` 0.2 para linux-gnu. Un
/// valor equivocado no falla: simplemente deja de proteger contra symlinks.
#[cfg(all(
    target_os = "linux",
    any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "powerpc",
        target_arch = "powerpc64",
        target_arch = "m68k"
    )
))]
const O_NOFOLLOW: i32 = 0o100000;

/// `O_NOFOLLOW` en Linux para loongarch64 (`0o00400000`, `0x100000`).
#[cfg(all(target_os = "linux", target_arch = "loongarch64"))]
const O_NOFOLLOW: i32 = 0o00400000;

/// `O_NOFOLLOW` en Linux para el resto de arquitecturas soportadas
/// (`0o400000`, `0x20000`): x86, x86_64, mips, mips64, riscv32, riscv64,
/// s390x, csky, sparc y sparc64.
#[cfg(all(
    target_os = "linux",
    not(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "powerpc",
        target_arch = "powerpc64",
        target_arch = "m68k",
        target_arch = "loongarch64"
    ))
))]
const O_NOFOLLOW: i32 = 0o400000;

/// Indica si dos metadatos corresponden al mismo fichero (dispositivo e inodo).
#[cfg(unix)]
fn same_file_metadata(a: &std::fs::Metadata, b: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    a.dev() == b.dev() && a.ino() == b.ino()
}

/// Indica si dos rutas apuntan al mismo fichero.
#[cfg(not(unix))]
fn same_file(a: &Path, b: &Path) -> bool {
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}
