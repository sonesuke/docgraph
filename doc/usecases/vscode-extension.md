# VS Code Extension Use Cases

<a id="UC-VSCODE-INSTALL"></a>

## Install VS Code Extension

The [ACT-DEV (Developer)](../actors/users.md#ACT-DEV) installs the `docgraph` VS Code extension to enable rich editing features for Markdown-based specifications.

**Derives:**

- [FR-VSC-002 (Binary Path Configuration)](../requirements/functional/vscode-extension.md#FR-VSC-002)
- [FR-VSC-003 (Markdown Activation)](../requirements/functional/vscode-extension.md#FR-VSC-003)
- [FR-VSC-004 (Server Lifecycle Commands)](../requirements/functional/vscode-extension.md#FR-VSC-004)

Depends on: [ACT-DEV (Developer)](../actors/users.md#ACT-DEV)

<a id="UC-VSCODE-LINT"></a>

## Real-time Linting

The [ACT-DEV (Developer)](../actors/users.md#ACT-DEV) sees real-time diagnostics (errors/warnings) while editing Markdown files. The extension uses the `docgraph lint` logic via LSP to identify issues like duplicate IDs or missing headings.

**Derives:**

- [FR-VSC-001 (LSP Client Integration)](../requirements/functional/vscode-extension.md#FR-VSC-001)

Depends on: [UC-EDITOR-LSP (Interactive Editing via LSP)](authoring.md#UC-EDITOR-LSP)

<a id="UC-VSCODE-NAV"></a>

## Rich Navigation

The [ACT-DEV (Developer)](../actors/users.md#ACT-DEV) navigates through the specification graph within VS Code:

- **Go to Definition**: Jumping from a reference to its corresponding anchor.
- **Find References**: Seeing all places where a specific ID is referenced.

**Derives:**

- [FR-VSC-001 (LSP Client Integration)](../requirements/functional/vscode-extension.md#FR-VSC-001)

Depends on: [UC-EDITOR-LSP (Interactive Editing via LSP)](authoring.md#UC-EDITOR-LSP)

<a id="UC-VSCODE-RENAME"></a>

## Global Renaming

The [ACT-DEV (Developer)](../actors/users.md#ACT-DEV) renames a SpecBlock ID, and the extension automatically updates all references across the workspace.

**Derives:**

- [FR-VSC-001 (LSP Client Integration)](../requirements/functional/vscode-extension.md#FR-VSC-001)

Depends on: [UC-EDITOR-LSP (Interactive Editing via LSP)](authoring.md#UC-EDITOR-LSP)
