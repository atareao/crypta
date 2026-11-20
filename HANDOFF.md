# HANDOFF — crypta

## Estado

Feature `cli-safety-flags` implementada y auditada. Pendiente commit y PR.

- **Rama:** `feature/cli-safety-flags`
- **Issue:** #12 — https://github.com/atareao/crypta/issues/12
- **Plan:** `plans/PLAN-001.md`
- **Change:** `openspec/changes/cli-safety-flags/`

## Qué falta

1. Commit de los cambios en `src/`, `tests/`, `docs/`, skill y bookkeeping con gitmoji.
2. PR de `feature/cli-safety-flags` a `develop`.
3. `openspec archive cli-safety-flags`.
4. Renombrar `plans/PLAN-001.md` a `plans/PLAN-001-ARCHIVED.md`.

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
