# docgraph

[![CI](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml/badge.svg)](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml)
[![Security audit](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml/badge.svg)](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/badge/coverage-view-blue)](https://github.com/sonesuke/docgraph/actions/workflows/ci.yml)

A lint tool to build and verify directed graphs embedded in standard Markdown files.

## Overview

`docgraph` treats Markdown documents as nodes in a graph. By using HTML anchors (`<a id="..."></a>`) and standard Markdown links, you can define relationships (edges) between documents to ensure traceability across your entire documentation set—from requirements and use cases down to implementation details.

For a comprehensive guide on concepts, architecture, and current specifications, please refer to the **[Documentation Overview](./doc/overview.md)**.

## Key Features

- **Graph Validation**: Automated checks for broken links, duplicate IDs, and relationship rule violations.
- **Traceability Analysis**: Commands to search, trace, and describe node relationships.
- **LSP Support**: Interactive editing with real-time diagnostics, go-to-definition, hover information, and workspace-wide renaming.

## Installation

```bash
cargo install --path .
```

## Quick Start

### 1. Define a SpecBlock
In any Markdown file, define an ID followed by a heading:

```markdown
<a id="REQ_001"></a>
## User Authentication Requirement
The system must support email-based login.
```

### 2. Define a Relationship
Reference another ID within the same or another file using standard Markdown links:

```markdown
<a id="TC_001"></a>
## Login Test Case
Verify that the user can log in ([REQ_001](#REQ_001)).
```

### 3. Run Validation
```bash
docgraph check .
```

## CLI Commands

- `check [path]`: Validate the graph for broken links and rule violations.
- `list <query>`: Search for spec blocks matching a pattern (supports wildcards).
- `trace <from> <to>`: Visualize relationship paths between two nodes.
- `describe <id>`: Show detailed bidirectional relationships for a specific node.
- `lsp`: Start the Language Server for interactive editing support.
- `graph [path]`: Output the raw graph structure as JSON for external analysis.

## Configuration (`docgraph.toml`)

`docgraph` is highly configurable. You can define custom node prefixes and strict relationship rules to enforce your documentation architecture. Refer to the **[Configuration Specification](./doc/specs/config_specs.md)** for details.

```toml
[node_types]
REQ = { desc = "Requirement" }
TC = { desc = "Test Case" }

[references.REQ]
rules = [
  { dir = "to", targets = ["TC"], min = 1, desc = "Each requirement must be verified by a test case" }
]
```

---
## Contributing

Contributions are welcome! If you're interested in helping improve `docgraph`, please check out our **[Developer Guide](./doc/devel/guide.md)** for information on environment setup, architecture, and testing standards.

Detailed technical documentation and use cases can be found in the **[doc/](./doc/overview.md)** directory.
