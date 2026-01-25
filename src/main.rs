use clap::{Parser, Subcommand};
use anyhow::Result;
use crypta::{secrets, git};

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
    /// Lista todas las claves
    Ls,
    /// Elimina una clave
    Rm { key: String },
    /// Sincroniza cambios con el remoto
    Sync { message: Option<String> },
}

fn main() {
    let home = std::env::var("HOME").expect("No se pudo encontrar $HOME");
    let secrets_dir = format!("{}/.secrets", home);
    let secrets_file = format!("{}/secrets.yml", secrets_dir);
    let cli = Cli::parse();

    if let Err(e) = run_command(&cli.command, &secrets_dir, &secrets_file) {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

fn run_command(command: &Commands, secrets_dir: &str, secrets_file: &str) -> Result<()> {
    match command {
        Commands::Add { key, value } => secrets::add(secrets_dir, secrets_file, key, value),
        Commands::Get { key } => secrets::get(secrets_file, key),
        Commands::Ls => secrets::list(secrets_file),
        Commands::Rm { key } => secrets::remove(secrets_file, key),
        Commands::Sync { message } => git::sync(secrets_dir, message.as_deref()),
    }
}
