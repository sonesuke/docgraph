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

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

<a id="FR_INSTALL_BINARY"></a>

## Binary Installation Support

<<<<<<< HEAD
The system must provide pre-compiled binaries and official installation scripts (hosted on [GitHub Releases](../../requirements/interfaces/interfaces.md#IF_GITHUB_RELEASES)) to simplify deployment.
=======
The system must provide pre-compiled binaries and official installation scripts to simplify deployment.
>>>>>>> origin/main

1. **Platform Support**: Linux (x86_64), macOS (x86_64/aarch64), Windows (x86_64).
2. **Automated Scripts**: Provide a shell script for Unix-like systems and a PowerShell script for Windows to handle downloading, extraction, and PATH configuration.

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

<a id="FR_INSTALL_EXT_ZED"></a>

## Zed Editor Extension

<<<<<<< HEAD
The system must provide an extension for the Zed editor to enable LSP support via [Zed UI](../../requirements/interfaces/interfaces.md#IF_ZED_UI).
=======
The system must provide an extension for the Zed editor to enable LSP support.
>>>>>>> origin/main

1. **Format**: WASM (`wasm32-wasip1`)
2. **Features**: Real-time diagnostics, Go to Definition, Traceability References.
3. **Distribution**: Source code or packaged archive.
4. **Security**: Requires **Trusting the Workspace** (exiting Restricted Mode) in Zed to allow the external binary execution.
5. **Configuration**: Requires a `.zed/settings.json` in the project root to explicitly enable the `docgraph` language server.

### Realized by

- [MOD_EXT_ZED (Zed Editor Extension)](../../architecture/view/module.md#MOD_EXT_ZED)
