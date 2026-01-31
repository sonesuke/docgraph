# docgraph

[![CI](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml/badge.svg)](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml)
[![Security audit](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml/badge.svg)](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/badge/coverage-view-blue)](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml)

A powerful lint tool and graph generator designed to build and verify directed graphs embedded in standard Markdown files. `docgraph` ensures your documentation is consistent, traceable, and AI-ready.

## Overview

`docgraph` treats Markdown blocks as nodes in a graph. By using HTML anchors (`<a id="..."></a>`) and standard Markdown links, you can define structured relationships (edges) between documents. This enables automated traceability across your entire documentation suite—from business actors and use cases down to technical requirements and architecture decisions.

For a comprehensive guide on concepts, architecture, and specifications, please refer to the **[Documentation Overview](./doc/overview.md)**.

## Key Features

- **Graph Validation**: Automated checks for broken links, duplicate IDs, and relationship rule violations defined in `docgraph.toml`.
- **Traceability Analysis**: Tools to search, trace, and describe complex node relationships.
- **AI-Agent Ready**: Built-in support for GraphRAG knowledge construction. AI Agents can consume `docgraph` outputs to assist in documentation and development workflows.
- **Modern IDE Integration**: A dedicated VS Code plugin with full LSP support.

## Quick Start: Claude Code (AI-First)

The fastest way to start using `docgraph` is via Claude Desktop.

### 1. Install the Plugin

In a Claude chat, run the following commands to add the official marketplace and install the plugin:

```text
/plugin marketplace add sonesuke/docgraph
/plugin install docgraph-plugin@docgraph-claude-plugins
```

### 2. Use with AI Agent

Once installed, the AI agent can automatically build a knowledge graph of your project and assist you:

> "Building knowledge graph... Done. Found 15 spec blocks and 24 relationships."
>
> **You**: "What are the dependencies for UC_WRITE?"
> **Claude**: "UC_WRITE depends on ACT_USER and is realized by IF_CLI_LINT."

## Quick Start: CLI

For local development and CI/CD integration.

### 1. Installation

```bash
cargo install --path .
```

### 2. Define a SpecBlock

In any Markdown file, define an ID followed by a heading:

```markdown
<a id="UC_LOGIN"></a>

## User Login

The system shall allow users to log in to their accounts ([FR_EMAIL_LOGIN](#FR_EMAIL_LOGIN)).
```

### 3. Define a Relationship

Reference another ID using standard Markdown links. Functional Requirements must be realized by a Module:

```markdown
<a id="FR_EMAIL_LOGIN"></a>

## Email Login Requirement

Users must be able to log in using their email address.

**Realized by:**

- [MOD_CORE (Core Modules)](./doc/architecture/view/module.md#MOD_CORE)
```

### 4. Run Validation

```bash
docgraph check .
```

## CLI Commands

- `check [path]`: Validate the graph for broken links and rule violations.
- `fmt [path]`: Automatically fix fixable formatting and lint issues.
- `list <query>`: Search for spec blocks matching a pattern (supports wildcards).
- `trace <from> <to>`: Trace and visualize relationship paths between nodes.
- `describe <id>`: Show detailed bidirectional relationships for a specific node.
- `graph [path]`: Generate raw graph data as JSON for AI agents or external analysis.
- `lsp`: Start the Language Server for interactive IDE support.

## Configuration (`docgraph.toml`)

`docgraph` is highly configurable. Enforce your documentation architecture by defining strict node types and relationship rules. Refer to the **[Configuration Specification](./doc/requirements/interfaces/config-specs.md)** for details.

```toml
[node_types]
UC = { desc = "Use Case" }
FR = { desc = "Functional Requirement" }

[references.FR]
rules = [
  { dir = "from", targets = ["UC"], min = 1, desc = "Requirements must be derived from a use case" }
]
```

---

## Contributing

Contributions are welcome! If you're interested in helping improve `docgraph`, please check out our **[Developer Guide](./doc/architecture/view/guide.md)** for more information.

Detailed technical documentation and use cases can be found in the **[doc/](./doc/overview.md)** directory.
