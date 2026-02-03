# Zed Extension Design

The `zed-extension` provides Language Server Protocol (LSP) support for the Docgraph tool within the Zed editor.

## Overview

Zed extensions are written in Rust and compiled to WebAssembly (`wasm32-wasip1`). They act as a bridge between the Zed editor and the `docgraph` language server.

## Architecture

The extension implements the `zed::Extension` trait, which provides hooks for managing the lifecycle of the language server.

### Language Server Lifecycle

The extension is responsible for:

1. **Detection**: Identifying when a Markdown file is opened.
2. **Activation**: Triggering the language server when the worktree is trusted.
3. **Command Execution**: Launching the `docgraph lsp` command.

### Configuration (`extension.toml`)

The extension is associated with the built-in Markdown language in Zed.

```toml
[language_servers.docgraph]
name = "Docgraph"
languages = ["Markdown"]
```

## Security and Trust

Since the extension executes an external binary (`docgraph`), Zed requires the user to **Trust the Workspace** (exit Restricted Mode) before the language server can start. If the LSP does not start, ensure that the workspace is trusted.

## Development

To install the extension locally for development:

1. Build the extension: `cd zed-extension && cargo build --release --target wasm32-wasip1`
2. In Zed, run the command `zed: install dev extension`.
3. Select the `zed-extension` directory.

## Traceability

- **Requirement**: [FR_INSTALL_EXT_ZED (Zed Editor Extension)](../../requirements/functional/installation.md#FR_INSTALL_EXT_ZED)
- **Module**: [MOD_EXT_ZED (Zed Editor Extension)](../view/module.md#MOD_EXT_ZED)
