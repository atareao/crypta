# HANDOFF — crypta

## Estado

Release v0.3.4 publicada y mergeada en `main` (commit `8befffe`, tag `v0.3.4`); `develop` en `786532a` con el back-merge. Feature `cli-safety-flags` (PR #13) y skill `crypta` ya están en `main`. Binario reinstalado localmente en v0.3.4. GitHub release creada: https://github.com/atareao/crypta/releases/tag/v0.3.4

## Tarea pendiente

Ninguna. La release v0.3.4 está publicada.

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
