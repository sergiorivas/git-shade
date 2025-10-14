# Contributing to git-shade

Thank you for your interest in contributing to git-shade! 🎉

## Development Setup

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Git

### Getting Started

```bash
# Clone the repository
git clone https://github.com/yourusername/git-shade.git
cd git-shade

# Build the project
cargo build

# Run tests
cargo test

# Run the binary
cargo run -- --help
```

## Project Structure

```
git-shade/
├── src/
│   ├── commands/     # Command implementations
│   │   ├── init.rs   # git-shade init
│   │   ├── add.rs    # git-shade add
│   │   ├── push.rs   # git-shade push
│   │   ├── pull.rs   # git-shade pull
│   │   └── status.rs # git-shade status
│   ├── core/         # Core business logic
│   │   ├── paths.rs    # Path management
│   │   ├── config.rs   # Configuration
│   │   ├── tracker.rs  # Sync timestamps
│   │   ├── sync.rs     # Sync state detection
│   │   └── conflict.rs # Conflict handling
│   ├── git/          # Git operations
│   │   └── exclude.rs  # .git/info/exclude management
│   ├── utils/        # Utilities
│   │   ├── fs.rs       # File operations
│   │   └── project.rs  # Project detection
│   ├── error.rs      # Error types
│   ├── cli.rs        # CLI definitions
│   ├── lib.rs        # Library entry point
│   └── main.rs       # Binary entry point
├── tests/            # Integration tests
└── Cargo.toml        # Dependencies
```

## Coding Guidelines

### Style

- Follow Rust standard style (use `cargo fmt`)
- Run clippy before committing: `cargo clippy`
- Keep functions focused and small
- Add doc comments for public APIs
- Use descriptive variable names

### Error Handling

- Use `Result<T>` (defined in `error.rs`)
- Create specific error variants in `ShadeError`
- Provide helpful error messages

```rust
// Good
return Err(ShadeError::FileNotFound(path.clone()));

// Bad
return Err(anyhow::anyhow!("file not found"));
```

### Testing

- Add unit tests for new functionality
- Use `#[cfg(test)]` modules
- Integration tests go in `tests/`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_feature() {
        let temp = TempDir::new().unwrap();
        // Test logic
    }
}
```

### Commits

- Write clear, descriptive commit messages
- Use conventional commits format:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation
  - `test:` for tests
  - `refactor:` for refactoring

```
feat: add conflict detection to pull command

- Implement FileMetadata comparison
- Add ConflictInfo struct
- Display conflict resolution instructions
```

## Adding a New Command

1. Create file in `src/commands/`:

```rust
// src/commands/mycommand.rs
use crate::error::Result;

pub fn run() -> Result<()> {
    // Implementation
    Ok(())
}
```

2. Add to `src/commands/mod.rs`:

```rust
pub mod mycommand;
```

3. Add CLI definition in `src/cli.rs`:

```rust
pub enum Commands {
    // ...
    MyCommand {
        #[arg(long)]
        some_flag: bool,
    },
}
```

4. Wire up in `src/main.rs`:

```rust
Commands::MyCommand { some_flag } => {
    commands::mycommand::run(some_flag)
}
```

5. Add tests!

## Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test integration_test
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Run fmt: `cargo fmt`
6. Run clippy: `cargo clippy`
7. Commit your changes
8. Push to your fork
9. Open a Pull Request

## Pull Request Guidelines

- Describe what the PR does
- Reference any related issues
- Include tests for new functionality
- Update documentation if needed
- Ensure all tests pass
- Keep PRs focused (one feature/fix per PR)

## Questions?

- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Check existing issues/PRs first

## Code of Conduct

Be respectful, inclusive, and constructive. We're all here to build something useful together!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
