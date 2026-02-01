# LSP Functional Requirements

The `docgraph` language server provides interactive features for IDEs (e.g., VS Code) to improve the experience of writing and maintaining docgraph specifications.

<a id="FR_LSP_GOTO"></a>

## Go to Definition

The server supports jumping to the source of a SpecBlock by clicking on its ID in a link.

**Capability**: `definitionProvider`

**Behavior**: Maps markdown link syntax to the anchor HTML tag in the corresponding file.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_HOVER"></a>

## Hover Information

The server displays the SpecBlock's description and metadata when hovering over an ID or link.

**Capability**: `hoverProvider`

**Behavior**: Shows the human-readable name and reference counts (incoming/outgoing) for the SpecBlock under the cursor.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_COMP"></a>

## Auto-completion

The server suggests existing SpecBlock IDs when typing links.

**Capability**: `completionProvider`

**Triggers**: `[` (link start), `#` (fragment start), `(` (path start)

**Behavior**: Returns a list of all SpecBlock IDs defined in the workspace.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_REFS"></a>

## Find References

The server searches all Markdown files for the ID string at the current cursor position.

**Capability**: `referencesProvider`

**Behavior**: Lists all occurrences of a SpecBlock ID throughout the workspace.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_RENAME"></a>

## Symbol Rename

The server renames a SpecBlock ID and automatically updates all its references across all files.

**Capability**: `renameProvider`

**Behavior**:

1. Validates if the selected text is a valid anchor ID or reference.
2. Applies workspace-wide edits to change the ID in both its definition and all identified references.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_HIERARCHY"></a>

## Call Hierarchy

The server explores incoming and outgoing relationship chains in a tree view for deep traceability analysis.

**Capability**: `callHierarchyProvider`

**Behavior**:

- **Incoming Calls**: Shows which SpecBlocks reference the current block.
- **Outgoing Calls**: Shows which SpecBlocks are referenced by the current block.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_DOC_SYMBOL"></a>

## Document Symbol

The server lists all spec blocks in the current document for navigation.

**Capability**: `documentSymbolProvider`

**Behavior**: Returns a list of all SpecBlocks defined in the current document. This populates the "Outline" view in most editors.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_WS_SYMBOL"></a>

## Workspace Symbol

The server allows searching for SpecBlocks across the entire workspace.

**Capability**: `workspaceSymbolProvider`

**Behavior**: Allows searching for SpecBlocks across the entire workspace by matching their ID or human-readable name.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_SUPPORT"></a>

## LSP Server

The system SHALL provide a Language Server Protocol server to support interactive editing.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)
