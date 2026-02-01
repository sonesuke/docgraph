
# Installation Requirements

The system must support installation on developer machines.

## Requirements

### <a id="FR_INSTALL_MANUAL"></a> Manual Installation

The system must support manual installation by providing valid configurations for standard tools.

#### Specifications

1. **Rust Toolchain**:
   - Version: Latest Stable
   - Components: `clippy`, `rustfmt`, `llvm-tools-preview`
2. **Node.js**:
   - Version: v18 or higher (Required for VS Code Extension)

**Derived From:**

- UC_INSTALL_MANUAL (Install Manual Setup)

**Realized By:**

- [MOD_CLI (CLI Modules)](../../architecture/view/module.md#MOD_CLI)

### <a id="FR_INSTALL_BINARY"></a> Binary Installation Support

The system must provide pre-compiled binaries and official installation scripts to simplify deployment.

#### Specifications

1. **Platform Support**: Linux (x86_64), macOS (x86_64/aarch64), Windows (x86_64).
2. **Automated Scripts**: Provide a shell script for Unix-like systems and a PowerShell script for Windows to handle downloading, extraction, and PATH configuration.

**Derived From:**

- [UC_INSTALL_BINARY (Install via Binary Script)](../../usecases/setup.md#UC_INSTALL_BINARY)

**Realized By:**

- [MOD_CLI (CLI Modules)](../../architecture/view/module.md#MOD_CLI)
