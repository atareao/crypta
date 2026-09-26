# Tasks

## 1. Dependencias y módulo

- [ ] 1.1 Añadir `minijinja = "6.0"` a `[dependencies]` en `Cargo.toml` y verificar que `cargo check` compila sin errores
- [ ] 1.2 Crear `src/templates.rs` con la función `pub fn render_template(template_content: &str, secrets: &BTreeMap<String, String>) -> Result<String>` y test unitario que verifique renderizado básico
- [ ] 1.3 Añadir `pub mod templates;` a `src/lib.rs` y verificar que `cargo check` compila

## 2. Lógica de integración con secrets

- [ ] 2.1 Implementar función auxiliar en `templates.rs` que decripte el secrets file y devuelva `BTreeMap<String, String>` (reutilizando patrón de `export_secrets`)
- [ ] 2.2 Verificar con test unitario que el mapa incluye todas las claves del YAML desencriptado

## 3. CLI: subcomando render

- [ ] 3.1 Añadir variante `Render` al enum `Commands` en `main.rs` con alias `r` y campo opcional `file: Option<String>`, y verificar que `cargo check` compila
- [ ] 3.2 Implementar el brazo `Commands::Render { file }` en `run_command`: leer archivo (o stdin), llamar al módulo templates, imprimir resultado
- [ ] 3.3 Verificar funcionalidad completa con test de integración que ejecuta el binario compilado con una plantilla temporal

## 4. Manejo de errores

- [ ] 4.1 Capturar `UndefinedError` de minijinja y convertirlo en mensaje descriptivo en español: "La variable 'X' no existe en el store de secretos"
- [ ] 4.2 Verificar con test que archivo inexistente produce error
- [ ] 4.3 Verificar con test que variable faltante produce error con el nombre de la variable en el mensaje