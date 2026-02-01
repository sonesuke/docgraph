# Developer Guide

Welcome to the `docgraph` project! This guide is designed to help you get up to speed with the codebase and start contributing.

## Quick Start

### 1. Environment Setup

#### Prerequisites

- [Rust Toolchain](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (for VSIX development)
- [VS Code Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers) (Optional)

#### Quick Start (Dev Container)

Since this project requires both Rust, Node.js, and specific VS Code extensions, we highly recommend using the provided Dev Container.

1. Open the project in VS Code.
2. Click "Reopen in Container" when prompted.
3. The environment will automatically install all dependencies (`cargo check`, `npm install`).

#### Manual Setup

`docgraph` is built with Rust. Ensure you have the latest stable toolchain installed:

```bash
rustup update stable
rustup component add clippy rustfmt llvm-tools-preview
```

### 2. Building the Project

Clone the repository ∏and build using cargo:

```bash
git clone https://github.com/sonesuke/docgraph.git
cd docgraph
cargo build
```

### 3. Running docgraph

You can run the CLI directly using `cargo run`:

```bash
cargo run -- check ./doc
```

## Project Structure

The project is split into several main areas:

- **`src/`**: The core logic and CLI implementation.
  - **`core/`**: Graph building, parsing, and validation rules.
  - **`cli/`**: CLI command definitions and output formatting.
  - **`lsp/`**: Language Server Protocol implementation.
- **`doc/`**: Documentation (this is where you are!).

## Coding Standards

- **Formatting**: Always run `cargo fmt` before committing.
- **Linting**: We use Clippy to ensure idiomatic code. The CI will fail if there are any warnings.

  ```bash
  cargo clippy -- -D warnings
  ```

- **Security**: Be mindful of dependency security. CI includes a `cargo audit` step.

---

For a deeper understanding of the system architecture, see:

- [Layered Architecture](./layered-architecture.md)
- [Module View](../view/module.md)
- [Testing & Coverage](./testing.md)
