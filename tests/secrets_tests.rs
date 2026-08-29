use tempfile::TempDir;

#[test]
fn test_secrets_file_creation() {
    let temp_dir = TempDir::new().unwrap();
    let secrets_dir = temp_dir.path().join(".secrets");
    let secrets_file = secrets_dir.join("secrets.yml");
    
    // Verificar que el directorio no existe inicialmente
    assert!(!secrets_dir.exists());
    assert!(!secrets_file.exists());
}

#[test]
fn test_temp_directory_cleanup() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();
    
    // El directorio temporal existe
    assert!(path.exists());
    
    drop(temp_dir);
    
    // Después de drop, el directorio se limpia
    assert!(!path.exists());
}

#[test]
fn test_yaml_parsing() {
    use serde_yaml::Value;
    
    let yaml_str = "key1: value1\nkey2: value2";
    let yaml: Value = serde_yaml::from_str(yaml_str).unwrap();
    
    if let Value::Mapping(map) = yaml {
        assert_eq!(map.len(), 2);
        assert!(map.contains_key(&Value::String("key1".to_string())));
        assert!(map.contains_key(&Value::String("key2".to_string())));
    } else {
        panic!("Expected a mapping");
    }
}

#[test]
fn test_yaml_modification() {
    use serde_yaml::Value;
    
    let yaml_str = "key1: value1";
    let mut yaml: Value = serde_yaml::from_str(yaml_str).unwrap();
    
    if let Value::Mapping(ref mut map) = yaml {
        map.insert(
            Value::String("key2".to_string()),
            Value::String("value2".to_string())
        );
        assert_eq!(map.len(), 2);
    }
    
    let updated = serde_yaml::to_string(&yaml).unwrap();
    assert!(updated.contains("key1"));
    assert!(updated.contains("key2"));
}

#[test]
fn test_yaml_removal() {
    use serde_yaml::Value;
    
    let yaml_str = "key1: value1\nkey2: value2";
    let mut yaml: Value = serde_yaml::from_str(yaml_str).unwrap();
    
    if let Value::Mapping(ref mut map) = yaml {
        map.remove(Value::String("key1".to_string()));
        assert_eq!(map.len(), 1);
        assert!(!map.contains_key(&Value::String("key1".to_string())));
    }
}

#[test]
fn test_yaml_value_extraction() {
    use serde_yaml::Value;
    
    let yaml_str = "api_key: secret123\nuser: admin";
    let yaml: Value = serde_yaml::from_str(yaml_str).unwrap();
    
    // Extraer un valor específico
    let api_key = yaml.get("api_key")
        .and_then(|v| v.as_str());
    
    assert_eq!(api_key, Some("secret123"));
    
    let user = yaml.get("user")
        .and_then(|v| v.as_str());
    
    assert_eq!(user, Some("admin"));
    
    // Intentar extraer una clave que no existe
    let nonexistent = yaml.get("nonexistent")
        .and_then(|v| v.as_str());
    
    assert_eq!(nonexistent, None);
}
