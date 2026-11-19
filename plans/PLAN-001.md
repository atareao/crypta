# PLAN-001 — Skill crypta + flags de seguridad en CLI

- **Issue:** #12 (https://github.com/atareao/crypta/issues/12)
- **Rama:** `feature/cli-safety-flags` (desde `develop`)
- **Estado:** implementado y auditado; pendiente commit + PR
- **Fecha:** 2026-11-19

## Objetivo

Permitir que un agente (o un humano) use Crypta sin exponer secretos en claro en el contexto, los logs o la salida estándar. Se descarta un servidor MCP: la frontera de seguridad real es el CLI, no el transporte.

## Alcance

1. `crypta render --out <path>`: escribe el resultado renderizado a un fichero y no imprime el contenido.
2. `crypta lookup --mask` (y `get --mask` si aplica): muestra existencia/metadatos, nunca el valor.
3. Skill `crypta` con el flujo seguro para agentes.
4. Documentación `docs/cli-safety-flags.md`.

## Fase 0 — Caracterización (solo si se toca código sin specs)

- [x] Revisar `src/main.rs` y `src/lib.rs` para confirmar el comportamiento actual de `render`, `lookup` y `get`.
- [x] Documentar el comportamiento actual en `openspec/specs/secrets/spec.md` si falta.
- [x] Añadir tests de caracterización si no existen y confirmar GREEN con `cargo test`.

## Fase 1 — SDD (OpenSpec)

- [x] Crear change `openspec/changes/cli-safety-flags/`.
- [x] Redactar `proposal.md`.
- [x] Redactar `specs/cli/spec.md` con contratos y escenarios Given/When/Then.
- [x] Redactar `tasks.md`.
- [x] Aprobación del usuario sobre la spec.

## Fase 2 — TDD (Red-Green-Refactor)

### Tarea 2.1 — `render --out`

- [x] RED: test de integración que invoca `crypta render --out <tmp>` y verifica que el fichero contiene el render y que stdout está vacío.
- [x] RED: test de error si el fichero no se puede escribir (ruta inexistente / sin permisos).
- [x] GREEN: implementar el flag `--out` en el subcomando `render`.
- [x] REFACTOR: `cargo fmt`, `cargo clippy -- -D warnings`.

### Tarea 2.2 — `lookup --mask` / `get --mask`

- [x] RED: test que verifica que la salida no contiene el valor y sí metadatos (existe, longitud).
- [x] RED: test que verifica que `--mask` no escribe el valor en stderr ni en logs.
- [x] GREEN: implementar `--mask` en `lookup` y `get`.
- [x] REFACTOR: `cargo fmt`, `cargo clippy -- -D warnings`.

### Tarea 2.3 — Verificación global

- [x] `just check` en verde (fmt + clippy + test).
- [x] `cargo test` sin regresiones.

## Fase 3 — Skill y documentación

- [x] Crear la skill `crypta` con el flujo: `list` → `render --out` → `lookup --mask`; prohibido `lookup` a stdout en contexto de agente.
- [x] Crear `docs/cli-safety-flags.md` (qué es, ficheros implicados, cómo funciona, variables/config, decisiones, follow-ups).

## Fase 4 — Cierre

- [ ] PR a `develop` con commits gitmoji.
- [ ] Archivar el change con `openspec archive cli-safety-flags`.
- [ ] Renombrar este plan a `plans/PLAN-001-ARCHIVED.md`.
- [x] Actualizar `HANDOFF.md`.

## Criterios de aceptación

- [x] `crypta render --out <path>` escribe el fichero y no imprime el contenido.
- [x] `crypta lookup --mask` no revela el valor en ningún caso.
- [x] Tests nuevos en verde y sin regresiones.
- [x] `just check` en verde.
- [x] Skill `crypta` disponible.
- [x] `docs/cli-safety-flags.md` creado.

## Riesgos y decisiones

- **MCP descartado:** no impone la frontera de seguridad porque el agente puede invocar `crypta` directamente. Se reevaluará solo si hace falta acceso desde clientes sin shell.
- **`--mask` no debe filtrar longitud exacta si el usuario lo considera sensible:** se documentará como metadato y se podrá añadir `--mask=len` en el futuro.
- **Compatibilidad:** los flags son opt-in; el comportamiento por defecto no cambia.
