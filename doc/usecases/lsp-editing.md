# LSP-Based Navigation Use Cases

<a id="UC_EDITOR_LSP"></a>

## Interactive Editing via LSP

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) uses a compatible editor to write and navigate specifications interactively.

**Features used:**

- **Navigation**: Jump to definition and find references.
- **Verification**: Real-time linting diagnostics.
- **Refactoring**: Workspace-wide renaming of SpecBlock IDs.

**Derives:**

- [IF_CLI (Command Line Interface)](../requirements/interfaces/interfaces.md#IF_CLI)
- [FR_LSP_SUPPORT (LSP Server)](../requirements/functional/lsp.md#FR_LSP_SUPPORT)
- [FR_VSC_LSP_CLIENT (LSP Client Integration)](../requirements/functional/vscode.md#FR_VSC_LSP_CLIENT)
- [FR_VSC_BINARY_PATH (Binary Path Configuration)](../requirements/functional/vscode.md#FR_VSC_BINARY_PATH)
- [FR_VSC_MARKDOWN_ACTIVATION (Markdown Activation)](../requirements/functional/vscode.md#FR_VSC_MARKDOWN_ACTIVATION)
- [FR_VSC_SERVER_LIFECYCLE (Server Lifecycle Commands)](../requirements/functional/vscode.md#FR_VSC_SERVER_LIFECYCLE)
- [FR_LSP_GOTO (Go to Definition)](../requirements/functional/lsp.md#FR_LSP_GOTO)
- [FR_LSP_HOVER (Hover Information)](../requirements/functional/lsp.md#FR_LSP_HOVER)
- [FR_LSP_COMP (Auto-completion)](../requirements/functional/lsp.md#FR_LSP_COMP)
- [FR_LSP_REFS (Find References)](../requirements/functional/lsp.md#FR_LSP_REFS)
- [FR_LSP_RENAME (Symbol Rename)](../requirements/functional/lsp.md#FR_LSP_RENAME)
- [FR_LSP_HIERARCHY (Call Hierarchy)](../requirements/functional/lsp.md#FR_LSP_HIERARCHY)
- [FR_LSP_DOC_SYMBOL (Document Symbol)](../requirements/functional/lsp.md#FR_LSP_DOC_SYMBOL)
- [FR_LSP_WS_SYMBOL (Workspace Symbol)](../requirements/functional/lsp.md#FR_LSP_WS_SYMBOL)
