# HANDOFF — crypta

## Estado

Feature `cli-safety-flags` mergeada en `develop` (PR #13). Change archivado y plan archivado. Skill movida a `skills/crypta/SKILL.md` con symlink en `.opencode/skills/crypta/SKILL.md` y documentada en el README.

- **Rama:** `develop` en `7104e2a`
- **Issue:** #12 — https://github.com/atareao/crypta/issues/12
- **Plan:** `plans/PLAN-001-ARCHIVED.md`
- **Change:** `openspec/changes/archive/2026-11-20-cli-safety-flags/`
- **Skill:** `skills/crypta/SKILL.md` (symlink: `.opencode/skills/crypta/SKILL.md`)
- **`main`:** `8d50c8e` (v0.3.3), sin la feature ni la skill

## Tarea pendiente

**Publicar release v0.3.4** para llevar todo a `main` (feature + skill). Se hará esta tarde.

```sh
just release-start v0.3.4
just release-finish v0.3.4
```

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
