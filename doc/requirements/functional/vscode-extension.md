# Functional Requirements: VS Code Extension

The `docgraph` VS Code extension acts as a bridge between the editor and the `docgraph` language server.

<a id="FR_VSC_001"></a>

## LSP Client Integration

The extension SHALL implement a Language Server Protocol (LSP) client that establishes communication with the `docgraph` binary running in LSP mode (`docgraph lsp`).

**Derives:**

- [NFR_VSC_002 (Cross-platform Portability)](../non-functional/vscode-extension.md#NFR_VSC_002)
- [NFR_VSC_003 (Lightweight Packaging)](../non-functional/vscode-extension.md#NFR_VSC_003)

**Realized by:**

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)
- [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

**Depends on:**

- [IF_LSP (Language Server Protocol (LSP) Support)](../interfaces/lsp-specs.md#IF_LSP)

<a id="FR_VSC_002_RENAMED"></a>

## Binary Path Configuration

The extension SHALL provide a setting (`docgraph.binaryPath`) allowing the user to specify the absolute path to the `docgraph` executable. If not specified, it SHALL attempt to find the binary in the system `PATH`.

**Realized by:**

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

<a id="FR_VSC_003"></a>

## Markdown Activation

The extension SHALL activate automatically when a workspace contains Markdown files (`*.md`) or when a Markdown file is opened. It SHALL associate its LSP capabilities exclusively with the `markdown` document selector.

**Derives:**

- [NFR_VSC_001 (Fast Activation)](../non-functional/vscode-extension.md#NFR_VSC_001)

**Realized by:**

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

<a id="FR_VSC_004"></a>

## Server Lifecycle Commands

The extension SHALL provide a command to restart the `docgraph` language server manually, which is useful when the binary is updated or the configuration changes.

**Realized by:**

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)
