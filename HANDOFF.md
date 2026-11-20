# HANDOFF — crypta

## Estado

Feature `cli-safety-flags` implementada y auditada. Commit y PR abiertos; pendiente merge y cierre.

- **Rama:** `feature/cli-safety-flags`
- **Issue:** #12 — https://github.com/atareao/crypta/issues/12
- **Commit:** `cb93d8e ✨ feat(cli): flags de seguridad --out y --mask para uso por agentes`
- **PR:** https://github.com/atareao/crypta/pull/13 (base `develop`, asignado a atareao)
- **Plan:** `plans/PLAN-001.md`
- **Change:** `openspec/changes/cli-safety-flags/`

## Qué falta

Pendiente tras el merge del PR:

1. `openspec archive cli-safety-flags`.
2. Renombrar `plans/PLAN-001.md` a `plans/PLAN-001-ARCHIVED.md`.

## Comandos de verificación

`just check` en verde, 70 tests.

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
