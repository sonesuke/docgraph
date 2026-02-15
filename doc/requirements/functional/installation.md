# Installation Requirements

The system must support installation on developer machines.

## Requirements

<a id="FR_INSTALL_MANUAL"></a>

## Manual Installation

The system must support manual installation by providing valid configurations for standard tools.

1. **Rust Toolchain**:
   - Version: Latest Stable
   - Components: `clippy`, `rustfmt`, `llvm-tools-preview`
2. **Node.js**:
   - Version: v18 or higher (Required for VS Code Extension)

### Derived from

- [UC_INSTALL_MANUAL (Install Manual Setup)](../../usecases/setup.md#UC_INSTALL_MANUAL)

<a id="FR_INSTALL_BINARY"></a>

## Binary Installation Support

The system must provide pre-compiled binaries and official installation scripts (hosted on
[GitHub Releases](../../requirements/interfaces/interfaces.md#IF_GITHUB_RELEASES)) to simplify deployment.

1. **Platform Support**: Linux (x86_64), macOS (x86_64/aarch64), Windows (x86_64).
2. **Automated Scripts**: Provide a shell script for Unix-like systems and a PowerShell script for Windows to handle
   downloading, extraction, and PATH configuration.

### Derived from

- [UC_INSTALL_BINARY (Install via Binary Script)](../../usecases/setup.md#UC_INSTALL_BINARY)

<a id="FR_INSTALL_EXT_ZED"></a>

## Zed Editor Extension

The system must provide an extension for the Zed editor to enable LSP support via
[Zed UI](../../requirements/interfaces/interfaces.md#IF_ZED_UI).

1. **Format**: WASM (`wasm32-wasip1`)
2. **Features**: Real-time diagnostics, Go to Definition, Traceability References.
3. **Distribution**: Source code or packaged archive.
4. **Security**: Requires **Trusting the Workspace** (exiting Restricted Mode) in Zed to allow the external binary
   execution.
5. **Configuration**: Requires a `.zed/settings.json` in the project root to explicitly enable the `docgraph` language
   server.

### Derived from

- [UC_ZED_INSTALL (Install Zed Extension)](../../usecases/setup.md#UC_ZED_INSTALL)

<a id="FR_INSTALL_PREK"></a>

## Pre-commit Hook Support

The system must support automated code quality checks before commits using `prek`.

1. **Tool**: `prek` (Rust-based pre-commit runner).
2. **Automated Checks**:
   - Rust: `cargo fmt`, `cargo clippy`, `cargo test`.
   - Docs/Config: `prettier` (with TOML plugin).
   - Validation: `docgraph check`.
3. **Setup**: Supports a one-command setup via `prek install -f`.

### Derived from

- [UC_PREK_SETUP (Set Up Development Hooks)](../../usecases/setup.md#UC_PREK_SETUP)
