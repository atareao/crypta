# Tasks — init-default-branch-main

## TDD checklist

- [x] **RED**: escribir test que verifique que `init_repo_with_remote` deja la rama activa en `main` (falla: libgit2 crea `master`)
- [x] **GREEN**: añadir `repo.set_head("refs/heads/main")` en `init_repo_with_remote()` tras `Repository::init()`
- [x] **REFACTOR**: `cargo fmt --check`, `cargo clippy -- -D warnings`
- [x] Verificar que los 37 tests existentes siguen pasando
- [x] `cargo test` completo en verde (38/38)
