<a id="CC_DEV_ENV"></a>

# Development Environment

We provide a standardized development environment using VS Code Dev Containers to ensure consistency across all contributors and the CI pipeline.

**Derived From:**

- [ADR_CI_ENV_PARITY (CI Environment Parity)](../../decisions/ci-env-parity.md#ADR_CI_ENV_PARITY)

**Realized by:** [MOD_DEV_CONTAINER (Dev Container Modules)](../view/module.md#MOD_DEV_CONTAINER)

Defined in [.devcontainer/devcontainer.json](../../../.devcontainer/devcontainer.json).

## Base Image

- **Image**: `mcr.microsoft.com/devcontainers/rust:1` (Debian Bullseye based).
- **Toolchain**: Latest stable Rust.

## Features

- `ghcr.io/devcontainers/features/node:1`: Required for VSIX (Extension) development.
- `ghcr.io/devcontainers/features/github-cli:1`: For PR creation and management.

## CLI Tools

The `postCreateCommand` installs additional tools:

- **docgraph**: Installed from source via `cargo install --path .`
- **Claude Code**: AI assistant installed via `curl -fsSL https://claude.ai/install.sh | bash`
  - Configured with docgraph plugin support
  - Aliased to skip permissions for container development

## Extensions

The environment automatically installs:

- `rust-lang.rust-analyzer` (Language Server)
- `tamasfe.even-better-toml` (Config Support)
- `vadimcn.vscode-lldb` (Debugging)
- `dbaeumer.vscode-eslint` (TS Linting)
- `anthropic.claude-code` (AI Assistant)

## Coding Standards

- **Formatting**: `cargo fmt` (Rust), `npm run format` (VSIX).
- **Linting**: `cargo clippy` (Rust), `npm run lint` (VSIX).
- **Security**: `cargo audit`.
