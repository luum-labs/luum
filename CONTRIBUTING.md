# Contributing to luum

Contributions are welcome. Please follow these guidelines:

## Getting Started

1. Fork and clone the repository
2. Create a new branch from `main`
3. Make your changes

## Development Setup

### Rust

```bash
rustup default stable
cargo build
cargo test
cargo fmt --all
cargo clippy --all-targets
```

### TypeScript SDK

```bash
cd sdk
npm install
npm test
```

## Pull Requests

- Write clear, descriptive commit messages
- Include tests for new functionality
- Run `cargo fmt` and `cargo clippy` before submitting
- Keep pull requests focused on a single change

## Code Style

- Rust: follow `rustfmt.toml` configuration
- TypeScript: strict mode, no `any` types
- All public functions require documentation

## Reporting Issues

Open an issue with:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs or error messages
