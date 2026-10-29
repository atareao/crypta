# secrets Specification

## Purpose
Asegurar que `encrypt_with_sops` pueda encriptar contenido YAML vía stdin con `sops 3.13+`, donde `--filename-override` es necesario para que `path_regex` en `.sops.yaml` matchee correctamente.

## Requirements

### Requirement: Encrypt_with_sops invoca sops con --filename-override

La función `encrypt_with_sops` DEBE pasar `--filename-override` y el nombre base del archivo de secretos (ej. `secrets.yml`) como argumentos a `sops -e` cuando lee el contenido desde stdin.

#### Scenario: encrypt_with_sops incluye --filename-override en el comando sops

- **GIVEN** una función `encrypt_with_sops(yaml_content, secrets_file)`
- **WHEN** se invoca con `yaml_content = "key: value"` y `secrets_file = "/home/user/.secrets/secrets.yml"`
- **THEN** el comando `sops` DEBE incluir los argumentos `"--filename-override"` y `"secrets.yml"`
- **AND** el comando DEBE incluir `"--input-type"` `"yaml"` y `"--output-type"` `"yaml"`
- **AND** el comando DEBE leer el contenido desde stdin (sin argumento posicional de archivo)

#### Scenario: encrypt_with_sops funciona con sops 3.13+ real

- **GIVEN** un archivo `.sops.yaml` configurado con `path_regex: .*\.yml$` y una clave Age
- **AND** la variable `SOPS_AGE_KEY_FILE` apunta a una clave Age válida
- **WHEN** se invoca `encrypt_with_sops("test_key: test_value", "/tmp/.secrets/secrets.yml")`
- **THEN** DEBE retornar contenido encriptado válido (formato `ENC[AES256_GCM,...]`)
- **AND** NO DEBE fallar con error de "no matching creation rules" o "no file specified"

### Requirement: Init configura repositorio git remoto

La función `init` DEBE soportar la configuración opcional de un repositorio git remoto en `~/.secrets/`.

#### Scenario: Init con --remote flag configura git automáticamente

- **GIVEN** una función `init(secrets_dir, secrets_file, remote_url)` 
- **WHEN** se invoca con `remote_url = Some("git@github.com:user/secrets.git")`
- **THEN** DEBE inicializar un repositorio git en `secrets_dir`
- **AND** DEBE añadir todos los archivos (`secrets_file`, `.sops.yaml`, `sops/age/key.txt`) al índice
- **AND** DEBE crear un commit inicial con mensaje "Initial secrets"
- **AND** DEBE configurar `remote origin` con la URL proporcionada
- **AND** DEBE hacer push a `origin main`
- **AND** DEBE imprimir "✅ Repositorio remoto configurado."

#### Scenario: Init sin --remote pregunta interactivamente

- **GIVEN** una función `init(secrets_dir, secrets_file, None)`
- **WHEN** se ejecuta y completa la generación de clave y `.sops.yaml`
- **THEN** DEBE preguntar al usuario: "¿Quieres configurar un repositorio remoto para sincronizar? (s/N)"
- **AND** si el usuario responde "s" o "S", DEBE solicitar la URL del repositorio
- **AND** DEBE ejecutar los mismos pasos que con `--remote` (init, add, commit, remote add, push)

#### Scenario: Init sin remoto mantiene comportamiento actual

- **GIVEN** una función `init(secrets_dir, secrets_file, None)`
- **WHEN** el usuario responde "n" (o Enter) a la pregunta del remoto
- **THEN** DEBE finalizar sin tocar git
- **AND** DEBE imprimir el mensaje de finalización actual

#### Scenario: Init --remote con push fallido no revierte repo local

- **GIVEN** un directorio `secrets_dir` sin conexión a internet
- **WHEN** `init` hace `git push` y falla
- **THEN** DEBE mostrar un error con el mensaje "⚠️ Push falló. El repo local está configurado."
- **AND** NO DEBE revertir el repo local ni eliminar el remote
