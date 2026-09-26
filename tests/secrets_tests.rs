use std::process::Command;
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
            Value::String("value2".to_string()),
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
    let api_key = yaml.get("api_key").and_then(|v| v.as_str());

    assert_eq!(api_key, Some("secret123"));

    let user = yaml.get("user").and_then(|v| v.as_str());

    assert_eq!(user, Some("admin"));

    // Intentar extraer una clave que no existe
    let nonexistent = yaml.get("nonexistent").and_then(|v| v.as_str());

    assert_eq!(nonexistent, None);
}

#[test]
fn test_import_env_format() {
    let input = "KEY1=valor1\nKEY2=valor2\n# comentario\nKEY3=valor3";
    let pairs: std::collections::BTreeMap<String, String> = input
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
        .filter_map(|l| {
            let t = l.trim();
            let eq_pos = t.find('=')?;
            let key = t[..eq_pos].trim().to_string();
            let value = t[eq_pos + 1..].trim().to_string();
            if key.is_empty() {
                None
            } else {
                Some((key, value))
            }
        })
        .collect();

    assert_eq!(pairs.len(), 3);
    assert_eq!(pairs.get("KEY1"), Some(&"valor1".to_string()));
    assert_eq!(pairs.get("KEY2"), Some(&"valor2".to_string()));
    assert_eq!(pairs.get("KEY3"), Some(&"valor3".to_string()));
}

#[test]
fn test_import_json_format() {
    let input = r#"{"API_KEY": "secret123", "DB_URL": "postgres://localhost"}"#;
    let pairs: std::collections::BTreeMap<String, String> = serde_json::from_str(input).unwrap();

    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs.get("API_KEY"), Some(&"secret123".to_string()));
    assert_eq!(
        pairs.get("DB_URL"),
        Some(&"postgres://localhost".to_string())
    );
}

#[test]
fn test_import_yaml_format() {
    let input = "api_key: secret123\ndb_url: postgres://localhost\n";
    let pairs: std::collections::BTreeMap<String, String> = serde_yaml::from_str(input).unwrap();

    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs.get("api_key"), Some(&"secret123".to_string()));
    assert_eq!(
        pairs.get("db_url"),
        Some(&"postgres://localhost".to_string())
    );
}

#[test]
fn test_export_env_format() {
    let mut pairs = std::collections::BTreeMap::new();
    pairs.insert("KEY1".to_string(), "valor1".to_string());
    pairs.insert("KEY2".to_string(), "valor2".to_string());

    let mut out = String::new();
    for (key, value) in &pairs {
        out.push_str(&format!("{}={}\n", key, value));
    }

    assert!(out.contains("KEY1=valor1"));
    assert!(out.contains("KEY2=valor2"));
}

#[test]
fn test_export_json_format() {
    let mut pairs = std::collections::BTreeMap::new();
    pairs.insert("KEY1".to_string(), "valor1".to_string());

    let json = serde_json::to_string_pretty(&pairs).unwrap();
    assert!(json.contains("KEY1"));
    assert!(json.contains("valor1"));
}

#[test]
fn test_export_yaml_format() {
    let mut pairs = std::collections::BTreeMap::new();
    pairs.insert("KEY1".to_string(), "valor1".to_string());

    let yaml = serde_yaml::to_string(&pairs).unwrap();
    assert!(yaml.contains("KEY1"));
    assert!(yaml.contains("valor1"));
}

#[test]
fn test_import_empty_input() {
    let input = "";
    let pairs: std::collections::BTreeMap<String, String> = input
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
        .filter_map(|l| {
            let t = l.trim();
            let eq_pos = t.find('=')?;
            let key = t[..eq_pos].trim().to_string();
            let value = t[eq_pos + 1..].trim().to_string();
            if key.is_empty() {
                None
            } else {
                Some((key, value))
            }
        })
        .collect();

    assert!(pairs.is_empty());
}

#[test]
fn test_import_env_with_comments() {
    let input = "# esto es un comentario\nKEY=val\n# otro comentario";
    let pairs: std::collections::BTreeMap<String, String> = input
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
        .filter_map(|l| {
            let t = l.trim();
            let eq_pos = t.find('=')?;
            let key = t[..eq_pos].trim().to_string();
            let value = t[eq_pos + 1..].trim().to_string();
            if key.is_empty() {
                None
            } else {
                Some((key, value))
            }
        })
        .collect();

    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs.get("KEY"), Some(&"val".to_string()));
}

/// Test que verifica que encrypt_with_sops funciona con --filename-override
///
/// Requiere: sops y age-keygen instalados en el sistema.
/// Sin --filename-override, sops 3.13+ falla porque /dev/stdin no matchea
/// path_regex: .*\.yml$ en .sops.yaml.
#[test]
fn test_encrypt_with_filename_override() {
    let temp_dir = TempDir::new().unwrap();
    let secrets_dir = temp_dir.path().join(".secrets");
    let secrets_file = secrets_dir.join("secrets.yml");
    let age_key_dir = secrets_dir.join("sops/age");
    let age_key_path = age_key_dir.join("key.txt");

    // Crear directorios
    std::fs::create_dir_all(&age_key_dir).expect("crear dir age");

    // Generar clave Age
    let output = Command::new("age-keygen")
        .arg("-o")
        .arg(&age_key_path)
        .output()
        .expect("age-keygen debe estar instalado");
    assert!(output.status.success(), "age-keygen falló");

    // Extraer clave pública del stderr de age-keygen
    // age-keygen escribe "Public key:" (con P mayúscula) en stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    let public_key = stderr
        .lines()
        .find(|l| l.to_lowercase().contains("public key:"))
        .and_then(|l| {
            l.split("Public key:")
                .nth(1)
                .or_else(|| l.split("# public key:").nth(1))
        })
        .map(|s| s.trim())
        .expect("No se pudo extraer la clave pública");

    // Crear .sops.yaml con path_regex .*\.yml$
    let sops_config = format!(
        r#"creation_rules:
  - path_regex: .*\.yml$
    age: "{}"
"#,
        public_key
    );
    std::fs::write(secrets_dir.join(".sops.yaml"), &sops_config).expect("escribir .sops.yaml");

    // Configurar variable de entorno para la clave Age
    unsafe {
        std::env::set_var("SOPS_AGE_KEY_FILE", &age_key_path);
    }

    // Ejecutar add() — esto llama a encrypt_with_sops internamente
    crypta::secrets::add(
        secrets_dir.to_str().unwrap(),
        secrets_file.to_str().unwrap(),
        "test_key",
        "test_value",
    )
    .expect("add() debe funcionar con --filename-override");

    // Verificar que el archivo se creó y está encriptado
    assert!(secrets_file.exists(), "El archivo debe existir");
    let content = std::fs::read_to_string(&secrets_file).expect("leer secrets.yml");
    assert!(
        content.contains("ENC[AES256_GCM,"),
        "El contenido debe estar encriptado: {}",
        content
    );

    // Limpiar variable de entorno
    unsafe {
        std::env::remove_var("SOPS_AGE_KEY_FILE");
    }
}
