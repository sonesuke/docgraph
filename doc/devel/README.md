# Developer Guide

Welcome to the `docgraph` project! This guide is designed to help you get up to speed with the codebase and start contributing.

## Quick Start

### 1. Environment Setup
`docgraph` is built with Rust. Ensure you have the latest stable toolchain installed:

```bash
rustup update stable
rustup component add clippy rustfmt llvm-tools-preview
```

### 2. Building the Project
Clone the repository and build using cargo:

```bash
git clone https://github.com/sonesuke/docgraph.git
cd docgraph
cargo build
```

### 3. Running docgraph
You can run the CLI directly using `cargo run`:

```bash
cargo run -- check ./test_data
```

## Project Structure

The project is split into several main areas:

- **`src/`**: The core logic and CLI implementation.
  - **`core/`**: Graph building, parsing, and validation rules.
  - **`cli/`**: CLI command definitions and output formatting.
  - **`lsp/`**: Language Server Protocol implementation.
- **`doc/`**: Documentation (this is where you are!).
- **`vscode-extension/`**: The VS Code extension that interacts with the LSP.

## Coding Standards

- **Formatting**: Always run `cargo fmt` before committing.
- **Linting**: We use Clippy to ensure idiomatic code. The CI will fail if there are any warnings.
  ```bash
  cargo clippy -- -D warnings
  ```
- **Security**: Be mindful of dependency security. CI includes a `cargo audit` step.

---

For deeper insights, check out:
- [Architecture Overview](./architecture.md)
- [Testing & Coverage](./testing.md)
