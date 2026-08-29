# AGENTS.md — crypta

## Quick start

```sh
just check          # fmt + clippy -D warnings + test (also runs as pre-commit hook)
just build-release  # cargo build --release
```

## GitFlow

| Branch | Purpose |
|---|---|
| `main` | Production releases only |
| `develop` | Integration branch |
| `feature/*` | New work (branch from `develop`) |
| `release/*` | Release prep (branch from `develop`) |
| `hotfix/*` | Urgent patches (branch from `main`) |

```sh
just feature-start name    # git flow feature start name
just feature-finish name   # merges to develop, deletes feature branch
just release-start v0.x.y  # git flow release start v0.x.y
just release-finish v0.x.y # merges to main + develop, tags
just hotfix-start name
just hotfix-finish name
```

## Pre-commit hook (enforced)

Runs automatically on every `git commit`:
1. `cargo fmt --check`
2. `cargo clippy -- -D warnings`
3. `cargo test`

## CI (`.github/workflows/ci.yml`)

Runs on push/PR to `main`: build → test → clippy (`-D warnings`) → `cargo fmt --check`.

Release workflow (`.github/workflows/release.yml`) only triggers on GitHub release creation — builds binary, renames to `crypta-linux-x86_64`, uploads with SHA256.

## Architecture

- **Library** (`src/lib.rs`): re-exports `pub mod secrets` and `pub mod git`
- **Binary** (`src/main.rs`): clap CLI with 9 subcommands, delegates to library
- **Secrets** (`src/secrets.rs`): decrypts `~/.secrets/secrets.yml` with `sops -d`, modifies YAML, re-encrypts via `sops -e` stdin pipe (never writes plaintext to disk)
- **Git** (`src/git.rs`): commit → pull --rebase → push via libgit2, falls back to system `git`

## CLI commands

| Command | Alias | Input | Output |
|---|---|---|---|
| `store [KEY]` | `s` | stdin | Encrypted YAML |
| `set --key K --value V` | `se` | `--value` flag | Encrypted YAML |
| `get [KEY]` | `g` | — | Clipboard (`arboard`) |
| `lookup [KEY]` | `l` | — | stdout |
| `list` | `ls` | — | Key list |
| `delete [KEY]` | `rm` | — | Removes from YAML |
| `init` | `i` | — | Creates `~/.secrets/`, Age key, SOPS config |
| `sync [MSG]` | `sy` | — | Git commit + pull --rebase + push |
| `password [-l N] [--special]` | `pwd` | — | Random password to stdout |

## Key env vars

| Variable | Required | Effect |
|---|---|---|
| `SECRET_ID` | No | Replaces the `KEY` argument in any command |
| `SOPS_AGE_KEY_FILE` | Yes (runtime) | Path to Age private key for `sops` |
| `RUST_LOG` | No | Tracing level: `error` (default), `info`, `debug`, `off` |
| `CRYPTA_USE_SYSTEM_GIT` | No | Set to `1` or `true` to bypass libgit2 and use system `git` |

## Runtime requirements

- `sops` binary must be installed and on `$PATH`
- `age-keygen` binary for `crypta init`
- SSH agent running or SSH keys in `~/.ssh/` for git sync with remotes

## Testing

```sh
cargo test                          # all tests
cargo test --test git_tests         # git integration tests only
cargo test --test test_password     # password generation tests
```

Integration tests use `env!("CARGO_BIN_EXE_crypta")` to reference the compiled binary. Tests requiring `sops` are not present — all existing tests are offline (YAML parsing, git operations on temp repos, password generation).

## Conventions

- Error messages and CLI help in **Spanish**
- All fallible functions return `anyhow::Result`
- `password_string(length, special)` returns `Result<String>` (testable); `generate_password` prints it (CLI-facing)
- Version managed by `vampus` (see `.vampus.yml`); run `just upgrade` for patch bumps

## Reference

For detailed architecture docs (module API, encryption flow, git sync flow), see `.github/copilot-instructions.md`.