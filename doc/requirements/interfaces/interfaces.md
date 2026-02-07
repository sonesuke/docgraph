# Interface Specifications

<a id="IF_CONFIG"></a>

## docgraph.toml Configuration

The `docgraph.toml` file at the project root defines the validation rules and workspace behavior, including node type definitions, relationship rules, and global graph settings.

**Configuration Structure:**

```toml
[graph]
doc_types = ["ACT", "DAT", "IF", "ADR"]

[node_types]
UC = { desc = "Use Case" }
FR = { desc = "Functional Requirement" }

[references.UC]
rules = [
  { dir = "to", targets = ["FR"], min = 1, desc = "Use cases must derive functional requirements" }
]
```

**Settings:**

| Attribute      | Type           | Description                                                  |
| :------------- | :------------- | :----------------------------------------------------------- |
| `doc_types`    | `List<String>` | Node types that are exempt from strict relation checks.      |
| `[node_types]` | `Table`        | Defines allowed prefixes for nodes.                          |
| `[references]` | `Table`        | Defines constraints on how different node types can connect. |

**Note:** Strict node type and strict relation validation are always enabled and cannot be disabled.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

---

<a id="IF_CLI"></a>

## Command Line Interface

The `docgraph` CLI provides commands for linting, graph generation, and analysis of Markdown documentation.

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

---

<a id="IF_CLAUDE_CODE"></a>

## Interface: Claude Code Plugin

The Docgraph Claude plugin provides a specialized interface for Claude Code, enabling the AI agent to perform complex documentation analysis and manipulation.

The interface acts as a high-level wrapper around the Docgraph core functions, tailored for the agentic capabilities of Claude. It leverages the Model Context Protocol (MCP) to expose tools and resources.

**Exposed Capabilities:**

The interface MUST expose the following capabilities to the AI agent:

1. **Knowledge Extraction**: Tools to retrieve the structured graph from Markdown files.
2. **Traceability Analysis**: Tools to trace dependencies and impacts across the graph.
3. **Linting & Fixing**: Tools to run validation rules and apply automated fixes.
4. **Rule Explanation**: Tools to retrieve human-readable descriptions of architectural rules.

**Implementation:**

The implementation is defined in:

- [plugin.json (Plugin Definition)](../../../docgraph-plugin/.claude-plugin/plugin.json)
- [SKILL.md (Plugin Skill Definition)](../../../docgraph-plugin/skills/docgraph/SKILL.md)

### Realized by

- [MOD_PLUGIN (Claude Code Plugin)](../../architecture/view/module.md#MOD_PLUGIN)

---

<a id="IF_CLAUDE_MARKETPLACE"></a>

## Claude Marketplace

The distribution platform for Claude Desktop plugins.

### Realized by

- [MOD_PLUGIN (Claude Code Plugin)](../../architecture/view/module.md#MOD_PLUGIN)

---

<a id="IF_VSCODE_MARKETPLACE"></a>

## VS Code Marketplace

The distribution platform for Visual Studio Code extensions.

### Realized by

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

---

<a id="IF_VSCODE_UI"></a>

## VS Code UI

The user interface of Visual Studio Code, including commands, views, and settings.

### Realized by

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

---

<a id="IF_GITHUB_RELEASES"></a>

## GitHub Releases Interface

The system interacts with GitHub Releases to distribute pre-compiled binaries and extension packages.

**Specifications:**

1. **Host**: `github.com`
2. **Access**: Public anonymous access for downloads.
3. **Format**: Compressed archives (`.tar.gz`, `.zip`) and VSIX packages.

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

---

<a id="IF_ZED_UI"></a>

## Zed UI

The user interface of the Zed editor, including the command palette and settings.

### Realized by

- [MOD_EXT_ZED (Zed Editor Extension)](../../architecture/view/module.md#MOD_EXT_ZED)
