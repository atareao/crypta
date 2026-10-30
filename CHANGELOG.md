# Changelog
## [0.3.2] - 2026-10-30

### Bug Fixes

- Fix(init): create initial branch as main instead of master

## [0.3.1] - 2026-10-29

### Bug Fixes

- Hotfix: add --filename-override /dev/stdin to sops -e for sops 3.13+ compatibility
- Hotfix: add --filename-override /dev/stdin to sops -e

### Features

- *(init)* Add --remote flag to configure git remote automatically

### Miscellaneous Tasks

- Release v0.3.1

### Other

- 0.3.1
## [0.3.0] - 2026-09-26

### Features

- Feat: add template render command with minijinja

### Miscellaneous Tasks

- Chore(release): v0.3.0

### Other

- V0.3.0
## [0.2.3] - 2026-09-25

### Bug Fixes

- Fix(completion): usar clap_complete::generate para todos los shells

### Dependencies

- *(deps)* Deps: actualizar dependencias
- *(deps)* Deps: actualizar dependencias

### Miscellaneous Tasks

- Chore(release): v0.2.3

### Other

- V0.2.3
## [0.2.2] - 2026-08-29

### Bug Fixes

- Fix(completion): usar clap_complete::generate para todos los shells
- Hotfix(completion): v0.2.2

### Documentation

- Docs(changelog): v0.2.2
## [0.2.1] - 2026-08-29

### Dependencies

- *(deps)* Bump rand from 0.10.0 to 0.10.1

### Miscellaneous Tasks

- Update Cargo.lock after v0.2.0 dependencies
- Chore(release): v0.2.1
## [0.2.0] - 2026-08-29

### Bug Fixes

- Fix double rebase, archivo temporal en claro, tests, clippy, CI y docs

### Features

- Actualiza la versión a 0.1.11 en .vampus.yml y Cargo.toml
## [0.1.11] - 2026-03-28

### Features

- Mejoras en la gestión de credenciales SSH, manejo de errores en push y refactorización del código.
- Actualiza la versión a 0.1.9 y añade opción para usar git del sistema en pull y push
- Actualiza la versión a 0.1.10 en archivos de configuración y dependencias
- Añade funcionalidad para generar contraseñas aleatorias con opciones de longitud y caracteres especiales

### Miscellaneous Tasks

- Mejorado el proceso de fetch y rebase con implementación en Rust y fallback a git CLI en caso de error
- Actualiza dependencias y versiones de crates en Cargo.lock y Cargo.toml
## [0.1.8] - 2026-02-14

### Dependencies

- *(deps)* Bump git2 from 0.18.3 to 0.20.4
## [0.1.1] - 2026-01-25

### Features

- Actualiza lógica de encriptación y añade comando 'show' para mostrar secretos por stdout
