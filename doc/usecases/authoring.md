# Authoring Use Cases

<a id="UC-WRITE"></a>

## Write Specifications

The [ACT-DEV (Developer)](../actors/users.md#ACT-DEV) writes specifications in Markdown using anchor heading format ([ADR-MARKDOWN-FORMAT (Choice of Plain Markdown and HTML Anchors)](../decisions/markdown-format.md#ADR-MARKDOWN-FORMAT)).

Depends on: [ACT-DEV (Developer)](../actors/users.md#ACT-DEV), [ADR-MARKDOWN-FORMAT (Choice of Plain Markdown and HTML Anchors)](../decisions/markdown-format.md#ADR-MARKDOWN-FORMAT)

<a id="UC-EDITOR-LSP"></a>

## Interactive Editing via LSP

The [ACT-DEV (Developer)](../actors/users.md#ACT-DEV) uses a compatible editor to write and navigate specifications interactively.

**Features used:**

- **Navigation**: Jump to definition and find references.
- **Verification**: Real-time linting diagnostics.
- **Refactoring**: Workspace-wide renaming of SpecBlock IDs.

Depends on: [ACT-DEV (Developer)](../actors/users.md#ACT-DEV), [UC-WRITE (Write Specifications)](#UC-WRITE), [IF-LSP (Language Server Protocol (LSP) Support)](../requirements/interfaces/lsp-specs.md#IF-LSP)
