# Tasks

## 1. RED: Tests para init con --remote ✅

- [x] 1.1 Escribir test unitario que verifique que `init()` con `remote_url = Some(...)` configura git
- [x] 1.2 Ejecutar `cargo test` y confirmar que el test falla (RED)

## 2. GREEN: Implementar remote setup en init ✅

- [x] 2.1 Modificar `init()` en `src/secrets.rs` para aceptar `remote_url: Option<&str>`
- [x] 2.2 Implementar helpers git: init repo, add files, create initial commit, configure remote
- [x] 2.3 Implementar lógica interactiva: pregunta al usuario cuando `remote_url` es None
- [x] 2.4 Añadir flag `--remote` / `-r` al comando `Init` en `src/main.rs`
- [x] 2.5 Ejecutar `cargo test` y confirmar que todos los tests pasan (GREEN)
- [x] 2.6 Ejecutar `cargo check` y confirmar compilación sin errores

## 3. REFACTOR: Calidad ✅

- [x] 3.1 Ejecutar `cargo clippy -- -D warnings` y corregir warnings
- [x] 3.2 Ejecutar `cargo fmt --check` y corregir formato
- [x] 3.3 Ejecutar `cargo test` y confirmar todo en verde
- [x] 3.4 Verificación manual
