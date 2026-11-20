# Tareas — cli-safety-flags

## Fase 0 — Caracterización

- [x] 0.1 Confirmar comportamiento actual de `render`, `lookup` y `get`.
- [x] 0.2 Añadir tests de caracterización si faltan y verificar GREEN.

## Fase 1 — Spec

- [x] 1.1 Crear el change `cli-safety-flags`.
- [x] 1.2 Redactar `proposal.md`.
- [x] 1.3 Redactar `specs/cli/spec.md`.
- [x] 1.4 Redactar `tasks.md`.
- [x] 1.5 Aprobación del usuario.

## Fase 2 — TDD

- [x] 2.1 RED: tests de `render --out` (éxito y error de escritura).
- [x] 2.2 GREEN: implementar `render --out`.
- [x] 2.3 RED: tests de `lookup --mask` y `get --mask`.
- [x] 2.4 GREEN: implementar `--mask`.
- [x] 2.5 REFACTOR: `cargo fmt` + `cargo clippy -- -D warnings`.
- [x] 2.6 `just check` en verde.

## Fase 3 — Skill y docs

- [x] 3.1 Crear la skill `crypta`.
- [x] 3.2 Crear `docs/cli-safety-flags.md`.

## Fase 4 — Cierre

- [ ] 4.1 PR a `develop`.
- [ ] 4.2 `openspec archive cli-safety-flags`.
- [ ] 4.3 Archivar `plans/PLAN-001.md`.
- [ ] 4.4 Actualizar `HANDOFF.md`.
