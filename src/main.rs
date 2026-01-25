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
    /// Añade o actualiza un secreto
    Add { key: String, value: String },
    /// Obtiene un valor y lo copia al portapapeles
    Get { key: String },
    /// Muestra un valor por stdout
    Show { key: String },
    /// Lista todas las claves
    Ls,
    /// Elimina una clave
    Rm { key: String },
    /// Sincroniza cambios con el remoto
    Sync { message: Option<String> },
}

fn main() {
    // Configurar tracing - usa RUST_LOG=debug para ver más detalles
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("crypta=info"))
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
        Commands::Add { key, value } => secrets::add(secrets_dir, secrets_file, key, value),
        Commands::Get { key } => secrets::get(secrets_file, key),
        Commands::Show { key } => secrets::show(secrets_file, key),
        Commands::Ls => secrets::list(secrets_file),
        Commands::Rm { key } => secrets::remove(secrets_file, key),
        Commands::Sync { message } => git::sync(secrets_dir, message.as_deref()),
    }
}
