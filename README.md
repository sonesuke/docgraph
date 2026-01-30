# docgraph

A lint tool to verify document graphs embedded in Markdown (MyST).
It uses a subset of MyST (Markedly Structured Text) to ensure traceability between documents.

## Overview

`docgraph` parses `{document}` blocks within Markdown files and checks for duplicate IDs and missing references. It also extracts and verifies relationships (edges) such as `verifies` or `depends_on`, which are often used in requirements specifications or test specifications.

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

## MyST Support (Subset)

This tool supports only the standard directive syntax of MyST. No custom extensions are used.

### Supported Directives

- `{document}`: Document definition block

````markdown
```{document} Requirements
:id: RQ-001
:kind: requirement
:verifies: TC-001
:depends_on: DEC-005

Details of the requirement are described here.
References to other documents are made in the format {ref}`another-id`.
```
````

### Supported Options

| Option           | Required | Description                                         | Multiple |
| :--------------- | :------- | :-------------------------------------------------- | :------- |
| `:id:`           | Yes      | Unique identifier for the document                  | No       |
| `:kind:`         | No       | Type of document (e.g., `requirement`, `test_case`) | No       |
| `:verifies:`     | No       | ID to verify (Edge)                                 | Yes      |
| `:depends_on:`   | No       | ID to depend on (Edge)                              | Yes      |
| `:derived_from:` | No       | ID derived from (Edge)                              | Yes      |

`<multiple>` can be expressed by specifying multiple IDs separated by spaces, or by writing the option line multiple times (planned implementation).

## Graph Structure Example

From Markdown files like the following, it recognizes and verifies the graph structure between documents.

**spec/reqs.md**

````markdown
```{document} Login Requirement
:id: RQ-AUTH-01
:kind: requirement

Users must be able to log in with an email address and password.
```
````

**spec/tests.md**

````markdown
```{document} Login Test
:id: TC-AUTH-01
:kind: test
:verifies: RQ-AUTH-01

1. Open the login page
2. Enter valid credentials
3. Verify that the dashboard is displayed ({ref}`RQ-AUTH-01`)
```
````

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
