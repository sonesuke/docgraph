# docgraph

A lint tool to verify document graphs embedded in Markdown.
It uses standard Markdown with HTML anchors to ensure traceability between documents.

## Overview

`docgraph` parses HTML anchor tags (`<a id="..."></a>`) followed by headings within Markdown files and checks for duplicate IDs and missing references. It also extracts and verifies relationships (edges) defined as links within the block scope.

## Installation

```bash
cargo install --path .
```

## VS Code Extension (Preview)

A preview extension is available to render `{document}` blocks transparently in VS Code's Markdown Preview.

### Installation

1. Go to the **GitHub Releases** page of this repository.
2. Download the `.vsix` file (e.g., `docgraph-preview-x.y.z.vsix`).
3. In VS Code:
   - Open the **Extensions** view (`Cmd+Shift+X` or `Ctrl+Shift+X`).
   - Click the `...` (Views and More Actions) menu at the top right of the pane.
   - Select **Install from VSIX...**.
   - Choose the downloaded `.vsix` file.

## Usage

Check for errors in the current directory:

```bash
docgraph lint
```

Specify a directory to check:

```bash
docgraph lint ./spec
```

Output diagnostics in JSON format:

```bash
docgraph lint --json
```

Generate a graph JSON:

```bash
docgraph gen --json
```

List spec blocks matching a query:

```bash
# Prefix match (matches FR-ABC, FR-DEF, etc.)
docgraph list FR

# Wildcard match
docgraph list "FR-*"
```

Trace relationships between documents:

```bash
# Forward trace (outgoing references)
docgraph trace REQ-001 TC-

# Backward trace (incoming dependencies)
docgraph trace TC-001 REQ- --direction up
```

Describe a specific item and its relations:

```bash
docgraph describe REQ-001
```

## Markdown Syntax Support

This tool supports standard Markdown with HTML anchors for ID definition.

### Defining a SpecBlock

To define a block with an ID, place an HTML anchor tag on its own line, immediately followed by a heading.

```markdown
<a id="RQ-001"></a>

## Requirement: Login

This is the content of the requirement.
It can reference other documents using standard Markdown links: [another-id](#another-id).
```

### Scoped Relationships

Relationships are extracted from any link within the scope of a block (from its heading to the next anchor).

- **Explicit Edges**: Links like `Depends on: [another-id](#another-id)` or `Verifies: [TC-001](#TC-001)`.
- **Inline References**: Standard Markdown links within the text.

## Graph Structure Example

From Markdown files like the following, it recognizes and verifies the graph structure between documents.

**spec/reqs.md**

```markdown
<a id="RQ-AUTH-01"></a>

## Login Requirement

Users must be able to log in with an email address and password.
```

**spec/tests.md**

```markdown
<a id="TC-AUTH-01"></a>

## Login Test

1. Open the login page
2. Enter valid credentials
3. Verify that the dashboard is displayed ([RQ-AUTH-01](#RQ-AUTH-01))

Verifies: [RQ-AUTH-01](#RQ-AUTH-01)
```

In this example, a `verifies` edge of `TC-AUTH-01` -> `RQ-AUTH-01` is formed.
`docgraph` reports an error if `RQ-AUTH-01` does not exist or if `TC-AUTH-01` is defined duplicately.

### Linting Example

```bash
docgraph lint spec
```

If everything is correct, no output (exit code 0).
If `RQ-AUTH-01` is missing:

```text
error[E_BAD_REF] spec/tests.md:4:1: Unknown target 'RQ-AUTH-01' in edge :verifies
error[E_BAD_REF] spec/tests.md:8:43: Unknown ref target 'RQ-AUTH-01'
```

### Generation Example

```bash
docgraph gen spec --json
```

Output:

```json
[
  {
    "id": "RQ-AUTH-01",
    "kind": "requirement",
    "edges": [],
    "file_path": "spec/reqs.md",
    "line_start": 1,
    "line_end": 6
  },
  {
    "id": "TC-AUTH-01",
    "kind": "test",
    "edges": [
      {
        "edge_type": "verifies",
        "target_id": "RQ-AUTH-01",
        "line": 4
      }
    ],
    "file_path": "spec/tests.md",
    "line_start": 1,
    "line_end": 9
  }
]
```

## Configuration (`docgraph.toml`)

You can configure node types and relationship rules in `docgraph.toml`.

### Relationship Rules

Rules define constraints on incoming (`from`) and outgoing (`to`) relationships. You can add an optional `desc` field to provide a more helpful error message when a rule is violated.

```toml
[references.FR]
# Functional Requirement rules
rules = [
  # Must be derived from a UC or CON
  { dir = "from", targets = ["UC", "CON"], min = 1, desc = "Requirements must have a source use case or constraint" },
  # Can reference other types
  { dir = "to", targets = ["NFR", "ERR", "BB", "RT", "CC", "IF"], min = 0 }
]
```

If a rule with a `desc` is violated, the description will be displayed in the lint output to help identify the cause and resolution.
