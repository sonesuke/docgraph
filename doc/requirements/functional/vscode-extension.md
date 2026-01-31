# Functional Requirements: VS Code Extension

The `docgraph` VS Code extension acts as a bridge between the editor and the `docgraph` language server.

<a id="FR-VSC-001"></a>

## LSP Client Integration

The extension SHALL implement a Language Server Protocol (LSP) client that establishes communication with the `docgraph` binary running in LSP mode (`docgraph lsp`).

**Derives:**

- [NFR-VSC-002 (Cross-platform Portability)](../non-functional/vscode-extension.md#NFR-VSC-002)
- [NFR-VSC-003 (Lightweight Packaging)](../non-functional/vscode-extension.md#NFR-VSC-003)

**Realized by:**

- [MOD-VSEX (VS Code Extension)](../../architecture/view/module.md#MOD-VSEX)
- [MOD-LSP (LSP Modules)](../../architecture/view/module.md#MOD-LSP)

**Depends on:**

- [IF-LSP (Language Server Protocol (LSP) Support)](../interfaces/lsp-specs.md#IF-LSP)

<a id="FR-VSC-002"></a>

## Binary Path Configuration

The extension SHALL provide a setting (`docgraph.binaryPath`) allowing the user to specify the absolute path to the `docgraph` executable. If not specified, it SHALL attempt to find the binary in the system `PATH`.

**Realized by:**

- [MOD-VSEX (VS Code Extension)](../../architecture/view/module.md#MOD-VSEX)

<a id="FR-VSC-003"></a>

## Markdown Activation

The extension SHALL activate automatically when a workspace contains Markdown files (`*.md`) or when a Markdown file is opened. It SHALL associate its LSP capabilities exclusively with the `markdown` document selector.

**Derives:**

- [NFR-VSC-001 (Fast Activation)](../non-functional/vscode-extension.md#NFR-VSC-001)

**Realized by:**

- [MOD-VSEX (VS Code Extension)](../../architecture/view/module.md#MOD-VSEX)

<a id="FR-VSC-004"></a>

## Server Lifecycle Commands

The extension SHALL provide a command to restart the `docgraph` language server manually, which is useful when the binary is updated or the configuration changes.

**Realized by:**

- [MOD-VSEX (VS Code Extension)](../../architecture/view/module.md#MOD-VSEX)
