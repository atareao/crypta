//! Utilidades compartidas por los tests de integración de la CLI.
//!
//! Aisla cada test en un `HOME` temporal con un `sops` falso (un script que
//! hace `cat` del último argumento). Así se ejercita el flujo real de la CLI
//! (desencriptado vía `sops -d`) sin depender de `sops` real, de Age ni de red.

#![allow(dead_code)]

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Entorno aislado para invocar el binario `crypta`.
pub struct TestEnv {
    // Mantiene vivo el directorio temporal mientras exista el entorno.
    _temp: TempDir,
    /// `HOME` del proceso: la CLI busca `$HOME/.secrets/secrets.yml`.
    pub home: PathBuf,
    /// Directorio que contiene el `sops` falso.
    pub bin_dir: PathBuf,
    /// Ruta de `$HOME/.secrets`.
    pub secrets_dir: PathBuf,
    /// Ruta del archivo de secretos (`$HOME/.secrets/secrets.yml`).
    pub secrets_file: PathBuf,
}

impl TestEnv {
    /// Crea un entorno con el YAML indicado como contenido "desencriptado".
    ///
    /// El `sops` falso ignora las banderas y vuelca el último argumento, de
    /// modo que el archivo de secretos se lee tal cual.
    pub fn new(secrets_yaml: &str) -> Self {
        let temp = TempDir::new().expect("no se pudo crear el directorio temporal");
        let home = temp.path().join("home");
        let bin_dir = temp.path().join("bin");
        let secrets_dir = home.join(".secrets");
        let secrets_file = secrets_dir.join("secrets.yml");

        std::fs::create_dir_all(&secrets_dir).expect("no se pudo crear $HOME/.secrets");
        std::fs::create_dir_all(&bin_dir).expect("no se pudo crear el directorio del sops falso");
        std::fs::write(&secrets_file, secrets_yaml).expect("no se pudo escribir secrets.yml");

        let sops = bin_dir.join("sops");
        std::fs::write(&sops, "#!/bin/sh\nfor last; do :; done\ncat \"$last\"\n")
            .expect("no se pudo escribir el sops falso");
        make_executable(&sops);

        Self {
            _temp: temp,
            home,
            bin_dir,
            secrets_dir,
            secrets_file,
        }
    }

    /// Construye un `Command` del binario con `HOME` y `PATH` aislados.
    ///
    /// Se elimina `SECRET_ID` para que no interfiera con las claves que se
    /// pasan explícitamente por argumento.
    pub fn command(&self) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_crypta"));
        cmd.env("HOME", &self.home);
        cmd.env("PATH", self.path_with_fake_sops());
        cmd.env_remove("SECRET_ID");
        // Evita que un RUST_LOG heredado contamine stdout/stderr y haga
        // que los tests dependan del entorno de quien los ejecuta.
        cmd.env_remove("RUST_LOG");
        cmd
    }

    /// Escribe una plantilla en el `HOME` del test y devuelve su ruta.
    pub fn write_template(&self, name: &str, content: &str) -> PathBuf {
        let path = self.home.join(name);
        std::fs::write(&path, content).expect("no se pudo escribir la plantilla");
        path
    }

    fn path_with_fake_sops(&self) -> OsString {
        let mut paths = vec![self.bin_dir.clone()];
        if let Some(path) = std::env::var_os("PATH") {
            paths.extend(std::env::split_paths(&path));
        }
        std::env::join_paths(paths).expect("PATH contiene componentes no válidos")
    }
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)
        .expect("no se pudo leer el modo del sops falso")
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).expect("no se pudo hacer ejecutable el sops falso");
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}
