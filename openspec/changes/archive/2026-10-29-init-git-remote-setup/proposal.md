# Propuesta: Configuración de repositorio remoto en `crypta init`

## Problema

Actualmente `crypta init` configura el directorio `~/.secrets/`, genera la clave Age y crea `.sops.yaml`, pero **no configura git**.
El usuario tiene que hacer manualmente:

```bash
cd ~/.secrets
git init
git remote add origin <url>
git add . && git commit -m "Initial" && git push -u origin main
```

Esto es tedioso y propenso a errores. Además, `crypta sync` ya asume que `~/.secrets/` es un repo git con `origin` configurado,
pero `init` no lo prepara — hay un gap de UX evidente.

## Solución propuesta

Modificar `crypta init` para que, al final del proceso, pregunte interactivamente al usuario si quiere configurar
un repositorio remoto. Si responde que sí:

1. Inicializa git en `~/.secrets/`
2. Crea el commit inicial con los archivos generados (`.sops.yaml`, `sops/age/key.txt`)
3. Pregunta la URL del remoto
4. Configura `origin` con esa URL
5. Hace `push -u origin main`

Todo con mensajes claros en español, coherentes con el estilo actual de crypta.

## Impacto

- **Scope**: solo `src/secrets.rs` (función `init`) y `src/main.rs` (flag `--remote` opcional)
- **Breaking changes**: ninguno — `init` sin flag sigue funcionando exactamente igual
- **Dependencias**: no se requieren nuevas — `git2` ya está en el proyecto

## Contratos

### Rust (función modificada)

```rust
pub fn init(secrets_dir: &str, secrets_file: &str, remote_url: Option<&str>) -> Result<()>
```

- Si `remote_url` es `Some(url)`, configura git automáticamente (no interactivo, ideal para scripting)
- Si `remote_url` es `None`, pregunta interactivamente al usuario

### CLI (nuevo flag opcional)

```
crypta init [--remote <URL>]

Flags:
  -r, --remote <URL>    URL del repositorio git remoto (opcional)
```

## Escenarios

### Happy path — interactivo
1. Usuario ejecuta `crypta init`
2. Se genera clave Age, `.sops.yaml`
3. crypta pregunta: "¿Quieres configurar un repositorio remoto para sincronizar? (s/N)"
4. Usuario responde "s"
5. crypta pregunta: "URL del repositorio:"
6. Usuario introduce `git@github.com:user/secrets.git`
7. crypta hace `git init`, commit, remote add, push
8. Mensaje final: "✅ Repositorio remoto configurado. Usa 'crypta sync' para sincronizar."

### Happy path — flag --remote
1. Usuario ejecuta `crypta init --remote git@github.com:user/secrets.git`
2. Flujo completo sin interacción (ideal para scripting/automation)

### Happy path — sin remoto (comportamiento actual)
1. Usuario ejecuta `crypta init`
2. Responde "n" a la pregunta del remoto (o pulsa Enter)
3. Comportamiento idéntico al actual — no se toca git

### Error — push falla
1. Usuario configura remoto
2. Commit y remote OK, pero push falla (red, autenticación)
3. crypta muestra error claro pero **no revierte** el repo local
4. Mensaje: "⚠️ Push falló. El repo local está configurado. Ejecuta 'crypta sync' cuando tengas conexión."
