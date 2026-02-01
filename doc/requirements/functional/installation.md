<a id="FR_INSTALL"></a>

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
