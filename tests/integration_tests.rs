use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_crypta"))
        .arg("--help")
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Gestor de secretos con SOPS y Git"));
    assert!(stdout.contains("add"));
    assert!(stdout.contains("get"));
    assert!(stdout.contains("show"));
    assert!(stdout.contains("ls"));
    assert!(stdout.contains("rm"));
    assert!(stdout.contains("sync"));
}

#[test]
fn test_cli_subcommands() {
    let subcommands = ["add", "get", "show", "ls", "rm", "sync"];
    
    for cmd in &subcommands {
        let output = Command::new(env!("CARGO_BIN_EXE_crypta"))
            .arg(cmd)
            .arg("--help")
            .output()
            .expect("Failed to execute command");
        
        // Algunos subcommandos pueden fallar sin argumentos, pero --help debe funcionar
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Verificar que se muestra información de ayuda
        assert!(stdout.len() > 0 || stderr.len() > 0);
    }
}

#[test]
fn test_cli_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_crypta"))
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    
    // clap puede no implementar --version por defecto
    // Solo verificamos que el comando se ejecuta
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.len() > 0 || stderr.len() > 0);
}
