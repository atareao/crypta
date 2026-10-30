# Propuesta: Rama inicial `main` en `crypta init`

## Problema

Cuando `crypta init --remote <URL>` (o el flujo interactivo) inicializa el repositorio git
con libgit2 (`Repository::init()`), la rama por defecto creada es **`master`** — el
default de libgit2 — en lugar de `main`.

Esto rompe el flujo posterior:

- `push()` empuja `refs/heads/main:refs/heads/main` → falla porque localmente no existe `main`
- `sync()` hace pull/push de `main` → mismamente
- El usuario se queda con un repo local en `master` que no sincroniza con nada

## Solución propuesta

Después de `Repository::init()`, forzar HEAD a `refs/heads/main` con `repo.set_head("refs/heads/main")`
**antes** de crear el commit inicial. Así el primer commit y toda la historia nacen en `main`,
coherente con `push()` (`refs/heads/main:refs/heads/main`) y `pull_rebase()` (`origin main`).

## Impacto

- **Scope**: `src/git.rs` — solo la función `init_repo_with_remote()`
- **Breaking changes**: ninguno
- **Dependencias**: ninguna — `git2::Repository::set_head` ya existe en la API

## Contratos

### Rust (función modificada)

```rust
pub fn init_repo_with_remote(secrets_dir: &str, remote_url: &str) -> Result<()>
```

- El repositorio inicializado DEBE tener HEAD apuntando a `refs/heads/main`
- El commit inicial DEBE crearse sobre `refs/heads/main`
- El push DEBE dirigirse a `refs/heads/main:refs/heads/main` (sin cambios)

## Escenarios

### Happy path
1. Usuario ejecuta `crypta init --remote git@github.com:user/secrets.git`
2. Se crea repo git con rama activa `main`
3. Commit inicial sobre `main`
4. Push a `origin main` exitoso
5. `git branch --show-current` devuelve `main`
