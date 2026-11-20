# cli Specification

## Purpose
TBD - created by archiving change cli-safety-flags. Update Purpose after archive.

## Requirements

### Requirement: render escribe a fichero sin volcar a stdout

El comando `crypta render [FILE] --out <PATH>` SHALL escribir el resultado renderizado en `<PATH>` y SHALL dejar stdout vacío. El valor renderizado no SHALL aparecer en stderr.

#### Scenario: render a fichero

- **GIVEN** una plantilla válida y una ruta de destino escribible
- **WHEN** el usuario ejecuta `crypta render --out /tmp/out.txt`
- **THEN** el fichero `/tmp/out.txt` contiene el resultado renderizado
- **AND** stdout está vacío
- **AND** el valor no aparece en stderr

#### Scenario: error de escritura

- **GIVEN** una ruta de destino en un directorio inexistente o sin permisos
- **WHEN** el usuario ejecuta `crypta render --out <ruta>`
- **THEN** el comando falla con código de salida distinto de 0
- **AND** el mensaje de error está en español y no contiene el valor renderizado

### Requirement: lookup y get enmascaran el valor

Los comandos `crypta lookup [KEY] --mask` y `crypta get [KEY] --mask` SHALL mostrar únicamente metadatos (existencia y longitud) y SHALL NOT revelar el valor ni en stdout ni en stderr. `get --mask` SHALL NOT copiar el valor al portapapeles.

#### Scenario: lookup enmascarado

- **GIVEN** una clave existente con valor `secreto123`
- **WHEN** el usuario ejecuta `crypta lookup KEY --mask`
- **THEN** la salida indica que la clave existe y su longitud
- **AND** la salida no contiene `secreto123`
- **AND** stderr no contiene `secreto123`

#### Scenario: get enmascarado

- **GIVEN** una clave existente
- **WHEN** el usuario ejecuta `crypta get KEY --mask`
- **THEN** el valor no se copia al portapapeles ni se imprime
- **AND** la salida indica que la clave existe

#### Scenario: clave inexistente con --mask

- **GIVEN** una clave que no existe
- **WHEN** el usuario ejecuta `crypta lookup KEY --mask`
- **THEN** el comando falla con código de salida distinto de 0
- **AND** el mensaje de error está en español

### Requirement: compatibilidad con el comportamiento por defecto

Sin los flags nuevos, `crypta render`, `crypta lookup KEY` y `crypta get KEY` SHALL mantener el comportamiento actual sin cambios.

#### Scenario: invocación sin flags

- **GIVEN** los comandos sin flags nuevos
- **WHEN** el usuario ejecuta `crypta render`, `crypta lookup KEY` o `crypta get KEY`
- **THEN** el comportamiento es idéntico al actual
