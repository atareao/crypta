use clap::{Parser, Subcommand};
use anyhow::Result;
use crypta::{secrets, git};
use tracing::{info, error};
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
    Store { key: String },
    /// Almacena o actualiza un secreto (alias de store)
    Set { key: String, value: String },
    /// Obtiene un valor y lo copia al portapapeles
    Get { key: String },
    /// Muestra un valor por stdout
    Lookup { key: String },
    /// Lista todas las claves
    List,
    /// Elimina una clave
    Delete { key: String },
    /// Sincroniza cambios con el remoto
    Sync { message: Option<String> },
}

fn main() {
    // Configurar tracing - usa RUST_LOG=debug para ver más detalles
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("error"))
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

fn run_command(command: &Commands, secrets_dir: &str, secrets_file: &str) -> Result<()> {
    match command {
        Commands::Store { key } => {
            // Leer valor desde stdin
            use std::io::{self, Read};
            let mut value = String::new();
            io::stdin().read_to_string(&mut value)?;
            let value = value.trim(); // Remover whitespace al final
            secrets::add(secrets_dir, secrets_file, key, value)
        },
        Commands::Set { key, value } => secrets::add(secrets_dir, secrets_file, key, value),
        Commands::Get { key } => secrets::get(secrets_file, key),
        Commands::Lookup { key } => secrets::show(secrets_file, key),
        Commands::List => secrets::list(secrets_file),
        Commands::Delete { key } => secrets::remove(secrets_file, key),
        Commands::Sync { message } => git::sync(secrets_dir, message.as_deref()),
    }
}
