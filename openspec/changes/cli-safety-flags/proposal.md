# Proposal — cli-safety-flags

## Intención

Añadir al CLI de Crypta dos primitivas de seguridad para uso por agentes y humanos: renderizar a fichero sin volcar a stdout (`render --out`) y consultar metadatos sin revelar el valor (`lookup --mask`, `get --mask`). Acompañar con una skill `crypta` que documente el flujo seguro.

## Motivación

Un agente con acceso a shell puede ejecutar `crypta lookup` y volcar secretos en el contexto del modelo. Un servidor MCP no resuelve el problema porque el agente puede saltárselo llamando al binario. La solución correcta es que el CLI ofrezca operaciones que no devuelvan plaintext.

## Contratos

### CLI

```
crypta render [FILE] [--out <PATH>]
crypta lookup [KEY] [--mask]
crypta get [KEY] [--mask]
```

- `--out <PATH>`: ruta de destino. Si se omite, se mantiene el comportamiento actual (stdout).
- `--mask`: booleano. Si se pasa, la salida no contiene el valor.

### Rust (orientativo)

```rust
pub fn render_to_file(template: &str, out: &Path) -> anyhow::Result<()>;
pub fn mask_value(value: &str) -> String;
```

## Alcance

- `render --out <path>`: escribir el resultado a fichero, stdout vacío.
- `lookup --mask` y `get --mask`: salida sin valor, con metadatos.
- Skill `crypta` y documentación en `docs/cli-safety-flags.md`.

## Fuera de alcance

- Servidor MCP.
- Cambios en el formato de cifrado o en el flujo SOPS/age.
- Cambios en el comportamiento por defecto de los comandos.

## Impacto

- `src/main.rs`: nuevos flags en los subcomandos `render`, `lookup` y `get`.
- `src/lib.rs` / `src/secrets.rs`: funciones de render a fichero y de enmascarado.
- Tests de integración en `tests/`.
- Nueva skill y documentación.
- Sin migraciones ni cambios de configuración.
