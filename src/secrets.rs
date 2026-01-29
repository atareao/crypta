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
    fs::create_dir_all(secrets_dir)
        .context("No se pudo crear el directorio de secretos")?;

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
        
        String::from_utf8(output.stdout)
            .context("El contenido desencriptado no es UTF-8 válido")?
    };

    // Parsear YAML y actualizar
    let mut data: Value = if decrypted_content.is_empty() {
        Value::Mapping(serde_yaml::Mapping::new())
    } else {
        serde_yaml::from_str(&decrypted_content)
            .context("No se pudo parsear el contenido YAML")?  
    };
    
    if let Value::Mapping(ref mut map) = data {
        map.insert(Value::String(key.to_string()), Value::String(value.to_string()));
    }
    
    let updated_yaml = serde_yaml::to_string(&data)
        .context("No se pudo serializar el YAML")?;
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
    
    let yaml: Value = serde_yaml::from_str(&decrypted_content)
        .context("No se pudo parsear el contenido YAML")?;
    
    let val = yaml.get(key)
        .and_then(|v| v.as_str())
        .context(format!("La clave '{}' no existe", key))?;
    
    // Copiar al portapapeles
    debug!("Copiando al portapapeles");
    let mut clipboard = Clipboard::new()
        .context("No se pudo acceder al portapapeles")?;
    clipboard.set_text(val)
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
    
    let yaml: Value = serde_yaml::from_str(&decrypted_content)
        .context("No se pudo parsear el contenido YAML")?;
    
    let val = yaml.get(key)
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
    
    let yaml: Value = serde_yaml::from_str(&decrypted_content)
        .context("No se pudo parsear el contenido YAML")?;
    
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
        anyhow::bail!(
            "El archivo de secretos no existe: {}",
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
    
    let mut yaml: Value = serde_yaml::from_str(&decrypted_content)
        .context("No se pudo parsear el contenido YAML")?;
    
    if let Value::Mapping(ref mut map) = yaml {
        map.remove(&Value::String(key.to_string()));
    }
    
    let updated_yaml = serde_yaml::to_string(&yaml)
        .context("No se pudo serializar el YAML")?;
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
    
    let output = Command::new("which")
        .arg("sops")
        .output();
    
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
    let work_dir = secrets_path.parent()
        .context("No se pudo obtener el directorio del archivo de secretos")?;
    
    // Escribir contenido a un archivo temporal .yml en el mismo directorio
    // para que SOPS pueda aplicar las reglas de creación basadas en path
    use std::io::Write;
    let temp_file_path = work_dir.join(".crypta_temp.yml");
    let mut temp_file = fs::File::create(&temp_file_path)
        .context("No se pudo crear archivo temporal")?;
    temp_file.write_all(yaml_content.as_bytes())
        .context("No se pudo escribir al archivo temporal")?;
    drop(temp_file);  // Cerrar el archivo
    
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
