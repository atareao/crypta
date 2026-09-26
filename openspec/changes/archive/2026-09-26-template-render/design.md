# Design

## Context

Véase `proposal.md` — Why y What Changes. El usuario tiene secretos cifrados con SOPS/Age en `~/.secrets/secrets.yml`. Quiere pasar plantillas Jinja2 que referencien esos secretos como variables y obtener el resultado sin exponer los valores en disco.

minijinja es un motor de templates Rust que implementa la sintaxis de Jinja2. No depende de Python ni de librerías externas. Soporta variables, filtros (`default`, `upper`, etc.), y bloques básicos.

## Goals / Non-Goals

**Goals:**
- Nuevo subcomando `crypta render [FILE]` que lee una plantilla, resuelve variables desde el store, y emite el resultado por stdout.
- La resolución de variables debe ser _case-sensitive_ — las claves del store se usan tal cual.
- Los errores de variable faltante deben ser explícitos: el mensaje indica qué variable no se encontró.

**Non-Goals:**
- No se implementa escritura a archivo directa (el usuario redirige stdout si lo necesita: `crypta render t.j2 > out.conf`).
- No se implementan includes/extends de Jinja2 en esta iteración (minijinja los soporta, pero crypta no gestiona rutas de búsqueda relativas).
- No se implementa modo interactivo ni preview.
- No se implementa lógica compleja (bucles `for`, condicionales `if/else`) aunque minijinja los soporte — el foco es resolución de variables.

## Decisions

### `minijinja` frente a `tera` o Jinja2 vía subproceso

- **Decisión**: `minijinja`.
- **Razón**: Es el crate de templating Rust más ligero (sin dependencias del sistema), con API sencilla y buena cobertura de la sintaxis Jinja2. `tera` es más pesado y está orientado a contexts con `Serialize` — para nuestro caso (variables planas clave→valor) `minijinja` es más directo. Llamar a `python3 -m jinja2` sería frágil (dependencia externa, parsing extra).
- **Alternativa**: `tera` — descartado por ser más complejo de lo necesario.

### Subcomando `render` frente a `template`

- **Decisión**: `render`, alias `r`.
- **Razón**: Más corto y descriptivo. El alias de una letra es consistente con otros comandos (`s`, `g`, `l`, `i`).

### Estrategia de resolución de variables

- Decryptar el secrets file completo con `sops -d` (reutilizando la función `export_secrets` o lógica similar).
- Parsear el YAML a un `BTreeMap<String, String>` (claves planas).
- Pasar ese mapa como contexto a minijinja.
- Si la plantilla referencia una clave que no existe en el mapa, minijinja lanza un error `UndefinedError` que capturamos y convertimos en `anyhow::Error`.

### Manejo de errores

- `UndefinedError` de minijinja: se muestra `❌ Error: La variable 'X' no existe en el store de secretos`.
- `TemplateNotFound` (no aplica — leemos el archivo antes de pasarlo a minijinja).
- Archivo inexistente: error estándar de IO convertido con `anyhow::Context`.
- Store vacío o inexistente: se reutiliza la lógica existente de `verify_sops_installed` y el error de archivo no encontrado.

## Risks / Trade-offs

- **[Rendimiento]** Decryptar todo el store en cada `render` puede ser lento si hay cientos de secretos. → Mitigación: minijinja es rápido y la decryptación vía sops es el cuello de botella, no el rendering. Si es necesario en el futuro se puede cachear.
- **[Seguridad]** El contenido renderizado se emite por stdout y queda en el historial del terminal. → Mitigación: es responsabilidad del usuario redirigir a archivo o pipe según necesite. Crypta nunca escribe el resultado a disco por defecto.
- **[Filtros avanzados]** minijinja soporta filtros pero no la biblioteca completa de Jinja2. → Mitigación: si un filtro no está disponible, el error de minijinja se muestra directamente. Para los casos de uso típicos (`default`, `upper`, `lower`) es suficiente.

## Open Questions

- Ninguna.