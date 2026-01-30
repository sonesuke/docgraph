# docgraph

A lint tool to build and verify directed graphs embedded in standard Markdown files.

## Overview

`docgraph` treats Markdown documents as nodes in a graph. By using HTML anchors (`<a id="..."></a>`) and standard Markdown links, you can define relationships (edges) between documents to ensure traceability across your entire documentation set.

For a comprehensive guide on the concepts, architecture, and documentation map, please refer to the **[Documentation Overview](./doc/overview.md)**.

## Installation

```bash
cargo install --path .
```

## Quick Start

### 1. Define a SpecBlock
In any Markdown file, define an ID followed by a heading:

```markdown
<a id="REQ-001"></a>
## User Authentication Requirement
The system must support email-based login.
```

### 2. Define a Relationship
Reference another ID within the same or another file:

```markdown
<a id="TC-001"></a>
## Login Test Case
Verify that the user can log in ([REQ-001](#REQ-001)).
Verifies: [REQ-001](#REQ-001)
```

### 3. Run Validation
```bash
docgraph check .
```

## CLI Commands

- `check [path]`: Validate the documentation graph for broken links and rule violations.
- `graph [path]`: Output the collected graph structure as JSON.
- `list <query>`: Search for spec blocks matching a pattern.
- `trace <from> <to>`: Find relationship paths between two nodes.
- `describe <id>`: Show detailed bidirectional relationships for a specific node.

## Configuration (`docgraph.toml`)

Refer to the **[Configuration Model](./doc/model/config_model.md)** for details on defining node types and relationship rules.

```toml
[node_types]
REQ = { desc = "Requirement" }

[references.REQ]
rules = [
  { dir = "to", targets = ["TC"], min = 1, desc = "Each requirement must be verified by a test case" }
]
```

---
Detailed technical documentation can be found in the **[doc/](./doc/overview.md)** directory.
