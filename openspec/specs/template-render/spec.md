# template-render Specification

## Purpose
Permite generar archivos de configuración, scripts y documentos a partir de plantillas Jinja2, resolviendo automáticamente las variables contra los secretos almacenados en el vault de crypta.

## Requirements

### Requirement: Renderizar plantilla desde archivo

El sistema DEBE leer una plantilla Jinja2 desde un archivo del sistema de archivos, resolver todas las variables usando el store de secretos de crypta (desencriptado vía sops), y escribir el resultado renderizado por stdout.

#### Scenario: Plantilla con variables resueltas desde secretos

- **WHEN** el usuario ejecuta `crypta render template.j2`
- **AND** el archivo `template.j2` contiene `Host: {{ DB_HOST }}; Password: {{ DB_PASSWORD }}`
- **AND** el store de secretos contiene `DB_HOST: localhost` y `DB_PASSWORD: s3cret`
- **THEN** la salida por stdout DEBE ser `Host: localhost; Password: s3cret`

#### Scenario: Plantilla sin variables

- **WHEN** el usuario ejecuta `crypta render template.j2`
- **AND** el archivo `template.j2` contiene solo texto literal sin expresiones Jinja2
- **THEN** la salida por stdout DEBE ser idéntica al contenido del archivo

### Requirement: Renderizar plantilla desde stdin

El sistema DEBE leer la plantilla desde stdin cuando no se proporciona un argumento de archivo.

#### Scenario: Plantilla desde stdin

- **WHEN** el usuario ejecuta `echo '{{ HOST }}:{{ PORT }}' | crypta render`
- **AND** el store de secretos contiene `HOST: app.example.com` y `PORT: 8080`
- **THEN** la salida por stdout DEBE ser `app.example.com:8080`

### Requirement: Error en variable faltante

El sistema DEBE fallar con un mensaje de error claro cuando una variable referenciada en la plantilla no existe en el store de secretos.

#### Scenario: Variable faltante en plantilla

- **WHEN** el usuario ejecuta `crypta render template.j2`
- **AND** el archivo `template.j2` contiene `{{ MISSING_VAR }}`
- **AND** el store de secretos NO contiene la clave `MISSING_VAR`
- **THEN** el sistema DEBE mostrar un error indicando que la variable `MISSING_VAR` no se encuentra en el store
- **AND** el sistema DEBE terminar con código de salida distinto de cero

### Requirement: Error en archivo inexistente

El sistema DEBE fallar con un mensaje de error cuando el archivo de plantilla especificado no existe.

#### Scenario: Archivo de plantilla no encontrado

- **WHEN** el usuario ejecuta `crypta render no-existe.j2`
- **AND** el archivo `no-existe.j2` no existe
- **THEN** el sistema DEBE mostrar un error indicando que el archivo no existe
- **AND** el sistema DEBE terminar con código de salida distinto de cero

### Requirement: Renderización con sintaxis de filtros básicos

El sistema DEBE soportar la sintaxis estándar de Jinja2 incluyendo filtros, siempre que las variables referenciadas existan en el store.

#### Scenario: Plantilla con filtro `default`

- **WHEN** el usuario ejecuta `crypta render template.j2`
- **AND** el archivo `template.j2` contiene `{{ TIMEOUT | default("30") }}`
- **AND** el store de secretos NO contiene `TIMEOUT`
- **THEN** la salida por stdout DEBE ser `30` (usa el valor por defecto del filtro, sin error)
