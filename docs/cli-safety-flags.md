# cli-safety-flags

## Qué es

Flags de seguridad añadidos al CLI de Crypta para que agentes y automatizaciones puedan operar con secretos sin exponer valores en claro en stdout, en el contexto del modelo o en los logs. La frontera de seguridad es el propio CLI: no se añade un servidor MCP porque el agente puede invocar el binario directamente y saltarse cualquier capa intermedia.

## Ficheros implicados

| Fichero | Rol |
|---|---|
| `src/main.rs` | Definición de los flags `--out` y `--mask` en los subcomandos `render`, `lookup` y `get`. |
| `src/secrets.rs` | Lectura/desencriptado del store y helpers de enmascarado. |
| `src/templates.rs` | Renderizado de plantillas y escritura segura a fichero. |
| `tests/cli_safety_flags_tests.rs` | Tests de integración de los nuevos flags. |
| `tests/cli_characterization_tests.rs` | Caracterización del comportamiento previo (sin flags). |
| `tests/common/mod.rs` | Utilidades compartidas por los tests de integración. |

## Cómo funciona

| Flag | Comportamiento |
|---|---|
| `render --out <PATH>` | Escribe el resultado renderizado a `<PATH>` con permisos `0600`. Rechaza symlinks, hardlinks al store, ficheros no regulares y el propio `secrets.yml`. Deja stdout vacío. |
| `lookup --mask` / `get --mask` | Imprime `🔒 Secreto '<KEY>' existe (<N> caracteres).` con la longitud en caracteres (no en bytes). Nunca muestra el valor. `get --mask` no toca el portapapeles. |
| Compatibilidad | Sin flags, el comportamiento es idéntico al anterior: `render` escribe a stdout, `lookup` imprime el valor y `get` lo copia al portapapeles. |
| `tracing` | Ahora escribe a stderr (antes lo hacía a stdout) para no contaminar la salida estándar que consumen los agentes. |

### `render --out <PATH>`

1. Renderiza la plantilla en memoria.
2. Abre el destino sin `truncate` y sin seguir symlinks (`O_NOFOLLOW` donde está disponible).
3. Verifica el descriptor abierto (`fstat`): debe ser un fichero regular, no debe ser un hardlink al store y no debe ser `secrets.yml`.
4. Solo entonces trunca y escribe con permisos `0600`.
5. No imprime nada por stdout.

### `lookup --mask` / `get --mask`

- `lookup --mask` lee la clave del store, calcula `value.chars().count()` y emite el mensaje de metadatos.
- `get --mask` hace lo mismo pero **no** invoca `arboard`: el portapapeles queda intacto.
- El valor nunca se serializa a stdout, stderr ni logs.

## Variables/config

| Variable | Efecto |
|---|---|
| `RUST_LOG` | Nivel de `tracing`; la salida va a **stderr**. |
| `SECRET_ID` | Sustituye el argumento `KEY` en cualquier comando. |
| `SOPS_AGE_KEY_FILE` | Ruta a la clave privada Age que usa `sops`. |

## Cómo habilitar/desplegar

No hay migraciones ni configuración nueva. Los flags son opt-in y el comportamiento por defecto no cambia.

```sh
just build-release
```

El binario resultante queda en `target/release/crypta`.

## Decisiones

### Enmascarar con longitud y no con hash

Un hash del secreto es un identificador estable: permite correlacionar el mismo secreto entre ejecuciones y, si el espacio de claves es pequeño, atacarlo offline. La longitud en caracteres basta para verificar que la clave existe y que tiene el tamaño esperado, sin crear un fingerprint reutilizable. Si en el futuro se necesita más metadato, se puede añadir `--mask=len` o similares sin romper compatibilidad.

### Rechazar `/dev/stdout` y process substitution

`/dev/stdout` es un symlink al terminal o a la tubería del proceso, y `<(cmd)` crea un fd anónimo. Aceptarlos permitiría que el secreto acabara en la terminal o en otro proceso, esquivando las comprobaciones de permisos y de tipo de fichero. El flag `--out` está pensado para materializar secretos en un fichero regular con `0600`; cualquier otra cosa se rechaza.

### `O_NOFOLLOW` por arquitectura

`O_NOFOLLOW` se aplica en el `open` para cerrar la ventana TOCTOU entre la comprobación del path y la apertura. No es portable: Windows no tiene equivalente y algunas variantes Unix cambian su semántica. Por eso el código lo usa bajo `cfg` en las plataformas que lo soportan y cae a una comprobación explícita con `symlink_metadata` en el resto.

### Abrir sin `truncate` y verificar el fd (TOCTOU)

Abrir con `O_TRUNC` truncaría un fichero existente antes de poder verificar que es seguro. En su lugar se abre sin truncar, se hace `fstat` sobre el descriptor y se comprueba que es un fichero regular, que no es un hardlink al store y que no es `secrets.yml`. Solo después se trunca y se escribe. Así la verificación se hace sobre el objeto real que se va a escribir, no sobre el path.

## Follow-ups

- **Enum `AccessAction` en vez de `&str`:** sustituir las cadenas de acción (`"lookup"`, `"get"`, `"render"`, etc.) por un enum tipado para evitar errores de comparación y facilitar el análisis estático.
- **Unificar la desencriptación duplicada:** `add`, `read_secret_value` y `templates` desencriptan el store por su cuenta. Consolidar en un único helper para reducir superficie y garantizar el mismo tratamiento de errores.
- **Servidor MCP descartado:** se reevaluará solo si aparecen clientes sin acceso a shell que necesiten invocar Crypta de forma nativa.
