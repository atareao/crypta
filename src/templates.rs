use anyhow::{bail, Context, Result};
use minijinja::{Environment, UndefinedBehavior, Value};
use std::collections::BTreeMap;
use std::io::Read;

/// Renderiza una plantilla Jinja2 reemplazando las variables con los valores
/// del mapa de secretos.
///
/// - `template_content`: el contenido de la plantilla en formato Jinja2
/// - `secrets`: mapa con las variables a inyectar en la plantilla
///
/// Devuelve el texto renderizado o un error si alguna variable no está definida.
pub fn render_template(
    template_content: &str,
    secrets: &BTreeMap<String, String>,
) -> Result<String> {
    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Strict);
    let ctx = Value::from_serialize(secrets);
    env.render_str(template_content, ctx).map_err(|e| {
        let msg = format!("{:#}", e);
        if msg.contains("is not defined") {
            if let Some(name) = msg.split('\'').nth(1) {
                anyhow::anyhow!("La variable '{}' no existe en el store de secretos", name)
            } else {
                anyhow::anyhow!("{}", msg)
            }
        } else {
            anyhow::anyhow!("{}", msg)
        }
    })
}

/// Obtiene un mapa plano de claves/valores desde el archivo de secretos
/// desencriptado vía sops.
///
/// - `secrets_file`: ruta al archivo YAML encriptado con sops
pub fn get_secrets_map(secrets_file: &str) -> Result<BTreeMap<String, String>> {
    let file_path = std::path::Path::new(secrets_file);
    if !file_path.exists() {
        bail!("El archivo de secretos no existe: {}", secrets_file);
    }

    let output = std::process::Command::new("sops")
        .arg("-d")
        .arg(secrets_file)
        .output()
        .context("No se pudo ejecutar sops")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("Error al desencriptar: {}", error);
    }

    let decrypted = String::from_utf8(output.stdout)
        .context("El contenido desencriptado no es UTF-8 válido")?;

    let yaml: serde_yaml::Value =
        serde_yaml::from_str(&decrypted).context("No se pudo parsear el contenido YAML")?;

    let mut map = BTreeMap::new();
    if let serde_yaml::Value::Mapping(mapping) = yaml {
        for (k, v) in &mapping {
            if let (Some(key_str), Some(val_str)) = (k.as_str(), v.as_str()) {
                map.insert(key_str.to_string(), val_str.to_string());
            }
        }
    }

    Ok(map)
}

/// Lee una plantilla desde archivo (o stdin) y la renderiza con los secretos.
///
/// - `secrets_file`: ruta al archivo de secretos
/// - `file_path`: ruta al archivo de plantilla, o `None` para leer desde stdin
pub fn render_file(secrets_file: &str, file_path: Option<&str>) -> Result<String> {
    let content = match file_path {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|| format!("No se pudo leer el archivo: {}", path))?,
        None => {
            let mut buf = String::new();
            std::io::stdin()
                .lock()
                .read_to_string(&mut buf)
                .context("No se pudo leer desde stdin")?;
            buf
        }
    };

    let secrets = get_secrets_map(secrets_file)?;
    render_template(&content, &secrets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_basic() {
        let template = "Host: {{ DB_HOST }}";
        let mut secrets = BTreeMap::new();
        secrets.insert("DB_HOST".to_string(), "localhost".to_string());
        let result = render_template(template, &secrets).unwrap();
        assert_eq!(result, "Host: localhost");
    }

    #[test]
    fn test_render_no_vars() {
        let template = "texto literal";
        let secrets = BTreeMap::new();
        let result = render_template(template, &secrets).unwrap();
        assert_eq!(result, "texto literal");
    }

    #[test]
    fn test_render_empty_template() {
        let template = "";
        let secrets = BTreeMap::new();
        let result = render_template(template, &secrets).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_render_default_filter() {
        let template = "{{ TIMEOUT | default(\"30\") }}";
        let secrets = BTreeMap::new();
        let result = render_template(template, &secrets).unwrap();
        assert_eq!(result, "30");
    }

    #[test]
    fn test_render_undefined_var() {
        let template = "{{ MISSING }}";
        let secrets = BTreeMap::new();
        let err = render_template(template, &secrets).unwrap_err();
        let err_msg = format!("{:#}", err);
        assert!(
            err_msg.contains("MISSING"),
            "El error debe contener el nombre de la variable faltante: {}",
            err_msg
        );
    }

    #[test]
    fn test_render_multiple_vars() {
        let template = "{{ A }}-{{ B }}";
        let mut secrets = BTreeMap::new();
        secrets.insert("A".to_string(), "x".to_string());
        secrets.insert("B".to_string(), "y".to_string());
        let result = render_template(template, &secrets).unwrap();
        assert_eq!(result, "x-y");
    }
}
