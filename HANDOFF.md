# HANDOFF — crypta

## Estado

Feature `cli-safety-flags` mergeada en `develop` (PR #13). Change archivado y plan archivado.

- **Rama:** `develop`
- **Issue:** #12 — https://github.com/atareao/crypta/issues/12
- **Plan:** `plans/PLAN-001-ARCHIVED.md`
- **Change:** `openspec/changes/archive/2026-11-20-cli-safety-flags/`

## Tarea pendiente

Ninguna. Próximo trabajo a definir.

## Follow-ups opcionales (reviewer)

- `access.log` con permisos `0600`.
- Revisar el `enum AccessAction`.
- Mitigar TOCTOU en macOS/BSD.
- Escritura atómica en `render --out`.
- Lints preexistentes en tests.

## Comandos de verificación

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
just check
```

Verificación funcional de los flags:

```sh
crypta render <tpl> --out /tmp/render-prueba
crypta lookup --mask <KEY>
crypta get --mask <KEY>
```

Comprobaciones de seguridad:

- `render --out` crea el fichero con permisos `0600` y deja stdout vacío.
- `lookup --mask` / `get --mask` no muestran el valor y no tocan el portapapeles.
- `render --out` rechaza symlinks, hardlinks al store, ficheros no regulares y `secrets.yml`.
