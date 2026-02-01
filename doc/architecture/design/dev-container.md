<a id="CC_DEV_ENV"></a>

# Development Environment

We provide a standardized development environment using VS Code Dev Containers to ensure consistency across all contributors.

## 1. Environment Configuration

Defined in [.devcontainer/devcontainer.json](../../../.devcontainer/devcontainer.json).

### Base Image

- **Image**: `mcr.microsoft.com/devcontainers/rust:1` (Debian Bullseye based).
- **Toolchain**: Latest stable Rust.

### Features

- `ghcr.io/devcontainers/features/node:1`: Required for VSIX (Extension) development.
- `ghcr.io/devcontainers/features/github-cli:1`: For PR creation and management.

### Extensions

The environment automatically installs:

- `rust-lang.rust-analyzer` (Language Server)
- `tamasfe.even-better-toml` (Config Support)
- `vadimcn.vscode-lldb` (Debugging)
- `dbaeumer.vscode-eslint` (TS Linting)

## 2. Usage Guide

### Starting the Container

1. Open `docgraph` project in VS Code.
2. Click **"Reopen in Container"** when prompted, or run the command from the palette.
3. Wait for initialization. The `postCreateCommand` will automatically run:
   - `npm install` (in `vsix/`)
   - `cargo check`

### Manual Setup (Alternative)

If you prefer to develop locally without Docker:

- **Rust**: `rustup update stable && rustup component add llvm-tools-preview`.
- **Node.js**: v18+ (Required for `vsix`).

## 3. Project Structure

- **`src/`**: Core logic (`core`), CLI (`cli`), and LSP (`lsp`).
- **`vsix/`**: VS Code Extension (TypeScript) acting as LSP Client.
- **`doc/`**: Documentation Graph.

## 4. Coding Standards

- **Formatting**: `cargo fmt` (Rust), `npm run format` (VSIX).
- **Linting**: `cargo clippy` (Rust), `npm run lint` (VSIX).
- **Security**: `cargo audit`.
