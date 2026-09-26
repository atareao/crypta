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
