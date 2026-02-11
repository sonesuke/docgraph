# docgraph

[![CI](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml/badge.svg)](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml)
[![Security audit](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml/badge.svg)](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/badge/coverage-view-blue)](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml)

A powerful lint tool and graph generator designed to build and verify directed graphs embedded in standard Markdown
files. `docgraph` ensures your documentation is consistent, traceable, and AI-ready.

## Overview

`docgraph` treats Markdown blocks as nodes in a graph. By using HTML anchors (`<a id="..."></a>`) and standard Markdown
links, you can define structured relationships (edges) between documents. This enables automated traceability across
your entire documentation suite—from business actors and use cases down to technical requirements and architecture
decisions.

For a comprehensive guide on concepts, architecture, and specifications, please refer to the
**[Documentation Overview](./doc/overview.md)**.

## Key Features

- **Graph Validation**: Automated checks for broken links, duplicate IDs, and relationship rule violations defined in
  `docgraph.toml`.
- **Template Validation**: Enforce consistent documentation structure using Markdown templates with wildcard support.
- **Traceability Analysis**: Tools to search, trace, and describe complex node relationships.

- **AI-Agent Ready**: Built-in support for GraphRAG knowledge construction. AI Agents can consume `docgraph` outputs to
  assist in documentation and development workflows.
- **Modern IDE Integration**: A dedicated VS Code plugin with full LSP support.

## Getting Started

Follow these steps to set up `docgraph` for your project.

### Step 1: Install CLI Binary

`docgraph` requires the CLI binary to be installed on your system for all use cases (CLI, AI, and IDE).

**macOS / Linux:**

```bash
curl -fsSL https://raw.githubusercontent.com/sonesuke/docgraph/main/install.sh | bash
```

**Windows (PowerShell):**

```powershell
powershell -c "irm https://raw.githubusercontent.com/sonesuke/docgraph/main/install.ps1 | iex"
```

_Or build from source:_ `cargo install --path .`

### Step 2: Configure (`docgraph.toml`)

Create a `docgraph.toml` file in your project root to define your documentation architecture rules.

```toml
[node_types]
UC = { desc = "Use Case", template = "doc/templates/use_case.md" }
FR = { desc = "Functional Requirement", template = "doc/templates/functional.md" }
IF = { desc = "Interface", template = "doc/templates/interface.md" }

[references.FR]
rules = [
  { dir = "from", targets = [
    "UC",
  ], min = 1, desc = "Requirements must be derived from a use case" },
  { dir = "to", targets = [
    "MOD",
  ], min = 1, desc = "Requirements must be realized by a module" },
]

[references.IF]
rules = [
  { dir = "from", targets = [
    "UC",
  ], min = 1, desc = "Interfaces must be justified by a use case" },
  { dir = "from", targets = [
    "FR",
  ], min = 1, desc = "Interfaces must be defined by a functional requirement" },
]
```

### Step 3: Set Up Your Environment

Choose how you want to interact with `docgraph`:

#### Option A: Claude Code

1. In a Claude chat, install the official plugin:
   ```text
   /plugin marketplace add sonesuke/docgraph
   /plugin install docgraph-plugin@docgraph-claude-plugins
   ```
2. The AI agent will now automatically use the installed `docgraph` binary to analyze your documentation.

   **Example Claude Interaction:**

   > **You**: "Build a knowledge graph from the current directory." **Claude**: "Building knowledge graph... Done. Found
   > 15 nodes and 24 relationships."
   >
   > **You**: "What are the dependencies for UC_WRITE?" **Claude**: "UC_WRITE depends on ACT_USER and is realized by
   > IF_CLI_LINT."

#### Option B: VS Code Extension

1. Download `docgraph.vsix` from [GitHub Releases](https://github.com/sonesuke/docgraph/releases).
2. Install via Command Palette (`Extensions: Install from VSIX...`) or CLI:
   ```bash
   code --install-extension docgraph.vsix
   ```

#### Option C: Zed Editor Extension

1. Build the extension or download from releases:
   ```bash
   cd zed-extension && cargo build --release --target wasm32-wasip1
   ```
2. In Zed, run the command `zed: install dev extension`.
3. Select the `zed-extension` directory.
4. Create `.zed/settings.json` in your project root to enable the language server:

   ```json
   {
     "languages": {
       "Markdown": {
         "language_servers": ["docgraph"],
         "format_on_save": "on"
       }
     }
   }
   ```

5. **Note**: Ensure the workspace is trusted (exit Restricted Mode) to allow the language server to start.

#### Option D: Standard CLI

Use the commands directly in your terminal for validation and analysis:

```bash
# Validate the graph
docgraph check .

# Trace relationships
docgraph trace UC_LOGIN FR_EMAIL_LOGIN
```

#### Development Setup

If you are contributing to `docgraph`, please set up the pre-commit hooks to ensure code quality.

1. **Install prek**:
   ```bash
   cargo install prek
   ```
2. **Install Git Hooks**:
   ```bash
   prek install -f
   ```

This will automatically run `cargo fmt`, `clippy`, `prettier`, `test`, and `docgraph check` before every commit.

## CLI Commands Reference

- `check [path]`: Validate the graph for broken links and rule violations.
- `fmt [path]`: Automatically fix fixable formatting and lint issues.
- `list <query>`: Search for nodes matching a pattern.
- `trace <from> <to>`: Trace and visualize relationship paths.
- `describe <id>`: Show bidirectional relationships for a specific node.
- `lsp`: Start the Language Server for IDE support.

---

Contributions are welcome! If you're interested in helping improve `docgraph`, please check out our
**[Module View](./doc/architecture/view/module.md)** for an overview of the technical structure.

Detailed technical documentation and use cases can be found in the **[Documentation Overview](./doc/overview.md)**.
