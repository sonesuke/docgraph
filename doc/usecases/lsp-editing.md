# LSP Editing Use Cases

<a id="UC_EDITOR_LSP"></a>

## Editor Support via LSP

The developer edits documentation in an LSP-compatible editor with real-time feedback.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)
- [ACT_CI (CI System)](../actors/systems.md#ACT_CI)

### Interfaces

- [IF_VSCODE_UI (VS Code UI)](../requirements/interfaces/interfaces.md#IF_VSCODE_UI)

### Flow

1. Developer opens a Markdown file in an LSP-enabled editor.
2. Editor starts the `docgraph` language server.
3. Developer receives real-time validation and hover info.
