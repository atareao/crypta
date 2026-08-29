# GitHub Copilot Instructions for crypta

## Project Overview

**crypta** is a command-line Lorem Ipsum generator written in Rust. It generates random Lorem Ipsum text in various formats (words, sentences, paragraphs) with customizable lengths.

## Project Structure

```
crypta/
├── src/
│   ├── main.rs          # CLI entry point with clap argument parsing
│   └── lib.rs           # Core generation logic (public API)
├── tests/
│   ├── test_crypta.rs           # Unit tests for core functions
│   ├── test_paragraph_length.rs  # Unit tests for paragraph length customization
│   └── integration_tests.rs      # Integration tests for the binary
├── Cargo.toml
└── README.md
```

## Architecture

### Library (src/lib.rs)

- **Public API** with generic RNG support
- Core functions:
  - `generate_words<R: Rng>(rng: &mut R, count: usize) -> String`
  - `generate_sentences<R: Rng>(rng: &mut R, count: usize, min_words: usize, max_words: usize) -> String`
  - `generate_paragraphs<R: Rng>(rng: &mut R, count: usize, min_words: usize, max_words: usize) -> String`
  - `generate_content<R: Rng>(rng: &mut R, content_type: &str, count: usize, paragraph_length: Option<(usize, usize)>) -> Result<String, String>`

### Binary (src/main.rs)

- Uses `clap` with derive macros for CLI with subcommands
- Subcommands: `words` (alias `w`), `sentences` (alias `s`), `paragraphs` (alias `p`)
- Default subcommand: `paragraphs` (when no subcommand specified)
- Global options: `--count`, `--min`, `--max`
- Delegates to library functions
- Handles error display and exit codes

## Key Dependencies

- **lipsum 0.9**: Lorem Ipsum generation with RNG support
- **clap 4.5**: CLI argument parsing with derive features
- **rand 0.8**: Random number generation
- **rand_chacha 0.3** (dev): Deterministic RNG for tests

## Code Style & Conventions

### General Rust

- Follow standard Rust conventions (`cargo fmt`)
- Use `cargo clippy` for linting
- Edition 2021
- Minimum Rust version: 1.70+

### Function Naming

- Use snake_case for functions
- Descriptive names: `generate_*` for generation functions
- Generic RNG parameter always `<R: Rng>`

### Error Handling

- Use `Result<String, String>` for generation functions
- Error messages in Spanish to match CLI
- Use `eprintln!` for errors in binary
- Exit code 1 for errors

### CLI Arguments

- Use clap's derive macros with subcommands (`#[derive(Subcommand)]`)
- Subcommands: `words` (alias `w`), `sentences` (alias `s`), `paragraphs` (alias `p`)
- Default subcommand: `paragraphs` when none specified
- Global options with `global = true`: `-c/--count`, `--min`, `--max`
- Document with doc comments (shown in --help)
- Defaults for all optional arguments
- Spanish descriptions for Spanish-speaking users
- `--min` and `--max` apply to both sentences and paragraphs:
  - Default: min=40, max=80
  - If only min specified: max = min * 2
  - If only max specified: min = max / 2
  - If both specified: use given values
  - **Validations:**
    - min cannot be negative (show error and exit)
    - max cannot be negative (show error and exit)
    - If min=0 and max=0: use defaults (40, 80)

### Randomness

- Always use `&mut R: Rng` generic parameter in library
- Use `thread_rng()` in binary
- Use `ChaCha8Rng::seed_from_u64()` in tests for determinism
- Variable ranges with `rng.gen_range(min..=max)`

### Text Generation

- Use `lipsum::lipsum_words_with_rng(&mut *rng, count)` for words
- Join multiple items with `Vec::join()`
- Paragraph separator: `"\n\n"`
- Sentence separator: `" "`
- Always generate random content (never static)

## Testing Guidelines

### Unit Tests (tests/test_crypta.rs, tests/test_paragraph_length.rs)

- Use `ChaCha8Rng` with fixed seeds for deterministic tests
- Test all public functions
- Test abbreviations (`w`, `s`, `p`)
- Test error cases
- Test randomness vs determinism
- Test custom paragraph lengths

### Integration Tests (tests/integration_tests.rs)

- Use `Command` to execute the binary
- Test via `env!("CARGO_BIN_EXE_crypta")`
- Verify exit codes
- Check stdout and stderr
- Test all CLI argument combinations
- Test default behavior
- Verify actual output format

### Test Structure

```rust
#[test]
fn test_name() {
    // Setup
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    
    // Execute
    let result = generate_function(&mut rng, params);
    
    // Assert
    assert!(result.is_ok());
    // More assertions
}
```

## Common Patterns

### Adding a New Generation Type

1. Add function to `src/lib.rs` with generic RNG
2. Update `generate_content()` to handle new type
3. Add CLI argument if needed in `src/main.rs`
4. Add unit tests in `tests/test_crypta.rs`
5. Add integration tests in `tests/integration_tests.rs`

### Adding a CLI Option

1. Add field to `Args` struct in `src/main.rs`
2. Add `#[arg(...)]` attribute with docs
3. Use the option in main logic
4. Update README with new option
5. Add integration tests

### Testing New Features

- **Always** add both unit and integration tests
- Use deterministic RNG for reproducible unit tests
- Test edge cases (empty, very large, invalid input)
- Verify error messages

## Important Notes

- **Never use static text**: Always generate random content with RNG
- **Use `&mut *rng`**: When passing RNG in loops to avoid move errors
- **Spanish messages**: Error messages and CLI help in Spanish
- **Ranges inclusive**: Use `min..=max` for inclusive ranges
- **Word count tolerance**: Allow ±10% variance in tests due to lipsum's behavior
- **Exit codes**: Use `std::process::exit(1)` for errors

## Feature Flags

Currently none. Keep it simple.

## Performance Considerations

- No need for optimization; generation is already fast
- Prefer readability over micro-optimizations
- String allocation is acceptable for this use case

## When Adding New Features

1. **Think library-first**: Add logic to `lib.rs`, not `main.rs`
2. **Test thoroughly**: Unit tests + integration tests
3. **Document**: Update README with examples
4. **Keep it simple**: Avoid over-engineering
5. **Spanish UX**: User-facing text in Spanish

## Common Tasks

### Run the app
```bash
cargo run -- words -c 10
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

## Examples to Guide Copilot

### Good: Generic RNG function
```rust
pub fn generate_something<R: Rng>(rng: &mut R, count: usize) -> String {
    // implementation
}
```

### Bad: Hardcoded RNG
```rust
pub fn generate_something(count: usize) -> String {
    let mut rng = thread_rng(); // Don't do this in lib
}
```

### Good: Error handling
```rust
match content_type {
    "words" => Ok(generate_words(rng, count)),
    _ => Err(format!("Tipo de contenido inválido: '{}'", content_type)),
}
```

### Good: Test with deterministic RNG
```rust
let mut rng = ChaCha8Rng::seed_from_u64(42);
let result1 = generate_words(&mut rng, 10);
let mut rng = ChaCha8Rng::seed_from_u64(42);
let result2 = generate_words(&mut rng, 10);
assert_eq!(result1, result2); // Deterministic
```

## Questions to Ask Before Changing Code

1. Does this belong in `lib.rs` or `main.rs`?
2. Have I added tests for this change?
3. Are error messages in Spanish?
4. Does this maintain the generic RNG pattern?
5. Is the README updated?

## Current Test Coverage

- 47 total tests (all passing)
- 12 unit tests (core functions)
- 4 unit tests (paragraph lengths)
- 31 integration tests (CLI with subcommands, min/max auto-calculation, error validation)

**Goal**: Maintain 100% test pass rate. Add tests for every new feature.
