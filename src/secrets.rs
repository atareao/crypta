use anyhow::{Context, Result};
use arboard::Clipboard;
use serde_yaml::Value;
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing::{debug, info};

pub fn add(secrets_dir: &str, secrets_file: &str, key: &str, value: &str) -> Result<()> {
    info!("Añadiendo secreto '{}'", key);
    debug!("Directorio: {}, Archivo: {}", secrets_dir, secrets_file);

    verify_sops_installed()?;

    // Crear directorio si no existe
    fs::create_dir_all(secrets_dir).context("No se pudo crear el directorio de secretos")?;

    let decrypted_content = if !Path::new(secrets_file).exists() {
        // Si no existe, crear estructura YAML vacía
        info!("Creando nuevo archivo de secretos");
        String::new()
    } else {
        // Desencriptar archivo existente
        info!("Actualizando secreto existente '{}'", key);
        debug!("Desencriptando con sops...");

        let output = Command::new("sops")
            .arg("-d")
            .arg(secrets_file)
            .output()
            .context("No se pudo ejecutar sops")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Error al desencriptar: {}", error);
        }

        String::from_utf8(output.stdout).context("El contenido desencriptado no es UTF-8 válido")?
    };

    // Parsear YAML y actualizar
    let mut data: Value = if decrypted_content.is_empty() {
        Value::Mapping(serde_yaml::Mapping::new())
    } else {
        serde_yaml::from_str(&decrypted_content).context("No se pudo parsear el contenido YAML")?
    };

    if let Value::Mapping(ref mut map) = data {
        map.insert(
            Value::String(key.to_string()),
            Value::String(value.to_string()),
        );
    }

    let updated_yaml = serde_yaml::to_string(&data).context("No se pudo serializar el YAML")?;
    debug!("YAML actualizado");

    // Encriptar con sops
    let encrypted_content = encrypt_with_sops(&updated_yaml, secrets_file)?;

    fs::write(secrets_file, encrypted_content)
        .context("No se pudo escribir el archivo de secretos")?;
    debug!("Archivo encriptado y guardado");

    println!("✅ Secreto '{}' añadido.", key);
    Ok(())
}

pub fn get(secrets_file: &str, key: &str) -> Result<()> {
    info!("Obteniendo secreto '{}'", key);
    debug!("Archivo: {}", secrets_file);

    if !Path::new(secrets_file).exists() {
        anyhow::bail!(
            "El archivo de secretos no existe: {}\n\nPrimero añade un secreto con: crypta add CLAVE valor",
            secrets_file
        );
    }

    verify_sops_installed()?;

    debug!("Desencriptando con sops...");
    let output = Command::new("sops")
        .arg("-d")
        .arg(secrets_file)
        .output()
        .context("No se pudo ejecutar sops")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Error al desencriptar: {}", error);
    }

    let decrypted_content = String::from_utf8(output.stdout)
        .context("El contenido desencriptado no es UTF-8 válido")?;

    let yaml: Value =
        serde_yaml::from_str(&decrypted_content).context("No se pudo parsear el contenido YAML")?;

    let val = yaml
        .get(key)
        .and_then(|v| v.as_str())
        .context(format!("La clave '{}' no existe", key))?;

    // Copiar al portapapeles
    debug!("Copiando al portapapeles");
    let mut clipboard = Clipboard::new().context("No se pudo acceder al portapapeles")?;
    clipboard
        .set_text(val)
        .context("No se pudo copiar al portapapeles")?;
    info!("Secreto copiado al portapapeles exitosamente");

    println!("📋 Secreto '{}' copiado al portapapeles.", key);
    Ok(())
}

pub fn show(secrets_file: &str, key: &str) -> Result<()> {
    info!("Mostrando secreto '{}'", key);
    debug!("Archivo: {}", secrets_file);

    if !Path::new(secrets_file).exists() {
        anyhow::bail!(
            "El archivo de secretos no existe: {}\n\nPrimero añade un secreto con: crypta add CLAVE valor",
            secrets_file
        );
    }

    verify_sops_installed()?;

    debug!("Desencriptando con sops...");
    let output = Command::new("sops")
        .arg("-d")
        .arg(secrets_file)
        .output()
        .context("No se pudo ejecutar sops")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Error al desencriptar: {}", error);
    }

    let decrypted_content = String::from_utf8(output.stdout)
        .context("El contenido desencriptado no es UTF-8 válido")?;

    let yaml: Value =
        serde_yaml::from_str(&decrypted_content).context("No se pudo parsear el contenido YAML")?;

    let val = yaml
        .get(key)
        .and_then(|v| v.as_str())
        .context(format!("La clave '{}' no existe", key))?;

    // Imprimir el valor por stdout
    println!("{}", val);
    Ok(())
}

pub fn list(secrets_file: &str) -> Result<()> {
    info!("Listando secretos");
    debug!("Archivo: {}", secrets_file);

    if !Path::new(secrets_file).exists() {
        anyhow::bail!(
            "El archivo de secretos no existe: {}\n\nPrimero añade un secreto con: crypta add CLAVE valor",
            secrets_file
        );
    }

    verify_sops_installed()?;

    debug!("Desencriptando con sops...");
    let output = Command::new("sops")
        .arg("-d")
        .arg(secrets_file)
        .output()
        .context("No se pudo ejecutar sops")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Error al desencriptar: {}", error);
    }

    let decrypted_content = String::from_utf8(output.stdout)
        .context("El contenido desencriptado no es UTF-8 válido")?;

    let yaml: Value =
        serde_yaml::from_str(&decrypted_content).context("No se pudo parsear el contenido YAML")?;

    println!("🔑 Claves en {}:", secrets_file);

    if let Value::Mapping(map) = yaml {
        for key in map.keys() {
            if let Some(key_str) = key.as_str() {
                println!("{}", key_str);
            }
        }
    }

    Ok(())
}

pub fn remove(secrets_file: &str, key: &str) -> Result<()> {
    info!("Eliminando secreto '{}'", key);
    debug!("Archivo: {}", secrets_file);

    if !Path::new(secrets_file).exists() {
        anyhow::bail!("El archivo de secretos no existe: {}", secrets_file);
    }

    verify_sops_installed()?;

    debug!("Desencriptando con sops...");
    let output = Command::new("sops")
        .arg("-d")
        .arg(secrets_file)
        .output()
        .context("No se pudo ejecutar sops")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Error al desencriptar: {}", error);
    }

    let decrypted_content = String::from_utf8(output.stdout)
        .context("El contenido desencriptado no es UTF-8 válido")?;

    let mut yaml: Value =
        serde_yaml::from_str(&decrypted_content).context("No se pudo parsear el contenido YAML")?;

    if let Value::Mapping(ref mut map) = yaml {
        map.remove(&Value::String(key.to_string()));
    }

    let updated_yaml = serde_yaml::to_string(&yaml).context("No se pudo serializar el YAML")?;
    debug!("YAML actualizado");

    // Encriptar con sops
    let encrypted_content = encrypt_with_sops(&updated_yaml, secrets_file)?;

    fs::write(secrets_file, encrypted_content)
        .context("No se pudo escribir el archivo de secretos")?;
    debug!("Archivo reencriptado y guardado");

    println!("🗑️ Secreto '{}' eliminado.", key);
    Ok(())
}

fn verify_sops_installed() -> Result<()> {
    debug!("Verificando que sops esté instalado...");

    let output = Command::new("which").arg("sops").output();

    match output {
        Ok(out) if out.status.success() => {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            debug!("sops encontrado en: {}", path);
            Ok(())
        }
        _ => {
            anyhow::bail!(
                "El comando 'sops' no está instalado.\n\n\
                Instala SOPS:\n\
                - Arch Linux: sudo pacman -S sops\n\
                - Ubuntu/Debian: sudo apt install sops\n\
                - macOS: brew install sops\n\
                - O descarga desde: https://github.com/getsops/sops/releases"
            )
        }
    }
}

fn encrypt_with_sops(yaml_content: &str, secrets_file: &str) -> Result<Vec<u8>> {
    debug!("Encriptando con sops...");

    // Obtener el directorio del archivo de secretos para .sops.yaml
    let secrets_path = Path::new(secrets_file);
    let work_dir = secrets_path
        .parent()
        .context("No se pudo obtener el directorio del archivo de secretos")?;

    // Escribir contenido a un archivo temporal .yml en el mismo directorio
    // para que SOPS pueda aplicar las reglas de creación basadas en path
    use std::io::Write;
    let temp_file_path = work_dir.join(".crypta_temp.yml");
    let mut temp_file =
        fs::File::create(&temp_file_path).context("No se pudo crear archivo temporal")?;
    temp_file
        .write_all(yaml_content.as_bytes())
        .context("No se pudo escribir al archivo temporal")?;
    drop(temp_file); // Cerrar el archivo

    // Encriptar el archivo temporal
    let output = Command::new("sops")
        .arg("-e")
        .arg(&temp_file_path)
        .current_dir(work_dir)
        .output()
        .context("No se pudo ejecutar sops")?;

    // Limpiar archivo temporal
    let _ = fs::remove_file(&temp_file_path);

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Error al encriptar: {}", error);
    }

    Ok(output.stdout)
}

/// Extrae la clave pública de un archivo de clave privada Age
fn extract_public_key_from_file(key_file_path: &str) -> Result<String> {
    debug!("Extrayendo clave pública del archivo: {}", key_file_path);

    // Leer el archivo de clave privada
    let key_content =
        fs::read_to_string(key_file_path).context("No se pudo leer el archivo de clave privada")?;

    // Buscar el comentario que contiene la clave pública
    for line in key_content.lines() {
        if line.starts_with("# public key: ") {
            let public_key = line.replace("# public key: ", "").trim().to_string();
            debug!("Clave pública extraída: {}", public_key);
            return Ok(public_key);
        }
    }

    anyhow::bail!("No se pudo encontrar la clave pública en el archivo")
}

/// Extrae la clave pública de la salida de age-keygen
fn extract_public_key_from_output(output: &str) -> Result<String> {
    debug!("Extrayendo clave pública de la salida de age-keygen");

    // Buscar la línea que contiene "Public key:"
    for line in output.lines() {
        if line.contains("Public key:") || line.starts_with("# public key: ") {
            let public_key = if line.contains("Public key:") {
                line.split("Public key:")
                    .nth(1)
                    .unwrap_or("")
                    .trim()
                    .to_string()
            } else {
                line.replace("# public key: ", "").trim().to_string()
            };

            debug!("Clave pública extraída: {}", public_key);
            return Ok(public_key);
        }
    }

    anyhow::bail!("No se pudo extraer la clave pública de la salida de age-keygen")
}

pub fn init(secrets_dir: &str, secrets_file: &str) -> Result<()> {
    info!("Inicializando directorio de secretos");
    debug!("Directorio: {}, Archivo: {}", secrets_dir, secrets_file);

    verify_sops_installed()?;

    // Crear directorio si no existe
    if !Path::new(secrets_dir).exists() {
        info!("Creando directorio de secretos: {}", secrets_dir);
        fs::create_dir_all(secrets_dir).context("No se pudo crear el directorio de secretos")?;
        println!("📁 Directorio creado: {}", secrets_dir);
    } else {
        info!("El directorio ya existe: {}", secrets_dir);
        println!("📁 Directorio ya existe: {}", secrets_dir);
    }

    // Verificar si el archivo ya existe
    if Path::new(secrets_file).exists() {
        println!("⚠️  El archivo de secretos ya existe: {}", secrets_file);
        println!("💡 Usa 'crypta set --key CLAVE --value VALOR' para añadir secretos");
        return Ok(());
    }

    // Configurar el directorio para las claves Age
    let age_key_dir = format!("{}/sops/age", secrets_dir);
    let age_key_path = format!("{}/key.txt", age_key_dir);

    // Crear directorio para claves Age si no existe
    fs::create_dir_all(&age_key_dir).context("No se pudo crear el directorio para claves Age")?;

    let public_key = if Path::new(&age_key_path).exists() {
        info!("Clave Age ya existe, extrayendo clave pública");
        println!("🔑 Clave Age encontrada: {}", age_key_path);

        // Leer la clave privada existente y extraer la pública
        extract_public_key_from_file(&age_key_path)?
    } else {
        info!("Generando nueva clave Age");
        println!("🔑 Generando nueva clave Age: {}", age_key_path);

        // Generar nueva clave Age
        let output = Command::new("age-keygen")
            .arg("-o")
            .arg(&age_key_path)
            .output()
            .context("No se pudo ejecutar age-keygen. ¿Está instalado age?")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Error al generar la clave Age: {}", error);
        }

        // La clave pública está en stderr de age-keygen
        let stderr_output = String::from_utf8_lossy(&output.stderr);
        extract_public_key_from_output(&stderr_output)?
    };

    // Crear archivo de configuración .sops.yaml
    let sops_config_path = format!("{}/.sops.yaml", secrets_dir);
    if !Path::new(&sops_config_path).exists() {
        info!("Creando archivo de configuración SOPS con clave pública");
        let sops_config = format!(
            r#"# Configuración de SOPS para crypta
# Generado automáticamente
# 
# Clave Age utilizada: {}
# Variables de entorno recomendadas:
#   export SOPS_AGE_KEY_FILE={}
# 
creation_rules:
  - path_regex: \.yml$
    age: {}
"#,
            age_key_path, age_key_path, public_key
        );

        fs::write(&sops_config_path, sops_config)
            .context("No se pudo crear el archivo .sops.yaml")?;

        println!("📄 Archivo de configuración creado: {}", sops_config_path);
    }

    // Configurar variable de entorno SOPS_AGE_KEY_FILE si no está definida
    if std::env::var("SOPS_AGE_KEY_FILE").is_err() {
        println!("⚠️  Variable de entorno no configurada");
        println!("💡 Para usar crypta, añade esto a tu archivo de configuración del shell:");
        println!("   export SOPS_AGE_KEY_FILE={}", age_key_path);
        println!("   ");
        println!(
            "   Bash/Zsh: echo 'export SOPS_AGE_KEY_FILE={}' >> ~/.bashrc",
            age_key_path
        );
        println!(
            "   Fish: echo 'set -gx SOPS_AGE_KEY_FILE {}' >> ~/.config/fish/config.fish",
            age_key_path
        );
    }

    println!("✅ Inicialización completada exitosamente");
    println!("🔐 Clave Age: {}", age_key_path);
    println!("📄 Configuración SOPS: {}", sops_config_path);
    println!("💡 Ahora puedes añadir secretos con: crypta set --key CLAVE --value VALOR");

    Ok(())
}
