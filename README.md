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

### Añadir/Actualizar un secreto

```bash
crypta add API_KEY "mi-secreto-super-seguro"
crypta add DATABASE_URL "postgresql://user:pass@localhost/db"
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
crypta show API_KEY

# Sin logs (limpio para scripts)
RUST_LOG=off crypta show API_KEY

# Capturar en variable (fish)
set TOKEN (RUST_LOG=off crypta show API_KEY)

# Capturar en variable (bash)
TOKEN=$(RUST_LOG=off crypta show API_KEY)

# Usar en pipes
crypta show API_KEY | wl-copy
```

### Listar todas las claves

```bash
crypta ls
# 🔑 Claves en /home/user/.secrets/secrets.yml:
# - API_KEY
# - DATABASE_URL
```

### Eliminar un secreto

```bash
crypta rm API_KEY
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
# Exportar secreto como variable de entorno
export API_KEY=$(RUST_LOG=off crypta show API_KEY)

# Usar en curl
curl -H "Authorization: Bearer $(RUST_LOG=off crypta show API_TOKEN)" \
     https://api.example.com/data
```

### Integración con Docker

```bash
# Pasar secreto a Docker
docker run -e DB_PASS=$(RUST_LOG=off crypta show DB_PASSWORD) myapp

# En docker-compose (usar .env file generado)
RUST_LOG=off crypta show DATABASE_URL > .env
```

### Fish shell

```fish
# Función para cargar secretos
function load_secret
    set -gx $argv[1] (RUST_LOG=off crypta show $argv[2])
end

# Uso
load_secret API_KEY my_api_key
echo $API_KEY
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

| Comando | Descripción | Salida |
|---------|-------------|--------|
| `add KEY VALUE` | Añade o actualiza un secreto | ✅ Confirmación |
| `get KEY` | Obtiene un secreto y lo copia al portapapeles | 📋 Al portapapeles |
| `show KEY` | Muestra un secreto por stdout (ideal para scripts) | 📝 stdout |
| `ls` | Lista todas las claves disponibles | 🔑 Lista |
| `rm KEY` | Elimina un secreto | 🗑️ Confirmación |
| `sync [MSG]` | Sincroniza cambios con Git | 🔄 Estado sync |

**Diferencia entre `get` y `show`:**
- `get`: Copia al portapapeles (uso interactivo)
- `show`: Imprime por stdout (uso en scripts, pipes, variables)

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

- [ ] Soporte para múltiples backends de encriptación (AWS KMS, GCP KMS)
- [ ] Exportación/importación de secretos
- [ ] Interfaz TUI interactiva
- [ ] Integración con gestores de contraseñas
- [ ] Soporte para .env files
- [ ] Auto-completado para shells (bash/zsh/fish)

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
