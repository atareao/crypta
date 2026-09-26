# Proposal

## Why

Los usuarios de crypta necesitan generar archivos de configuración, scripts y otros documentos a partir de plantillas Jinja2, rellenando las variables con los secretos almacenados en el vault. Actualmente no hay forma de integrar crypta con herramientas de templating — hay que extraer secretos manualmente uno a uno. Esta feature aporta una integración directa: la plantilla se pasa a crypta, y las variables se resuelven automáticamente desde el store cifrado.

## What Changes

- **Nuevo subcomando `render`** (alias `r`): Lee una plantilla Jinja2 (archivo o stdin), resuelve sus variables contra el store de secretos de crypta, y escribe el resultado por stdout.
- **Nuevo crate `minijinja`** como dependencia para el motor de templates.
- **Nuevo módulo `templates`** en `src/templates.rs` con la función pública `render_template`.
- **Modificación de `lib.rs`**: exportar el nuevo módulo `pub mod templates`.
- **Modificación de `main.rs`**: añadir el subcomando `Render` al enum `Commands` y su gestión en `run_command`.

## Capabilities

### New Capabilities

- `template-render`: Resolución de plantillas Jinja2 contra el store de secretos de crypta, con soporte para lectura desde archivo o stdin, y manejo de errores para variables faltantes o archivo inexistente.

### Modified Capabilities

<!-- No existing capabilities are modified — this is purely additive. -->

## Impact

- **Dependencias**: Se añade `minijinja = "6.0"` a `Cargo.toml`.
- **Código**: Nuevo archivo `src/templates.rs`. Modificaciones en `src/lib.rs` y `src/main.rs`.
- **CLI**: Nuevo subcomando: `crypta render [FILE]`.
- **Tests**: Nuevos tests unitarios en `src/templates.rs` y test de integración CLI.
- **Rendimiento**: minijinja es ligero y sin dependencias extra; impacto despreciable.