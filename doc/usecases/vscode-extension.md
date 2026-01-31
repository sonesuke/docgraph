# VS Code Extension Use Cases

<a id="UC_VSCODE_INSTALL"></a>

## Install VS Code Extension

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) installs the `docgraph` VS Code extension to enable rich editing features for Markdown-based specifications.

**Derives:**

- [FR_VSC_002_RENAMED (Binary Path Configuration)](../requirements/functional/vscode-extension.md#FR_VSC_002_RENAMED)
- [FR_VSC_003 (Markdown Activation)](../requirements/functional/vscode-extension.md#FR_VSC_003)
- [FR_VSC_004 (Server Lifecycle Commands)](../requirements/functional/vscode-extension.md#FR_VSC_004)

Depends on: [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

<a id="UC_VSCODE_LINT"></a>

## Real-time Linting

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) sees real-time diagnostics (errors/warnings) while editing Markdown files. The extension uses the `docgraph lint` logic via LSP to identify issues like duplicate IDs or missing headings.

**Derives:**

- [FR_VSC_001 (LSP Client Integration)](../requirements/functional/vscode-extension.md#FR_VSC_001)

Depends on: [UC_EDITOR_LSP (Interactive Editing via LSP)](authoring.md#UC_EDITOR_LSP)

<a id="UC_VSCODE_NAV"></a>

## Rich Navigation

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) navigates through the specification graph within VS Code:

- **Go to Definition**: Jumping from a reference to its corresponding anchor.
- **Find References**: Seeing all places where a specific ID is referenced.

**Derives:**

- [FR_VSC_001 (LSP Client Integration)](../requirements/functional/vscode-extension.md#FR_VSC_001)

Depends on: [UC_EDITOR_LSP (Interactive Editing via LSP)](authoring.md#UC_EDITOR_LSP)

<a id="UC_VSCODE_RENAME"></a>

## Global Renaming

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) renames a SpecBlock ID, and the extension automatically updates all references across the workspace.

**Derives:**

- [FR_VSC_001 (LSP Client Integration)](../requirements/functional/vscode-extension.md#FR_VSC_001)

Depends on: [UC_EDITOR_LSP (Interactive Editing via LSP)](authoring.md#UC_EDITOR_LSP)
