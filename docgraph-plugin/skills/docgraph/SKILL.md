---
description: Use Docgraph CLI to check document consistency, analyze, and manipulate graph structures.
---

# Docgraph CLI Usage

Docgraph is a tool for managing dependencies and traceability in Markdown documents using HTML anchor tags.

## Basic Syntax

```bash
docgraph <COMMAND> [OPTIONS]
```

## Subcommands

### `check` - Document Consistency Check
Checks for duplicate identifiers, broken links, and dependency rule violations within the documents.

```bash
docgraph check [PATH] [OPTIONS]
```
- `PATH`: Directory to scan (default: `.` )
- `--json`: Output results in JSON format
- `--fix`: Automatically fix fixable issues (e.g., formatting)
- `--rule <RULE>`: Run only specific rules (can be specified multiple times)

### `fmt` - Document Formatting
Automatically fixes fixable errors, similar to `check --fix`.

```bash
docgraph fmt [PATH] [OPTIONS]
```
- `--rule <RULE>`: Apply only specific rules

### `list` - List Elements
Lists elements matching a specific pattern.

```bash
docgraph list <QUERY> [PATH]
```
- `QUERY`: Search pattern (e.g., `FR-*`)

### `trace` - Trace Relationships
Traces dependencies between two elements.

```bash
docgraph trace <FROM> <TO> [PATH] [OPTIONS]
```
- `FROM`: Source ID to start tracing from
- `TO`: Target ID or prefix
- `--direction <DIRECTION>`: Direction of trace (`down`: forward/derive, `up`: backward/depends on) [default: `down`]

### `describe` - Show Detailed Information
Displays detailed information about a specific element and its direct relationships.

```bash
docgraph describe <ID> [PATH]
```
- `ID`: Target ID to describe

### `graph` - Generate Graph Data
Outputs the element and relationship data for the entire workspace as graph data.

```bash
docgraph graph [PATH]
```

### `rule` - Show Rule Information
Displays a list of available linters or details for a specific rule.

```bash
docgraph rule [RULE_NAME]
```

### `lsp` - Start LSP Server
Starts the Language Server for IDE integration.

```bash
docgraph lsp
```

### `type` - Show Node Type Information
Displays node type information from the configuration file.

```bash
docgraph type              # List all node types with descriptions
docgraph type <TYPE_ID>    # Show type details and rules
```
- `TYPE_ID`: Type ID to show details for (e.g., `FR`, `NFR`)