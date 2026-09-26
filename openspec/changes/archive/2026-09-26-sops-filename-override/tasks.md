# Tasks

## 1. RED: Test unitario que verifica --filename-override

- [x] 1.1 Añadir test en `src/secrets.rs` que verifique que `encrypt_with_sops` construye el comando `sops` con `--filename-override` y el nombre base del archivo
- [x] 1.2 Ejecutar `cargo test` y confirmar que el test nuevo falla (RED)

## 2. GREEN: Implementar --filename-override en encrypt_with_sops

- [x] 2.1 Modificar `encrypt_with_sops` en `src/secrets.rs`: añadir `"--filename-override"` y `/dev/stdin` a los args de `Command::new("sops")`
- [x] 2.2 Ejecutar `cargo test` y confirmar que todos los tests pasan (GREEN)
- [x] 2.3 Ejecutar `cargo check` y confirmar que compila sin errores

## 3. REFACTOR: Clippy + fmt

- [x] 3.1 Ejecutar `cargo clippy -- -D warnings` y corregir cualquier warning
- [x] 3.2 Ejecutar `cargo fmt --check` y corregir formato si es necesario
- [x] 3.3 Ejecutar `cargo test` y confirmar que todo sigue en verde

## 4. Verificación con sops real

- [ ] 4.1 Ejecutar `crypta set -k test_key -v test_value` con `SOPS_AGE_KEY_FILE` apuntando a la clave real y confirmar que funciona sin errores