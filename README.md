# 🔐 Crypta

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/atareao/crypta)
[![Tests](https://img.shields.io/badge/tests-passing-success.svg)](https://github.com/atareao/crypta)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.7-blue.svg)](https://crates.io/crates/crypta)

Gestor de secretos moderno escrito en Rust puro, compatible con SOPS/Age para encriptación de secretos y sincronización automática con Git.

## ✨ Características

- 🔒 **Encriptación robusta** usando SOPS/Age con AES-256-GCM
- 📋 **Portapapeles integrado** multiplataforma (Linux, macOS, Windows)
- 📝 **Salida por stdout** para scripts con comando `lookup`
- 🔄 **Sincronización Git** automática con rebase
- ⚡ **Setup completamente automatizado** - `init` configura todo por ti
- 🦀 **Rust + SOPS nativo** - Mejor compatibilidad
- ⚡ **Rápido y eficiente** - Compilado nativamente
- 🧪 **Completamente testeado** - Tests unitarios e integración
- 📦 **Modular** - Biblioteca reutilizable + CLI
- 🔍 **Debugging con tracing** - Logs configurables con RUST_LOG

## 📦 Instalación

### Desde el código fuente

```bash
git clone https://github.com/atareao/crypta.git
cd crypta
cargo build --release
sudo cp target/release/crypta /usr/local/bin/
```

### Usando Cargo

```bash
cargo install crypta
```

## 🔑 Configuración

### Configuración completamente automatizada ✨

Crypta incluye un comando de inicialización que configura **todo automáticamente**:

```bash
# Un solo comando configura todo: directorio, clave Age y SOPS
crypta init
# O usando el alias corto:
crypta i
```

Esto crea **automáticamente**:

- `~/.secrets/` - Directorio para secretos
- `~/.secrets/sops/age/key.txt` - Clave Age generada automáticamente
- `~/.secrets/.sops.yaml` - Configuración SOPS con la clave correcta

Solo necesitas configurar la variable de entorno una vez:

```bash
# Añadir a tu ~/.bashrc, ~/.zshrc, o ~/.config/fish/config.fish
export SOPS_AGE_KEY_FILE=~/.secrets/sops/age/key.txt
```

¡Y listo! Ya puedes usar crypta inmediatamente.

## 🚀 Uso

### Configuración inicial (solo una vez)

```bash
# 1. Inicializar crypta (totalmente automatizado)
crypta init

# 2. Configurar variable de entorno (sigue las instrucciones mostradas)
export SOPS_AGE_KEY_FILE=~/.secrets/sops/age/key.txt

# 3. ¡Listo! Crear tu primer secreto
crypta set --key TEST --value "mi-primer-secreto"
```

### Almacenar/Actualizar un secreto

#### Usando `store` (valor desde stdin)

```bash
# Secreto simple
echo "mi-secreto-super-seguro" | crypta store API_KEY
# O usando comando corto:
echo "mi-secreto-super-seguro" | crypta s API_KEY

# Desde variable
printf "$SECRET_VALUE" | crypta store DATABASE_URL
# O usando comando corto:
printf "$SECRET_VALUE" | crypta s DATABASE_URL

# Contenido multilínea (ej: claves SSH)
cat ~/.ssh/id_rsa | crypta store SSH_PRIVATE_KEY

# JSON o configuración compleja
cat << EOF | crypta store DB_CONFIG
{
  "host": "localhost",
  "port": 5432,
  "user": "admin",
  "password": "secret123"
}
EOF
```

#### Usando `set` (valor como argumento)

```bash
# Sintaxis tradicional - ideal para scripts simples
crypta set API_KEY "mi-secreto-super-seguro"
crypta set DATABASE_URL "postgresql://user:pass@localhost/db"

# O usando comandos cortos:
crypta se API_KEY "mi-secreto-super-seguro"
crypta se DATABASE_URL "postgresql://user:pass@localhost/db"
```

### Obtener un secreto (copia al portapapeles)

```bash
crypta get API_KEY
# 📋 Secreto 'API_KEY' copiado al portapapeles.

# O usando comando corto:
crypta g API_KEY
```

### Mostrar un secreto (stdout)

Útil para scripts y captura en variables:

```bash
# Mostrar directamente
crypta lookup API_KEY
# O usando comando corto:
crypta l API_KEY

# Sin logs (limpio para scripts)
RUST_LOG=off crypta lookup API_KEY
# O usando comando corto:
RUST_LOG=off crypta l API_KEY

# Capturar en variable (fish)
set TOKEN (RUST_LOG=off crypta lookup API_KEY)
# O usando comando corto:
set TOKEN (RUST_LOG=off crypta l API_KEY)

# Capturar en variable (bash)
TOKEN=$(RUST_LOG=off crypta lookup API_KEY)
# O usando comando corto:
TOKEN=$(RUST_LOG=off crypta l API_KEY)

# Usar en pipes
crypta lookup API_KEY | wl-copy
crypta l API_KEY | wl-copy  # comando corto
```

### Listar todas las claves

```bash
crypta list
# O usando comando corto:
crypta ls

# 🔑 Claves en /home/user/.secrets/secrets.yml:
# - API_KEY
# - DATABASE_URL
```

### Eliminar un secreto

```bash
crypta delete API_KEY
# 🗑️ Secreto 'API_KEY' eliminado.
```

### Sincronizar con Git

```bash
crypta sync
# 🔄 Sincronizando con el remoto...
# 🚀 Sincronización completada.

# Con mensaje personalizado
crypta sync "Añadido nuevo secreto de producción"
```

## 💡 Ejemplos Prácticos

### Configuración inicial (completamente automatizada)

```bash
# 🎩 Configuración mágica en 30 segundos
crypta init

# Crypta muestra algo como:
# 🔑 Generando nueva clave Age: ~/.secrets/sops/age/key.txt
# 📄 Archivo de configuración creado: ~/.secrets/.sops.yaml
# ✅ Inicialización completada exitosamente
# 💡 Para usar crypta, añade esto a tu shell:
#    export SOPS_AGE_KEY_FILE=~/.secrets/sops/age/key.txt

# Configurar variable de entorno (solo una vez)
export SOPS_AGE_KEY_FILE=~/.secrets/sops/age/key.txt
echo 'export SOPS_AGE_KEY_FILE=~/.secrets/sops/age/key.txt' >> ~/.bashrc

# 🎉 ¡Listo! Probar con tu primer secreto
crypta set --key SALUDO --value "Hola desde crypta!"
crypta lookup SALUDO
# Hola desde crypta!
```

### Usar secretos en scripts

```bash
#!/bin/bash
# Almacenar desde archivo
cat /path/to/secret.key | crypta store API_KEY
# O usando variable de entorno
SECRET_ID=API_KEY cat /path/to/secret.key | crypta store

# Almacenar desde comando
kubectl config view --raw | crypta store KUBECONFIG

# Usar variable de entorno para workflows automatizados
SECRET_ID=DATABASE_PASSWORD echo "super-secret-db-pass" | crypta store

# Exportar secreto como variable de entorno
export API_KEY=$(RUST_LOG=off crypta lookup API_KEY)
# O usando SECRET_ID
export API_KEY=$(SECRET_ID=API_KEY RUST_LOG=off crypta lookup)

# Usar en curl
curl -H "Authorization: Bearer $(RUST_LOG=off crypta lookup API_TOKEN)" \
     https://api.example.com/data
```

### Workflows con SECRET_ID

```bash
#!/bin/bash
# Script que procesa múltiples secretos
SECRETS=("API_KEY" "DB_PASS" "SSL_CERT")

for secret in "${SECRETS[@]}"; do
    echo "Procesando $secret..."
    SECRET_ID="$secret"

    # Verificar si existe
    if SECRET_ID="$secret" crypta lookup >/dev/null 2>&1; then
        echo "✅ $secret existe"
    else
        echo "⚠️ $secret no encontrado"
        # Generar nuevo secreto
        openssl rand -base64 32 | SECRET_ID="$secret" crypta store
        echo "🆕 $secret creado"
    fi
done
```

### Integración con Docker

```bash
# Pasar secreto a Docker
docker run -e DB_PASS=$(RUST_LOG=off crypta lookup DB_PASSWORD) myapp

# En docker-compose (usar .env file generado)
RUST_LOG=off crypta lookup DATABASE_URL > .env

# Almacenar configuración Docker
docker-compose config | crypta store DOCKER_COMPOSE_CONFIG
```

### Fish shell

```fish
# Función para cargar secretos
function load_secret
    set -gx $argv[1] (RUST_LOG=off crypta lookup $argv[2])
end

# Almacenar desde clipboard
wl-paste | crypta store CLIPBOARD_SECRET

# Generar y almacenar password
openssl rand -base64 32 | crypta store RANDOM_PASSWORD

# Uso
load_secret API_KEY my_api_key
echo $API_KEY
```

## 🔥 Ejemplos Avanzados

### Setup automatizado para equipos

```bash
#!/bin/bash
# Script de configuración completamente automatizado para nuevos desarrolladores

echo "🚀 Configurando crypta para el equipo..."

# Inicializar crypta (genera clave Age y configura SOPS automáticamente)
crypta init

# Obtener la ruta de la clave generada
AGE_KEY_FILE=$(find ~/.secrets -name "key.txt" -type f | head -1)

if [ -n "$AGE_KEY_FILE" ]; then
    echo "⚙️  Configurando variable de entorno..."

    # Detectar shell y configurar apropiadamente
    if [ -n "$BASH_VERSION" ]; then
        echo "export SOPS_AGE_KEY_FILE=$AGE_KEY_FILE" >> ~/.bashrc
        echo "✅ Configuración añadida a ~/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        echo "export SOPS_AGE_KEY_FILE=$AGE_KEY_FILE" >> ~/.zshrc
        echo "✅ Configuración añadida a ~/.zshrc"
    else
        echo "export SOPS_AGE_KEY_FILE=$AGE_KEY_FILE" >> ~/.profile
        echo "✅ Configuración añadida a ~/.profile"
    fi

    # Configurar para la sesión actual
    export SOPS_AGE_KEY_FILE="$AGE_KEY_FILE"

    echo "🗝 Probando configuración..."
    crypta set --key TEAM_WELCOME --value "Bienvenido al equipo!"

    if crypta lookup TEAM_WELCOME >/dev/null 2>&1; then
        echo "🎉 ¡Configuración exitosa!"
        echo "💡 Para usar crypta en nuevas terminales, ejecuta: source ~/.bashrc"
        crypta rm TEAM_WELCOME  # Limpiar secreto de prueba
    else
        echo "⚠️  Algo salió mal. Reinicia la terminal e intenta de nuevo."
    fi
fi

# Configurar Git hooks para sincronización automática (si está en un repo)
if [ -d .git ]; then
    echo "⚙️  Configurando hooks Git..."
    cat << 'EOF' > .git/hooks/post-commit
#!/bin/bash
if [ -f ~/.secrets/secrets.yml ]; then
    crypta sync "Auto-sync after commit $(git rev-parse --short HEAD)"
fi
EOF
    chmod +x .git/hooks/post-commit
    echo "✅ Hook Git configurado"
fi

echo "🎉 ¡Setup completado! Crypta está listo para usar."
# Migrar desde archivos .env a crypta

# Inicializar crypta si no está configurado
if [ ! -d ~/.secrets ]; then
    crypta init
    echo "⚠️  Configura tu clave Age antes de continuar"
    exit 1
fi

# Migrar desde .env
if [ -f .env ]; then
    echo "📦 Migrando desde .env..."
    while IFS='=' read -r key value; do
        if [[ $key =~ ^[A-Z_][A-Z0-9_]*$ ]] && [ ! -z "$value" ]; then
            echo "Migrando $key..."
            echo "$value" | crypta store "$key"
        fi
    done < .env

    # Backup del archivo original
    mv .env .env.bak
    echo "✅ Migración completada. Backup en .env.bak"
fi
```

### Gestión de Certificados SSL

```bash
# Almacenar certificados desde archivos
cat /etc/ssl/certs/server.crt | crypta store SSL_CERT
cat /etc/ssl/private/server.key | crypta store SSL_PRIVATE_KEY

# Almacenar certificado desde comando
openssl req -x509 -newkey rsa:4096 -keyout - -out - -days 365 -nodes \
    -subj "/CN=example.com" | crypta store SELF_SIGNED_CERT
```

### DevOps y CI/CD

```bash
# Almacenar tokens usando variables de entorno (ideal para CI/CD)
SECRET_ID=GITHUB_TOKEN echo "$GITHUB_TOKEN" | crypta store
SECRET_ID=GITLAB_TOKEN echo "$GITLAB_TOKEN" | crypta store

# Configuración AWS
aws configure list --profile production | crypta store AWS_CONFIG

# Almacenar secrets de Kubernetes usando SECRET_ID
kubectl get secret my-secret -o yaml | SECRET_ID=K8S_SECRET crypta store

# Pipeline de CI/CD automatizado
#!/bin/bash
DEPLOY_SECRETS=("API_KEY" "DB_PASSWORD" "JWT_SECRET")

for secret_name in "${DEPLOY_SECRETS[@]}"; do
    if [ ! -z "${!secret_name}" ]; then
        echo "Almacenando $secret_name desde variable de entorno..."
        SECRET_ID="$secret_name" echo "${!secret_name}" | crypta store
    fi
done

# Variables de entorno para deployment
cat << EOF | SECRET_ID=PROD_ENV_VARS crypta store
NODE_ENV=production
DATABASE_URL=postgresql://prod-user:$(SECRET_ID=DB_PASS RUST_LOG=off crypta lookup)@prod-db:5432/myapp
REDIS_URL=redis://prod-redis:6379
API_BASE_URL=https://api.example.com
EOF
```

### Gestión de Bases de Datos

```bash
# Connection strings completas
echo "postgresql://user:$(openssl rand -hex 16)@localhost:5432/mydb" | crypta store DATABASE_URL

# Scripts SQL sensibles
cat sensitive_migration.sql | crypta store SQL_MIGRATION_V2

# Configuración MongoDB
cat << EOF | crypta store MONGO_CONFIG
{
  "hosts": ["mongo1:27017", "mongo2:27017", "mongo3:27017"],
  "replicaSet": "rs0",
  "username": "admin",
  "password": "$(openssl rand -base64 24)"
}
EOF
```

### Integración con Password Managers

```bash
# Desde 1Password CLI
op item get "API Key" --field password | crypta store OP_API_KEY

# Desde Bitwarden CLI
bw get password "Database Password" | crypta store BW_DB_PASS

# Desde pass (Unix password manager)
pass show services/api-key | crypta store PASS_API_KEY
```

### Automatización y Scripts

```bash
#!/bin/bash
# Script para rotar contraseñas
rotate_password() {
    local key_name=$1
    local new_pass=$(openssl rand -base64 32)

    # Almacenar nueva contraseña
    echo "$new_pass" | crypta store "$key_name"

    # Sincronizar cambios
    crypta sync "Rotated password for $key_name"

    echo "✅ Password rotated for $key_name"
}

# Uso
rotate_password "API_KEY"
rotate_password "DB_PASSWORD"
```

### Backup y Migración

```bash
# Exportar todos los secretos (para backup)
for key in $(crypta list | grep -o '[A-Z_][A-Z0-9_]*'); do
    echo "=== $key ===" >> backup.txt
    RUST_LOG=off crypta lookup "$key" >> backup.txt
    echo "" >> backup.txt
done

# Migrar desde otro gestor de secretos
jq -r '.secrets[] | "\(.key)\n\(.value)"' old_secrets.json | \
while read key && read value; do
    echo "$value" | crypta store "$key"
done
```

## 🏗️ Arquitectura

```
crypta/
├── src/
│   ├── lib.rs          # API pública y type aliases
│   ├── main.rs         # CLI con clap
│   ├── secrets.rs      # Operaciones con secretos encriptados
│   └── git.rs          # Operaciones Git (sync, pull, push)
├── tests/
│   ├── secrets_tests.rs      # Tests de manipulación YAML
│   ├── git_tests.rs          # Tests de operaciones Git
│   └── integration_tests.rs  # Tests del CLI
└── Cargo.toml
```

## 📋 Comandos Disponibles

| Comando                         | Alias | Descripción                                                               | Key                      | Entrada  | Salida            |
| ------------------------------- | ----- | ------------------------------------------------------------------------- | ------------------------ | -------- | ----------------- |
| `init`                          | `i`   | Inicializa **automáticamente** directorio, clave Age y configuración SOPS | -                        | -        | 🧠 Setup completo |
| `store [KEY]`                   | `s`   | Almacena o actualiza un secreto                                           | Parámetro o `$SECRET_ID` | 📝 stdin | ✅ Confirmación   |
| `set --key [KEY] --value VALUE` | `se`  | Almacena o actualiza un secreto                                           | `--key` o `$SECRET_ID`   | 💬 Flag  | ✅ Confirmación   |
| `get [KEY]`                     | `g`   | Obtiene un secreto y lo copia al portapapeles                             | Parámetro o `$SECRET_ID` | -        | 📋 Portapapeles   |
| `lookup [KEY]`                  | `l`   | Muestra un secreto por stdout (ideal para scripts)                        | Parámetro o `$SECRET_ID` | -        | 📝 stdout         |
| `list`                          | `ls`  | Lista todas las claves disponibles                                        | -                        | -        | 🔑 Lista          |
| `delete [KEY]`                  | `rm`  | Elimina un secreto                                                        | Parámetro o `$SECRET_ID` | -        | 🗑️ Confirmación   |
| `sync [MSG]`                    | `sy`  | Sincroniza cambios con Git                                                | -                        | -        | 🔄 Estado sync    |

### 🔑 Gestión de Claves

Todos los comandos que requieren una clave pueden obtenerla de dos formas:

1. **Parámetro directo**: `crypta get API_KEY`
2. **Variable de entorno**: `SECRET_ID=API_KEY crypta get`

```bash
# Métodos equivalentes:
crypta get API_KEY
SECRET_ID=API_KEY crypta get

# Store desde stdin
echo "secreto" | crypta store API_KEY
SECRET_ID=API_KEY echo "secreto" | crypta store

# Set con flags
crypta set --key API_KEY --value "secreto"
SECRET_ID=API_KEY crypta set --value "secreto"
```

### 🚀 Comandos Cortos (Aliases)

Todos los comandos tienen versiones cortas para mayor rapidez:

```bash
# Comandos largos
crypta init
crypta store API_KEY < secret.txt
crypta set API_KEY "value"
crypta get API_KEY
crypta lookup API_KEY
crypta list
crypta delete API_KEY
crypta sync "mensaje"

# Comandos cortos (equivalentes)
crypta i
crypta s API_KEY < secret.txt
crypta se API_KEY "value"
crypta g API_KEY
crypta l API_KEY
crypta ls
crypta rm API_KEY
crypta sy "mensaje"
```

**Diferencias entre comandos de almacenamiento:**

- `store`: Lee valor desde stdin - ideal para contenido complejo, multilínea, o desde pipes
- `set`: Toma valor como argumento - ideal para valores simples en scripts

**Diferencias entre comandos de lectura:**

- `get`: Copia al portapapeles (uso interactivo)
- `lookup`: Imprime por stdout (uso en scripts, pipes, variables)

## �🛠️ Tecnologías

| Dependencia    | Propósito                                 |
| -------------- | ----------------------------------------- |
| **SOPS**       | Encriptación de secretos (comando nativo) |
| **Age**        | Criptografía moderna para SOPS            |
| **git2**       | Operaciones Git nativas                   |
| **arboard**    | Portapapeles multiplataforma              |
| **clap**       | CLI parsing con derive macros             |
| **serde_yaml** | Manipulación de YAML                      |
| **anyhow**     | Manejo de errores ergonómico              |
| **tracing**    | Logging estructurado                      |

## 🧪 Tests

```bash
# Ejecutar todos los tests
cargo test

# Tests con output detallado
cargo test -- --nocapture

# Solo tests unitarios
cargo test --lib

# Solo tests de integración
cargo test --test '*'
```

**Cobertura actual:** 14 tests (6 secrets + 5 git + 3 integración)

## 📊 Benchmarks

```bash
# Añadir secreto: ~50ms
# Leer secreto: ~30ms
# Sincronizar: ~200ms (depende de red)
```

## 🔒 Seguridad

- ✅ Encriptación AES-256-GCM
- ✅ Hash SHA-512 para integridad
- ✅ Claves Age con curvas elípticas Curve25519
- ✅ Los secretos nunca se escriben en texto plano al disco
- ✅ Limpieza automática de memoria (zeroize)

## 🤝 Contribuir

Las contribuciones son bienvenidas! Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

### Directrices

- Todos los tests deben pasar: `cargo test`
- Código formateado: `cargo fmt`
- Sin warnings de clippy: `cargo clippy`
- Añadir tests para nuevas funcionalidades

## 📝 Roadmap

### En desarrollo

- [x] Comando `store` con entrada stdin para contenido complejo
- [x] Comando `set` como alias tradicional para compatibilidad
- [x] Soporte para contenido multilínea y binario
- [x] Comando `init` para inicialización automática

### Próximas características

- [ ] Soporte para múltiples backends de encriptación (AWS KMS, GCP KMS)
- [ ] Comando `import` para migrar desde otros gestores (.env, JSON, YAML)
- [ ] Comando `export` para backup en diferentes formatos
- [ ] Interfaz TUI interactiva con navegación y búsqueda
- [ ] Auto-completado para shells (bash/zsh/fish)
- [ ] Plantillas de secretos para configuraciones comunes
- [ ] Integración nativa con gestores de contraseñas (1Password, Bitwarden)
- [ ] Soporte para etiquetas y categorización de secretos
- [ ] Auditoría y logs de acceso a secretos
- [ ] Rotación automática de contraseñas con webhooks

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## 🙏 Agradecimientos

- [SOPS](https://github.com/getsops/sops) - Secrets OPerationS para encriptación
- [Age](https://github.com/FiloSottile/age) - Sistema de encriptación simple y seguro

## 💬 Soporte

¿Encontraste un bug? ¿Tienes una sugerencia?

- 🐛 [Reportar un bug](https://github.com/atareao/crypta/issues/new?labels=bug)
- 💡 [Solicitar una feature](https://github.com/atareao/crypta/issues/new?labels=enhancement)
- 📖 [Documentación](https://github.com/atareao/crypta/wiki)

---

Hecho con ❤️ y 🦀 por la comunidad Rust
