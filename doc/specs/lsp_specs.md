
# LSP Specifications

<a id="IF-LSP"></a>

## Language Server Protocol (LSP) Support

The `docgraph` language server provides interactive features for IDEs (e.g., VS Code) to improve the experience of writing and maintaining docgraph specifications.

### Real-time Diagnostics

The server automatically runs `docgraph check` on the workspace when files are opened or saved. 
Errors such as duplicate IDs or broken references are reported immediately as editor diagnostics.

### Navigation and Inspection

- **Definition ([IF-LSP-GOTO](#IF-LSP-GOTO))**: Jump to the source of a SpecBlock by clicking on its ID in a link.
- **Hover ([IF-LSP-HOVER](#IF-LSP-HOVER))**: View the SpecBlock's description and metadata by hovering over an ID or link.
- **Find References ([IF-LSP-REFS](#IF-LSP-REFS))**: List all occurrences of a SpecBlock ID throughout the workspace.

### Editing Support

- **Auto-completion ([IF-LSP-COMP](#IF-LSP-COMP))**: Suggests existing SpecBlock IDs when typing links (triggers on `[`, `#`, or `(`).
- **Symbol Rename ([IF-LSP-RENAME](#IF-LSP-RENAME))**: Rename a SpecBlock ID and automatically update all its references across all files.

### Relationship Analysis

- **Call Hierarchy ([IF-LSP-HIERARCHY](#IF-LSP-HIERARCHY))**: Explore incoming and outgoing relationship chains in a tree view, allowing for deep traceability analysis within the editor.

<a id="IF-LSP-GOTO"></a>

#### Feature: Go to Definition
- **Capability**: `definitionProvider`
- **Behavior**: Maps `[text](#ID)` to the anchor `<a id="ID"></a>` in the corresponding file.

<a id="IF-LSP-HOVER"></a>

#### Feature: Hover
- **Capability**: `hoverProvider`
- **Behavior**: Displays the human-readable name and reference counts (incoming/outgoing) for the SpecBlock under the cursor.

<a id="IF-LSP-COMP"></a>

#### Feature: Auto-completion
- **Capability**: `completionProvider`
- **Trigger**: `[` (link start), `#` (fragment start), `(` (path start)
- **Behavior**: Returns a list of all SpecBlock IDs defined in the workspace.

<a id="IF-LSP-REFS"></a>

#### Feature: Find References
- **Capability**: `referencesProvider`
- **Behavior**: Searches all Markdown files for the ID string at the current cursor position.

<a id="IF-LSP-RENAME"></a>

#### Feature: Rename
- **Capability**: `renameProvider`
- **Behavior**:
  1. Validates if the selected text is a valid anchor ID or reference.
  2. Applies workspace-wide edits to change the ID in both its definition and all identified references.

<a id="IF-LSP-HIERARCHY"></a>

#### Feature: Call Hierarchy
- **Capability**: `callHierarchyProvider`
- **Behavior**:
  - **Incoming Calls**: Shows which SpecBlocks reference the current block.
  - **Outgoing Calls**: Shows which SpecBlocks are referenced by the current block.
