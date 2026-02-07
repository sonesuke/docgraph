# Functional Requirements: VS Code Extension

The `docgraph` VS Code extension acts as a bridge between the editor and the `docgraph` language server.

<a id="FR_VSC_LSP_CLIENT"></a>

## LSP Client Integration

The extension SHALL implement a Language Server Protocol (LSP) client that establishes communication with the `docgraph` binary running in LSP mode (`docgraph lsp`).

### Realized by

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

<a id="FR_VSC_BINARY_PATH"></a>

## Binary Path Configuration

The extension SHALL provide a setting (`docgraph.binaryPath`) allowing the user to specify the absolute path to the `docgraph` executable. If not specified, it SHALL attempt to find the binary in the system `PATH`.

### Realized by

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

<a id="FR_VSC_MARKDOWN_ACTIVATION"></a>

## Markdown Activation

The extension SHALL activate automatically when a workspace contains Markdown files (`*.md`) or when a Markdown file is opened. It SHALL associate its LSP capabilities exclusively with the `markdown` document selector.

### Realized by

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

<a id="FR_VSC_SERVER_LIFECYCLE"></a>

## Server Lifecycle Commands

The extension SHALL provide a command to restart the `docgraph` language server manually, which is useful when the binary is updated or the configuration changes.

### Realized by

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)
