# VS Code Extension Use Cases

<a id="UC_VSCODE_INSTALL"></a>

## Install VS Code Extension

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) installs the `docgraph` VS Code extension to enable rich editing features for Markdown-based specifications.

**Derives:**

- [FR_VSC_BINARY_PATH (Binary Path Configuration)](../requirements/functional/vscode-extension.md#FR_VSC_BINARY_PATH)
- [FR_VSC_MARKDOWN_ACTIVATION (Markdown Activation)](../requirements/functional/vscode-extension.md#FR_VSC_MARKDOWN_ACTIVATION)
- [FR_VSC_SERVER_LIFECYCLE (Server Lifecycle Commands)](../requirements/functional/vscode-extension.md#FR_VSC_SERVER_LIFECYCLE)

Depends on: [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

<a id="UC_VSCODE_LINT"></a>

## Real-time Linting

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) sees real-time diagnostics (errors/warnings) while editing Markdown files. The extension uses the `docgraph lint` logic via LSP to identify issues like duplicate IDs or missing headings.

**Derives:**

- [FR_VSC_LSP_CLIENT (LSP Client Integration)](../requirements/functional/vscode-extension.md#FR_VSC_LSP_CLIENT)

Depends on: [UC_EDITOR_LSP (Interactive Editing via LSP)](authoring.md#UC_EDITOR_LSP)

<a id="UC_VSCODE_NAV"></a>

## Rich Navigation

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) navigates through the specification graph within VS Code:

- **Go to Definition**: Jumping from a reference to its corresponding anchor.
- **Find References**: Seeing all places where a specific ID is referenced.

**Derives:**

- [FR_VSC_LSP_CLIENT (LSP Client Integration)](../requirements/functional/vscode-extension.md#FR_VSC_LSP_CLIENT)

Depends on: [UC_EDITOR_LSP (Interactive Editing via LSP)](authoring.md#UC_EDITOR_LSP)

<a id="UC_VSCODE_RENAME"></a>

## Global Renaming

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) renames a SpecBlock ID, and the extension automatically updates all references across the workspace.

**Derives:**

- [FR_VSC_LSP_CLIENT (LSP Client Integration)](../requirements/functional/vscode-extension.md#FR_VSC_LSP_CLIENT)

Depends on: [UC_EDITOR_LSP (Interactive Editing via LSP)](authoring.md#UC_EDITOR_LSP)
