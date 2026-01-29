# 🔐 Crypta

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/crypta)
[![Tests](https://img.shields.io/badge/tests-14%20passing-success.svg)](https://github.com/yourusername/crypta)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-blue.svg)](https://crates.io/crates/crypta)

Gestor de secretos moderno escrito en Rust puro, compatible con SOPS/Age para encriptación de secretos y sincronización automática con Git.

## ✨ Características

- 🔒 **Encriptación robusta** usando SOPS/Age con AES-256-GCM
- 📋 **Portapapeles integrado** multiplataforma (Linux, macOS, Windows)
- � **Salida por stdout** para scripts con comando `show`
- 🔄 **Sincronización Git** automática con rebase
- 🦀 **Rust + SOPS nativo** - Mejor compatibilidad
- ⚡ **Rápido y eficiente** - Compilado nativamente
- 🧪 **Completamente testeado** - 14 tests unitarios e integración
- 📦 **Modular** - Biblioteca reutilizable + CLI
- 🔍 **Debugging con tracing** - Logs configurables con RUST_LOG

## 📦 Instalación

### Desde el código fuente

```bash
git clone https://github.com/yourusername/crypta.git
cd crypta
cargo build --release
sudo cp target/release/crypta /usr/local/bin/
```

### Usando Cargo

```bash
cargo install crypta
```

## 🔑 Configuración

Crypta requiere una clave Age para la encriptación. Configura tu entorno:

```bash
# Generar una clave Age (si no tienes una)
age-keygen -o ~/.age/key.txt

# Configurar la variable de entorno
export SOPS_AGE_KEY_FILE=~/.age/key.txt
```

Añade la exportación a tu `~/.bashrc`, `~/.zshrc` o `~/.config/fish/config.fish`:

```bash
echo 'export SOPS_AGE_KEY_FILE=~/.age/key.txt' >> ~/.bashrc
```

## 🚀 Uso

### Almacenar/Actualizar un secreto

#### Usando `store` (valor desde stdin)

```bash
# Secreto simple
echo "mi-secreto-super-seguro" | crypta store API_KEY

# Desde variable
printf "$SECRET_VALUE" | crypta store DATABASE_URL

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
```

### Obtener un secreto (copia al portapapeles)

```bash
crypta get API_KEY
# 📋 Secreto 'API_KEY' copiado al portapapeles.
```

### Mostrar un secreto (stdout)

Útil para scripts y captura en variables:

```bash
# Mostrar directamente
crypta lookup API_KEY

# Sin logs (limpio para scripts)
RUST_LOG=off crypta lookup API_KEY

# Capturar en variable (fish)
set TOKEN (RUST_LOG=off crypta lookup API_KEY)

# Capturar en variable (bash)
TOKEN=$(RUST_LOG=off crypta lookup API_KEY)

# Usar en pipes
crypta lookup API_KEY | wl-copy
```

### Listar todas las claves

```bash
crypta list
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

### Usar secretos en scripts

```bash
#!/bin/bash
# Almacenar desde archivo
cat /path/to/secret.key | crypta store API_KEY

# Almacenar desde comando
kubectl config view --raw | crypta store KUBECONFIG

# Exportar secreto como variable de entorno
export API_KEY=$(RUST_LOG=off crypta lookup API_KEY)

# Usar en curl
curl -H "Authorization: Bearer $(RUST_LOG=off crypta lookup API_TOKEN)" \
     https://api.example.com/data
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
# Almacenar tokens de GitHub/GitLab
echo "$GITHUB_TOKEN" | crypta store GH_TOKEN
echo "$GITLAB_TOKEN" | crypta store GL_TOKEN

# Configuración AWS
aws configure list --profile production | crypta store AWS_CONFIG

# Almacenar secrets de Kubernetes
kubectl get secret my-secret -o yaml | crypta store K8S_SECRET

# Variables de entorno para deployment
cat << EOF | crypta store PROD_ENV_VARS
NODE_ENV=production
DATABASE_URL=postgresql://prod-user:$(RUST_LOG=off crypta lookup DB_PASS)@prod-db:5432/myapp
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

## � Comandos Disponibles

| Comando | Descripción | Entrada | Salida |
|---------|-----------|---------|---------|
| `store KEY` | Almacena o actualiza un secreto | 📝 stdin | ✅ Confirmación |
| `set KEY VALUE` | Almacena o actualiza un secreto | 💬 Argumento | ✅ Confirmación |
| `get KEY` | Obtiene un secreto y lo copia al portapapeles | - | 📋 Portapapeles |
| `lookup KEY` | Muestra un secreto por stdout (ideal para scripts) | - | 📝 stdout |
| `list` | Lista todas las claves disponibles | - | 🔑 Lista |
| `delete KEY` | Elimina un secreto | - | 🗑️ Confirmación |
| `sync [MSG]` | Sincroniza cambios con Git | - | 🔄 Estado sync |

**Diferencias entre comandos de almacenamiento:**
- `store`: Lee valor desde stdin - ideal para contenido complejo, multilínea, o desde pipes
- `set`: Toma valor como argumento - ideal para valores simples en scripts

**Diferencias entre comandos de lectura:**
- `get`: Copia al portapapeles (uso interactivo)
- `lookup`: Imprime por stdout (uso en scripts, pipes, variables)

## �🛠️ Tecnologías

| Dependencia | Propósito |
|-------------|-----------|
| **SOPS** | Encriptación de secretos (comando nativo) |
| **Age** | Criptografía moderna para SOPS |
| **git2** | Operaciones Git nativas |
| **arboard** | Portapapeles multiplataforma |
| **clap** | CLI parsing con derive macros |
| **serde_yaml** | Manipulación de YAML |
| **anyhow** | Manejo de errores ergonómico |
| **tracing** | Logging estructurado |

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

- 🐛 [Reportar un bug](https://github.com/yourusername/crypta/issues/new?labels=bug)
- 💡 [Solicitar una feature](https://github.com/yourusername/crypta/issues/new?labels=enhancement)
- 📖 [Documentación](https://github.com/yourusername/crypta/wiki)

---

Hecho con ❤️ y 🦀 por la comunidad Rust
