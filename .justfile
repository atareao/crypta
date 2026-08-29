user     := "atareao"
name     := `basename ${PWD}`
version  := `vampus show`

# ──────────────────────────────────────────────
# 🚀 GitFlow Workflow
# ──────────────────────────────────────────────

# Lista de comandos disponibles
list:
    @just --list

# Iniciar una nueva feature
feature-start name:
    git flow feature start {{name}}

# Publicar una feature en el remoto
feature-publish name:
    git flow feature publish {{name}}

# Finalizar una feature (merge a develop)
feature-finish name:
    git flow feature finish {{name}}

# Iniciar una release
release-start version:
    git flow release start {{version}}

# Publicar una release
release-publish version:
    git flow release publish {{version}}

# Finalizar una release (merge a main + develop, tag)
release-finish version:
    git flow release finish {{version}}

# Iniciar un hotfix
hotfix-start name:
    git flow hotfix start {{name}}

# Finalizar un hotfix
hotfix-finish name:
    git flow hotfix finish {{name}}

# ──────────────────────────────────────────────
# 📦 Release (legacy, simplificado)
# ──────────────────────────────────────────────

install:
    cargo install --path .

upgrade:
    @vampus upgrade --patch

# Release completa: desde develop → release → main + tag
release:
    @just release-start {{version}}
    @just release-finish {{version}}

# ──────────────────────────────────────────────
# 🧪 Calidad
# ──────────────────────────────────────────────

# Formatear código
fmt:
    cargo fmt

# Lint (clippy con deny warnings)
lint:
    cargo clippy -- -D warnings

# Tests completos
test:
    cargo test

# Todo: fmt + lint + test
check:
    @just fmt
    @just lint
    @just test

# ──────────────────────────────────────────────
# 🛠️ Construcción
# ──────────────────────────────────────────────

build:
    cargo build

build-release:
    cargo build --release