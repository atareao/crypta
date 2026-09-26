# Proposal

## Why

`sops 3.13.3` (la versión instalada) introdujo un cambio de comportamiento: cuando `sops -e` lee de stdin (sin archivo), necesita `--filename-override` para matchear `path_regex` en `.sops.yaml`. Sin este flag, falla con `Error: no file specified` porque el filename virtual es `/dev/stdin`, que no matchea `.*\.yml$`.

Esto rompe cualquier operación de escritura en crypta: `set`, `store`, `delete` e `import`.

## What Changes

- **`src/secrets.rs` — `encrypt_with_sops()`**: Añadir `"--filename-override"` y el nombre base del archivo de secretos (ej. `secrets.yml`) a los argumentos de `sops -e`.
- **Ningún otro cambio**: Las 7 llamadas a `sops -d` pasan el archivo como argumento posicional y no se ven afectadas.

## Capabilities

### Modified Capabilities

- `secrets-encrypt`: La función `encrypt_with_sops` ahora pasa `--filename-override <basename>` a `sops -e` para que `path_regex` en `.sops.yaml` matchee correctamente al leer desde stdin.

## Impact

- **Código**: Cambio de una línea en `src/secrets.rs` (añadir 2 argumentos al `Command`).
- **Tests**: Se añade un test unitario en `src/secrets.rs` que verifica que `encrypt_with_sops` invoca `sops` con `--filename-override`.
- **CLI**: Sin cambios en la interfaz.
- **Rendimiento**: Sin impacto.