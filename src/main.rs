use anyhow::Result;
use clap::{Parser, Subcommand};
use crypta::{git, secrets};
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
    },
    /// Muestra un valor por stdout
    #[command(alias = "l")]
    Lookup {
        /// Clave del secreto (o usa variable de entorno SECRET_ID)
        key: Option<String>,
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
fn resolve_key(key_param: Option<String>) -> Result<String> {
    match key_param {
        Some(key) => Ok(key),
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
            let key = resolve_key(key.clone())?;
            // Leer valor desde stdin
            use std::io::{self, Read};
            let mut value = String::new();
            io::stdin().read_to_string(&mut value)?;
            let value = value.trim(); // Remover whitespace al final
            secrets::add(secrets_dir, secrets_file, &key, value)
        }
        Commands::Set { key, value } => {
            let key = resolve_key(key.clone())?;
            secrets::add(secrets_dir, secrets_file, &key, value)
        }
        Commands::Get { key } => {
            let key = resolve_key(key.clone())?;
            secrets::get(secrets_file, &key)
        }
        Commands::Lookup { key } => {
            let key = resolve_key(key.clone())?;
            secrets::show(secrets_file, &key)
        }
        Commands::List => secrets::list(secrets_file),
        Commands::Delete { key } => {
            let key = resolve_key(key.clone())?;
            secrets::remove(secrets_file, &key)
        }
        Commands::Init => secrets::init(secrets_dir, secrets_file),
        Commands::Sync { message } => git::sync(secrets_dir, message.as_deref()),
    }
}
