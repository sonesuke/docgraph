# LSP Functional Requirements

The `docgraph` language server provides interactive features for IDEs (e.g., VS Code) to improve the experience of writing and maintaining docgraph specifications.

<a id="FR_LSP_GOTO"></a>

## Go to Definition

The server supports jumping to the source of a Node by clicking on its ID in a link.

**Capability**: `definitionProvider`

**Behavior**: Maps markdown link syntax to the anchor HTML tag in the corresponding file.

### Realized by

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_HOVER"></a>

## Hover Information

The server displays the Node's description and metadata when hovering over an ID or link.

**Capability**: `hoverProvider`

**Behavior**: Shows the human-readable name and reference counts (incoming/outgoing) for the Node under the cursor.

### Realized by

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_COMP"></a>

## Auto-completion

The server suggests existing Node IDs when typing links.

**Capability**: `completionProvider`

**Triggers**: `[` (link start), `#` (fragment start), `(` (path start)

**Behavior**: Returns a list of all Node IDs defined in the workspace.

### Realized by

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_REFS"></a>

## Find References

The server searches all Markdown files for the ID string at the current cursor position.

**Capability**: `referencesProvider`

**Behavior**: Lists all occurrences of a Node ID throughout the workspace.

### Realized by

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_RENAME"></a>

## Symbol Rename

The server renames a Node ID and automatically updates all its references across all files.

**Capability**: `renameProvider`

**Behavior**:

1. Validates if the selected text is a valid anchor ID or reference.
2. Applies workspace-wide edits to change the ID in both its definition and all identified references.

### Realized by

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_HIERARCHY"></a>

## Call Hierarchy

The server explores incoming and outgoing relationship chains in a tree view for deep traceability analysis.

**Capability**: `callHierarchyProvider`

**Behavior**:

- **Incoming Calls**: Shows which Nodes reference the current block.
- **Outgoing Calls**: Shows which Nodes are referenced by the current block.

### Realized by

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_DOC_SYMBOL"></a>

## Document Symbol

The server lists all nodes in the current document for navigation.

**Capability**: `documentSymbolProvider`

**Behavior**: Returns a list of all Nodes defined in the current document. This populates the "Outline" view in most editors.

### Realized by

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_WS_SYMBOL"></a>

## Workspace Symbol

The server allows searching for Nodes across the entire workspace.

**Capability**: `workspaceSymbolProvider`

**Behavior**: Allows searching for Nodes across the entire workspace by matching their ID or human-readable name.

### Realized by

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)

<a id="FR_LSP_SUPPORT"></a>

## LSP Server

The system SHALL provide a Language Server Protocol server to support interactive editing.

### Realized by

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)
