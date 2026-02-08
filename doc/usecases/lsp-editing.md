# LSP Editing Use Cases

<a id="UC_EDITOR_LSP"></a>

## Editor Support via LSP

The developer edits documentation in an LSP-compatible editor with real-time feedback.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_CLI (Command Line Interface)](../requirements/interfaces/interfaces.md#IF_CLI)
- [IF_VSCODE_UI (VS Code UI)](../requirements/interfaces/interfaces.md#IF_VSCODE_UI)

### Requirements

- [FR_VSC_LSP_CLIENT (LSP Client Integration)](../requirements/functional/vscode.md#FR_VSC_LSP_CLIENT) Enabling real-time communication between VS Code and the LSP server
- [FR_LSP_GOTO (Go to Definition)](../requirements/functional/lsp.md#FR_LSP_GOTO) Enabling quick navigation to requirement definitions
- [FR_LSP_HOVER (Hover Information)](../requirements/functional/lsp.md#FR_LSP_HOVER) Displaying requirement titles and descriptions on hover
- [FR_LSP_COMP (Auto-completion)](../requirements/functional/lsp.md#FR_LSP_COMP) Assisting in linking known Requirement IDs
- [FR_LSP_REFS (Find References)](../requirements/functional/lsp.md#FR_LSP_REFS) Identifying all dependencies pointing to a node
- [FR_LSP_RENAME (Symbol Rename)](../requirements/functional/lsp.md#FR_LSP_RENAME) Safely updating Node IDs across multiple files
- [FR_LSP_HIERARCHY (Call Hierarchy)](../requirements/functional/lsp.md#FR_LSP_HIERARCHY) Visualizing the structural layers of specifications
- [FR_LSP_DOC_SYMBOL (Document Symbol)](../requirements/functional/lsp.md#FR_LSP_DOC_SYMBOL) Providing an outline of current file nodes
- [FR_CORE_VALID_REF (Valid References)](../requirements/functional/core.md#FR_CORE_VALID_REF) Immediate highlighting of broken links
- [FR_LSP_WS_SYMBOL (Workspace Symbol)](../requirements/functional/lsp.md#FR_LSP_WS_SYMBOL) Searching for any specification node in the workspace
- [FR_LSP_SUPPORT (LSP Server)](../requirements/functional/lsp.md#FR_LSP_SUPPORT) Mandatory core logic for editor intelligence

### Flow

1. Developer opens a Markdown file in an LSP-enabled editor.
2. Editor starts the `docgraph` language server.
3. Developer receives real-time validation and hover info.

