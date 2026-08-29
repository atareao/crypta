# GitHub Copilot Instructions for crypta

## Project Overview

**crypta** is a modern secrets manager written in Rust. It uses SOPS/Age for encryption and Git for automatic synchronization. It stores encrypted secrets in a YAML file at `~/.secrets/secrets.yml`.

## Project Structure

```
crypta/
├── src/
│   ├── main.rs          # CLI entry point with clap argument parsing
│   ├── lib.rs           # Module declarations
│   ├── secrets.rs       # Secret operations (add, get, show, list, remove, init, password)
│   └── git.rs           # Git operations (sync with pull --rebase + push)
├── tests/
│   ├── secrets_tests.rs       # Tests for YAML manipulation
│   ├── git_tests.rs           # Tests for Git operations
│   ├── integration_tests.rs   # CLI integration tests
│   └── test_password.rs       # Password generation tests
├── Cargo.toml
└── README.md
```

## Architecture

### Library (src/lib.rs)

- Declares two public modules: `secrets` and `git`
- Used as a library (`crypta`) and as a binary

### Secrets Module (src/secrets.rs)

- **Public functions:**
  - `add(secrets_dir, secrets_file, key, value)` - Add or update a secret (decrypts, modifies, re-encrypts with SOPS)
  - `get(secrets_file, key)` - Get a secret and copy to clipboard via `arboard`
  - `show(secrets_file, key)` - Print a secret to stdout
  - `list(secrets_file)` - List all secret keys
  - `remove(secrets_file, key)` - Delete a secret
  - `init(secrets_dir, secrets_file)` - Initialize secrets directory, generate Age key, create SOPS config
  - `password_string(length, special)` - Generate a random password string
  - `generate_password(length, special)` - Generate and print a random password
- **Private helpers:**
  - `verify_sops_installed()` - Check that `sops` binary is available
  - `encrypt_with_sops(yaml_content, secrets_file)` - Encrypt YAML content via `sops -e` stdin pipe
  - `extract_public_key_from_file(key_file_path)` - Extract Age public key from key file
  - `extract_public_key_from_output(output)` - Extract Age public key from age-keygen output

### Git Module (src/git.rs)

- **Public functions:**
  - `sync(secrets_dir, message)` - Full sync: commit local changes, pull --rebase, push
- **Private helpers:**
  - `pull_rebase(repo)` - Fetch + rebase via libgit2, with fallback to system `git`
  - `push(repo)` - Push via libgit2, with fallback to system `git`
- Supports `CRYPTA_USE_SYSTEM_GIT` env var to bypass libgit2 entirely

### Binary (src/main.rs)

- Uses `clap` with derive macros for CLI
- Subcommands: `store` (s), `set` (se), `get` (g), `lookup` (l), `list` (ls), `delete` (rm), `init` (i), `sync` (sy), `password` (pwd)
- Key resolution: parameter or `SECRET_ID` environment variable
- Delegates to library functions
- Error handling with `anyhow`

## Key Dependencies

- **clap 4.6**: CLI argument parsing with derive features
- **serde_yaml 0.9**: YAML serialization/deserialization
- **anyhow 1.0**: Error handling
- **git2 0.21**: libgit2 bindings for Git operations
- **arboard 3.6**: Cross-platform clipboard
- **tracing 0.1 + tracing-subscriber 0.3**: Structured logging with env-filter
- **rand 0.10**: Random password generation
- **tempfile 3.27** (dev): Temporary directories for tests

## Code Style & Conventions

### General Rust

- Follow standard Rust conventions (`cargo fmt`)
- Use `cargo clippy` for linting
- Edition 2021
- Minimum Rust version: 1.70+

### Error Handling

- Use `anyhow::Result` and `anyhow::Context` for all fallible functions
- Error messages in Spanish to match CLI
- Use `eprintln!` for errors in binary
- Exit code 1 for errors

### CLI Arguments

- Use clap's derive macros with subcommands (`#[derive(Subcommand)]`)
- All commands have short aliases
- Document with doc comments (shown in --help)
- Spanish descriptions for Spanish-speaking users

### Logging

- Use `tracing` for structured logging
- Default level: `error` (configurable via `RUST_LOG`)
- Use `info!` for operation start/end
- Use `debug!` for detailed diagnostics

## Testing Guidelines

### Unit Tests

- Inline `#[cfg(test)] mod tests` in source files
- Separate test files in `tests/` directory for integration tests
- Use `tempfile::TempDir` for filesystem tests
- Test all public functions

### Integration Tests (tests/integration_tests.rs)

- Use `Command` to execute the binary
- Test via `env!("CARGO_BIN_EXE_crypta")`
- Verify exit codes
- Check stdout and stderr
- Test all CLI subcommands and their short aliases

## Common Patterns

### Adding a New Secret Operation

1. Add function to `src/secrets.rs`
2. Add CLI command variant in `src/main.rs`
3. Add match arm in `run_command()`
4. Add tests in `tests/secrets_tests.rs`
5. Add integration tests in `tests/integration_tests.rs`

### Encryption Flow

1. Decrypt existing file with `sops -d` (or start with empty YAML)
2. Parse YAML, modify the mapping
3. Serialize back to YAML string
4. Pipe to `sops -e` via stdin for encryption
5. Write encrypted bytes to file

### Git Sync Flow

1. Check for local changes via `repo.statuses()`
2. If changes: add all, write tree, commit
3. Fetch from origin + rebase (libgit2, fallback to system git)
4. Push to origin (libgit2, fallback to system git)

## Important Notes

- **Spanish messages**: Error messages and CLI help in Spanish
- **Exit codes**: Use `std::process::exit(1)` for errors
- **No hardcoded secrets**: All secrets are encrypted with SOPS/Age
- **No temp files**: Encryption uses stdin pipe, never writes plaintext to disk
- **SSH auth**: libgit2 handles SSH via ssh-agent or key files; fallback to system git if libgit2 fails

## Common Tasks

### Run the app
```bash
cargo run -- --help
```

### Run tests
```bash
cargo test
```

### Format code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

### Build release
```bash
cargo build --release
```