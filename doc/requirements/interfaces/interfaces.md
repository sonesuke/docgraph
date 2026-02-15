# Interface Specifications

<a id="IF_CONFIG"></a>

## docgraph.toml Configuration

The `docgraph.toml` file at the project root defines the validation rules and workspace behavior.

**Specifications:**

- **File Path**: `docgraph.toml` (Project Root)
- **Format**: TOML

### Defined by

- [FR_CORE_CONFIGURATION (Configuration)](../../requirements/functional/core.md#FR_CORE_CONFIGURATION)

### Justified by

- [UC_WRITE (Write Specifications)](../../usecases/writing-specification.md#UC_WRITE)

---

<a id="IF_CLI"></a>

## Command Line Interface

The `docgraph` CLI provides commands for linting, graph generation, and analysis of Markdown documentation.

### Defined by

- [FR_CLI_LINT (Lint Command)](../../requirements/functional/cli.md#FR_CLI_LINT)
- [FR_CLI_GRAPH (Graph Command)](../../requirements/functional/cli.md#FR_CLI_GRAPH)
- [FR_CLI_LIST (List Capability)](../../requirements/functional/cli.md#FR_CLI_LIST)

### Justified by

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)

---

<a id="IF_CLAUDE_CODE"></a>

## Interface: Claude Code Plugin

The Docgraph Claude plugin provides a specialized interface for Claude Code, enabling the AI agent to perform complex
documentation analysis and manipulation.

The interface acts as a high-level wrapper around the Docgraph core functions, tailored for the agentic capabilities of
Claude. It leverages the Model Context Protocol (MCP) to expose tools and resources.

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

### Defined by

- [FR_CLAUDE_RAG (Retrieval-Augmented Generation)](../../requirements/functional/claude.md#FR_CLAUDE_RAG)

### Justified by

- [UC_AI_ASSISTANCE (AI-Assisted Documentation)](../../usecases/ai-assistance.md#UC_AI_ASSISTANCE)

---

<a id="IF_CLAUDE_MARKETPLACE"></a>

## Claude Marketplace

The distribution platform for Claude Desktop plugins.

### Defined by

- [FR_CLAUDE_MARKETPLACE (Claude Marketplace Support)](../../requirements/functional/claude.md#FR_CLAUDE_MARKETPLACE)

### Justified by

- [UC_CLAUDE_INSTALL (Install Claude Plugin)](../../usecases/setup.md#UC_CLAUDE_INSTALL)

---

<a id="IF_VSCODE_MARKETPLACE"></a>

## VS Code Marketplace

The distribution platform for Visual Studio Code extensions.

### Defined by

- [FR_VSC_LSP_CLIENT (LSP Client Integration)](../../requirements/functional/vscode.md#FR_VSC_LSP_CLIENT)

### Justified by

- [UC_VSCODE_INSTALL (Install VS Code Extension)](../../usecases/setup.md#UC_VSCODE_INSTALL)

---

<a id="IF_VSCODE_UI"></a>

## VS Code UI

The user interface of Visual Studio Code, including commands, views, and settings.

### Defined by

- [FR_VSC_LSP_CLIENT (LSP Client Integration)](../../requirements/functional/vscode.md#FR_VSC_LSP_CLIENT)

### Justified by

- [UC_EDITOR_LSP (Editor Support via LSP)](../../usecases/lsp-editing.md#UC_EDITOR_LSP)

---

<a id="IF_GITHUB_RELEASES"></a>

## GitHub Releases Interface

The system interacts with GitHub Releases to distribute pre-compiled binaries and extension packages.

**Specifications:**

1. **Host**: `github.com`
2. **Access**: Public anonymous access for downloads.
3. **Format**: Compressed archives (`.tar.gz`, `.zip`) and VSIX packages.

### Defined by

- [FR_INSTALL_BINARY (Binary Installation Support)](../../requirements/functional/installation.md#FR_INSTALL_BINARY)

### Justified by

- [UC_INSTALL_BINARY (Install via Binary Script)](../../usecases/setup.md#UC_INSTALL_BINARY)

---

<a id="IF_ZED_UI"></a>

## Zed UI

The user interface of the Zed editor, including the command palette and settings.

### Defined by

- [FR_INSTALL_EXT_ZED (Zed Editor Extension)](../../requirements/functional/installation.md#FR_INSTALL_EXT_ZED)

### Justified by

- [UC_ZED_INSTALL (Install Zed Extension)](../../usecases/setup.md#UC_ZED_INSTALL)
