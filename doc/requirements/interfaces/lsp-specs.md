
# LSP Specifications

<a id="IF_LSP"></a>

## Language Server Protocol (LSP) Support

The `docgraph` language server provides interactive features for IDEs (e.g., VS Code) to improve the experience of writing and maintaining docgraph specifications.

### Real-time Diagnostics

The server automatically runs `docgraph check` on the workspace when files are opened or saved.
Errors such as duplicate IDs or broken references are reported immediately as editor diagnostics.

### Navigation and Inspection

- **Definition ([IF_LSP_GOTO (Feature: Go to Definition)](#IF_LSP_GOTO))**: Jump to the source of a SpecBlock by clicking on its ID in a link.
- **Hover ([IF_LSP_HOVER (Feature: Hover)](#IF_LSP_HOVER))**: View the SpecBlock's description and metadata by hovering over an ID or link.
- **Find References ([IF_LSP_REFS (Feature: Find References)](#IF_LSP_REFS))**: List all occurrences of a SpecBlock ID throughout the workspace.

### Editing Support

- **Auto-completion ([IF_LSP_COMP (Feature: Auto-completion)](#IF_LSP_COMP))**: Suggests existing SpecBlock IDs when typing links (triggers on link start, fragment hash, or path parenthesis).
- **Symbol Rename ([IF_LSP_RENAME (Feature: Rename)](#IF_LSP_RENAME))**: Rename a SpecBlock ID and automatically update all its references across all files.

### Relationship Analysis

- **Call Hierarchy ([IF_LSP_HIERARCHY (Feature: Call Hierarchy)](#IF_LSP_HIERARCHY))**: Explore incoming and outgoing relationship chains in a tree view, allowing for deep traceability analysis within the editor.

<a id="IF_LSP_GOTO"></a>

#### Feature: Go to Definition

- **Capability**: `definitionProvider`
- **Behavior**: Maps markdown link syntax to the anchor HTML tag in the corresponding file.

<a id="IF_LSP_HOVER"></a>

#### Feature: Hover

- **Capability**: `hoverProvider`
- **Behavior**: Displays the human-readable name and reference counts (incoming/outgoing) for the SpecBlock under the cursor.

<a id="IF_LSP_COMP"></a>

#### Feature: Auto-completion

- **Capability**: `completionProvider`
- **Trigger**: `[` (link start), `#` (fragment start), `(` (path start)
- **Behavior**: Returns a list of all SpecBlock IDs defined in the workspace.

<a id="IF_LSP_REFS"></a>

#### Feature: Find References

- **Capability**: `referencesProvider`
- **Behavior**: Searches all Markdown files for the ID string at the current cursor position.

<a id="IF_LSP_RENAME"></a>

#### Feature: Rename

- **Capability**: `renameProvider`
- **Behavior**:
  1. Validates if the selected text is a valid anchor ID or reference.
  2. Applies workspace-wide edits to change the ID in both its definition and all identified references.

<a id="IF_LSP_HIERARCHY"></a>

#### Feature: Call Hierarchy

- **Capability**: `callHierarchyProvider`
- **Behavior**:
  - **Incoming Calls**: Shows which SpecBlocks reference the current block.
  - **Outgoing Calls**: Shows which SpecBlocks are referenced by the current block.
