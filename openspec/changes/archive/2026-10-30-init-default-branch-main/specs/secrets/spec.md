# Spec Delta

## Purpose

Corregir la rama inicial del repositorio git creado por `crypta init`: debe ser `main`, no `master` (default de libgit2).

## MODIFIED Requirements

### Requirement: Init configura repositorio git remoto

La función `init` DEBE soportar la configuración opcional de un repositorio git remoto en `~/.secrets/`, con la rama inicial `main`.

#### Scenario: Init con --remote flag configura git automáticamente

- **GIVEN** una función `init(secrets_dir, secrets_file, remote_url)` 
- **WHEN** se invoca con `remote_url = Some("git@github.com:user/secrets.git")`
- **THEN** DEBE inicializar un repositorio git en `secrets_dir`
- **AND** DEBE crear la rama inicial `main` (nunca `master`)
- **AND** DEBE añadir todos los archivos (`secrets_file`, `.sops.yaml`, `sops/age/key.txt`) al índice
- **AND** DEBE crear un commit inicial con mensaje "Initial secrets" sobre `main`
- **AND** DEBE configurar `remote origin` con la URL proporcionada
- **AND** DEBE hacer push a `origin main`
- **AND** DEBE imprimir "✅ Repositorio remoto configurado."

#### Scenario: Init sin --remote pregunta interactivamente

- **GIVEN** una función `init(secrets_dir, secrets_file, None)`
- **WHEN** se ejecuta y completa la generación de clave y `.sops.yaml`
- **THEN** DEBE preguntar al usuario: "¿Quieres configurar un repositorio remoto para sincronizar? (s/N)"
- **AND** si el usuario responde "s" o "S", DEBE solicitar la URL del repositorio
- **AND** DEBE ejecutar los mismos pasos que con `--remote` (init, add, commit, remote add, push) con rama `main`

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
- **AND** la rama local DEBE seguir siendo `main`

#### Scenario: Init con --remote deja la rama activa en main

- **GIVEN** un repositorio recién inicializado por `init_repo_with_remote`
- **WHEN** se inspecciona con `repo.head().shorthand()`
- **THEN** la rama activa DEBE ser `main`
- **AND** el commit inicial DEBE estar sobre `refs/heads/main`
- **AND** NO DEBE existir la rama `master`
