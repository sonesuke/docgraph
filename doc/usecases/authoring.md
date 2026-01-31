# Authoring Use Cases

<a id="UC_WRITE"></a>

## Write Specifications

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) writes specifications in Markdown using anchor heading format ([ADR_MARKDOWN_FORMAT (Choice of Plain Markdown and HTML Anchors)](../decisions/markdown-format.md#ADR_MARKDOWN_FORMAT)).

**Derives:**

- [IF_EDITOR (Markdown Editor)](../requirements/interfaces/external.md#IF_EDITOR)
- [FR_UNIQUE (Unique IDs)](../requirements/functional/verification.md#FR_UNIQUE)

Depends on: [ACT_DEV (Developer)](../actors/users.md#ACT_DEV), [ADR_MARKDOWN_FORMAT (Choice of Plain Markdown and HTML Anchors)](../decisions/markdown-format.md#ADR_MARKDOWN_FORMAT)

<a id="UC_EDITOR_LSP"></a>

## Interactive Editing via LSP

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) uses a compatible editor to write and navigate specifications interactively.

**Features used:**

- **Navigation**: Jump to definition and find references.
- **Verification**: Real-time linting diagnostics.
- **Refactoring**: Workspace-wide renaming of SpecBlock IDs.

**Derives:**

- [FR_CLI_LSP (LSP Server)](../requirements/functional/cli.md#FR_CLI_LSP)

Depends on: [ACT_DEV (Developer)](../actors/users.md#ACT_DEV), [UC_WRITE (Write Specifications)](#UC_WRITE), [IF_LSP (Language Server Protocol (LSP) Support)](../requirements/interfaces/lsp-specs.md#IF_LSP)
