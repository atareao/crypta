use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell};
use crypta::{git, secrets};
use std::io::{self, Read};
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
    },
    /// Muestra un valor por stdout
    #[command(alias = "l")]
    Lookup {
        /// Clave del secreto (o usa variable de entorno SECRET_ID)
        key: Option<String>,
        /// Registrar el acceso en el log de auditoría
        #[arg(long, default_value_t = false)]
        log_access: bool,
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
    Init,
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
}

fn main() {
    // Configurar tracing - usa RUST_LOG=debug para ver más detalles
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error")),
        )
        .with_target(false)
        .init();

    let home = std::env::var("HOME").expect("No se pudo encontrar $HOME");
    let secrets_dir = format!("{}/.secrets", home);
    let secrets_file = format!("{}/secrets.yml", secrets_dir);
    let cli = Cli::parse();

    info!("Crypta iniciado");

    if let Err(e) = run_command(&cli.command, &secrets_dir, &secrets_file) {
        error!("Error ejecutando comando: {}", e);
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }

    info!("Comando ejecutado exitosamente");
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
        Commands::Get { key, log_access } => {
            let key = resolve_key(key.as_deref())?;
            secrets::get(secrets_file, &key, *log_access)
        }
        Commands::Lookup { key, log_access } => {
            let key = resolve_key(key.as_deref())?;
            secrets::show(secrets_file, &key, *log_access)
        }
        Commands::List => secrets::list(secrets_file),
        Commands::Delete { key } => {
            let key = resolve_key(key.as_deref())?;
            secrets::remove(secrets_file, &key)
        }
        Commands::Init => secrets::init(secrets_dir, secrets_file),
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
        Commands::Completion { shell } => {
            let shell: Shell = shell.parse().map_err(|_| {
                anyhow::anyhow!(
                    "Shell no soportado: '{}'. Usa: bash, zsh, fish, powershell, elvish",
                    shell
                )
            })?;
            let cmd = Cli::command();
            shell.generate(&cmd, &mut io::stdout());
            // clap_complete escribe directamente a stdout
            Ok(())
        }
    }
}
