use std::path::Path;
use serde_yaml::Value;
use anyhow::{Result, Context};
use arboard::Clipboard;
use rops::file::builder::RopsFileBuilder;

use crate::{EncryptedRopsFile, DecryptedRopsFile, YamlFileFormat};

pub fn add(secrets_dir: &str, secrets_file: &str, key: &str, value: &str) -> Result<()> {
    // Inicializar archivo si no existe
    if !Path::new(secrets_file).exists() {
        std::fs::create_dir_all(secrets_dir).context("No se pudo crear el directorio de secretos")?;
        
        // Crear archivo YAML vacío con el secreto inicial
        let initial_yaml = format!("{}: {}", key, value);
        
        // Encriptar usando rops - necesita una clave Age configurada
        let builder = RopsFileBuilder::<YamlFileFormat>::new(&initial_yaml)?;
        
        // Obtener la clave Age desde las variables de entorno
        if let Ok(age_key) = std::env::var("SOPS_AGE_KEY_FILE") {
            eprintln!("ℹ️ Usando clave Age desde: {}", age_key);
        }
        
        let encrypted: EncryptedRopsFile = builder.encrypt()?;
        std::fs::write(secrets_file, encrypted.to_string())?;
        
        println!("✅ Secreto '{}' añadido/actualizado correctamente.", key);
        return Ok(());
    }

    // Desencriptar, modificar y reencriptar
    let encrypted_content = std::fs::read_to_string(secrets_file)?;
    let encrypted: EncryptedRopsFile = encrypted_content.parse()?;
    let decrypted: DecryptedRopsFile = encrypted.decrypt()?;
    
    // Obtener el mapa actual y parsearlo como YAML
    let current_yaml = decrypted.map().to_string();
    let mut yaml: Value = serde_yaml::from_str(&current_yaml)?;
    
    if let Value::Mapping(ref mut map) = yaml {
        map.insert(
            Value::String(key.to_string()),
            Value::String(value.to_string())
        );
    }
    
    let updated_yaml = serde_yaml::to_string(&yaml)?;
    let builder = RopsFileBuilder::<YamlFileFormat>::new(&updated_yaml)?;
    let encrypted: EncryptedRopsFile = builder.encrypt()?;
    std::fs::write(secrets_file, encrypted.to_string())?;
    
    println!("✅ Secreto '{}' añadido/actualizado correctamente.", key);
    Ok(())
}

pub fn get(secrets_file: &str, key: &str) -> Result<()> {
    let encrypted_content = std::fs::read_to_string(secrets_file)
        .context("No se pudo leer el archivo de secretos")?;
    let encrypted: EncryptedRopsFile = encrypted_content.parse()?;
    let decrypted: DecryptedRopsFile = encrypted.decrypt()
        .context("No se pudo desencriptar el archivo de secretos")?;
    
    let yaml_str = decrypted.map().to_string();
    let yaml: Value = serde_yaml::from_str(&yaml_str)?;
    
    let val = yaml.get(key)
        .and_then(|v| v.as_str())
        .context(format!("La clave '{}' no existe", key))?;
    
    // Copiar al portapapeles
    let mut clipboard = Clipboard::new()
        .context("No se pudo acceder al portapapeles")?;
    clipboard.set_text(val)
        .context("No se pudo copiar al portapapeles")?;
    
    println!("📋 Secreto '{}' copiado al portapapeles.", key);
    Ok(())
}

pub fn list(secrets_file: &str) -> Result<()> {
    let encrypted_content = std::fs::read_to_string(secrets_file)
        .context("No se pudo leer el archivo de secretos")?;
    let encrypted: EncryptedRopsFile = encrypted_content.parse()?;
    let decrypted: DecryptedRopsFile = encrypted.decrypt()
        .context("No se pudo desencriptar el archivo de secretos")?;
    
    let yaml_str = decrypted.map().to_string();
    let yaml: Value = serde_yaml::from_str(&yaml_str)?;
    
    println!("🔑 Claves en {}:", secrets_file);
    
    if let Value::Mapping(map) = yaml {
        for key in map.keys() {
            if let Some(key_str) = key.as_str() {
                println!("- {}", key_str);
            }
        }
    }
    
    Ok(())
}

pub fn remove(secrets_file: &str, key: &str) -> Result<()> {
    let encrypted_content = std::fs::read_to_string(secrets_file)
        .context("No se pudo leer el archivo de secretos")?;
    let encrypted: EncryptedRopsFile = encrypted_content.parse()?;
    let decrypted: DecryptedRopsFile = encrypted.decrypt()
        .context("No se pudo desencriptar el archivo de secretos")?;
    
    let yaml_str = decrypted.map().to_string();
    let mut yaml: Value = serde_yaml::from_str(&yaml_str)?;
    
    if let Value::Mapping(ref mut map) = yaml {
        map.remove(Value::String(key.to_string()));
    }
    
    let updated_yaml = serde_yaml::to_string(&yaml)?;
    let builder = RopsFileBuilder::<YamlFileFormat>::new(&updated_yaml)?;
    let encrypted: EncryptedRopsFile = builder.encrypt()?;
    std::fs::write(secrets_file, encrypted.to_string())?;
    
    println!("🗑️ Secreto '{}' eliminado.", key);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_yaml_value_operations() {
        let yaml_str = "test_key: test_value";
        let yaml: Value = serde_yaml::from_str(yaml_str).unwrap();
        
        assert!(yaml.get("test_key").is_some());
        assert_eq!(yaml.get("test_key").unwrap().as_str().unwrap(), "test_value");
    }

    #[test]
    fn test_yaml_mapping_insert() {
        let mut yaml = Value::Mapping(serde_yaml::Mapping::new());
        
        if let Value::Mapping(ref mut map) = yaml {
            map.insert(
                Value::String("key1".to_string()),
                Value::String("value1".to_string())
            );
            assert_eq!(map.len(), 1);
        }
    }

    #[test]
    fn test_yaml_mapping_remove() {
        let yaml_str = "key1: value1\nkey2: value2";
        let mut yaml: Value = serde_yaml::from_str(yaml_str).unwrap();
        
        if let Value::Mapping(ref mut map) = yaml {
            let removed = map.remove(Value::String("key1".to_string()));
            assert!(removed.is_some());
            assert_eq!(map.len(), 1);
        }
    }

    #[test]
    fn test_directory_creation() {
        let temp_dir = TempDir::new().unwrap();
        let secrets_dir = temp_dir.path().join("test_secrets");
        
        fs::create_dir_all(&secrets_dir).unwrap();
        assert!(secrets_dir.exists());
        assert!(secrets_dir.is_dir());
    }
}
