# Functional Requirements: CLI

<a id="FR_CLI_LINT"></a>

## Lint Command

The `lint` command shall parse all Markdown files in the target directory, build the graph, and report any violations of
validation rules via the [Command Line Interface](../../requirements/interfaces/interfaces.md#IF_CLI).

### Derived from

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)

<a id="FR_CLI_GRAPH"></a>

## Graph Command

The `graph` command shall output the graph structure in JSON format.

### Derived from

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)

<a id="FR_CLI_LIST"></a>

## List Capability

The `list` capability shall output nodes matching a specific query with their names. This is achieved using the `query`
command.

**Usage:**

```bash
docgraph query "MATCH (n) WHERE n.id =~ 'FR-*' RETURN n.id, n.name"
```

**Output format:**

```text
┌────────┬─────────────┐
│ n.id   ┆ n.name      │
╞════════╪═════════════╡
│ FR-001 ┆ ...         │
└────────┴─────────────┘
```

### Derived from

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)

<a id="FR_CLI_TRACE"></a>

## Trace Capability

The `trace` capability shall find and display all paths between a start ID and target IDs matching a query. This is
achieved using the `query` command.

**Usage:**

```bash
docgraph query "MATCH p=(src)-[*]->(dst) WHERE src.id = 'A' AND dst.id = 'B' RETURN p"
```

### Derived from

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)

<a id="FR_CLI_DESCRIBE"></a>

## Describe Command

The `describe` command shall display the details and relationships of a specific Node.

**Usage:**

```bash
docgraph describe <id>
```

- `<id>`: The ID of the Node to describe.

**Output format:**

```text
ID: Name
ID references to
target_id: target_name
...

The following IDs are depends on ID
source_id: source_name
source_id: source_name
...
```

### Derived from

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)

<a id="FR_CLI_TYPE"></a>

## Type Command

The `type` command shall display node type information from the configuration file.

**Usage:**

```bash
docgraph type             # List all node types with descriptions
docgraph type <type-id>   # Show type details and rules
```

- Without arguments: Lists all defined node types with their descriptions.
- With `<type-id>`: Shows the type's description and reference rules.

**Output format (list):**

```text
Node Types:

  FR - Functional Requirement
  NFR - Non-Functional Requirement
  ...
```

**Output format (details):**

```text
Type: FR
Description: Functional Requirement

Rules:
  from [UC, CON] min=1 max=-: Functional requirements are derived from business needs
  to [MOD] min=1 max=-: Each functional requirement must be realized by at least one module
```

### Derived from

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)

<a id="FR_CLI_VERSION"></a>

## Version Command

The `version` command shall display the current version of the `docgraph` tool.

### Derived from

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)

<a id="FR_CLI_HELP"></a>

## Help Command

The `help` command shall display usage information for `docgraph` and its subcommands.

### Derived from

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)

<a id="FR_CLI_QUERY"></a>

## Query Command

The `query` command shall execute Cypher-like queries against the documentation graph to retrieve nodes and
relationships matching specific patterns.

**Usage:**

```bash
docgraph query "MATCH (n:UC) WHERE n.name CONTAINS 'Login' RETURN n.id" [--format <table|json>]
```

- Query string: A Cypher-like pattern matching string.
  - Supports `MATCH` clause with node and relationship patterns (e.g., `(n:Type)`, `(a)-[r]->(b)`).
  - Supports `WHERE` clause with operators: `=`, `<>`, `<`, `>`, `<=`, `>=`, `CONTAINS`, `AND`, `OR`.
  - Supports `RETURN` clause to select specific properties (`n.id`, `n.file`, etc.).
- `--format`: Output format.
  - `table` (default): Tidy ASCII table.
  - `json`: Structured JSON output.

**Supported Properties:**

Returning the node variable itself (e.g., `RETURN n`) extends to all available properties.

- `id`: Node ID.
- `name`: Node name (Markdown heading).
- `type` / `node_type`: Node type identifier.
- `file`: Relative file path.
- `line`: Start line number.
- `content`: Raw Markdown content.

**Output format (table):**

```text
┌────────┬─────────────┐
│ n.id   ┆ n.name      │
╞════════╪═════════════╡
│ UC_001 ┆ User Login  │
└────────┴─────────────┘
```

**Output format (json):**

```json
[
  {
    "n.id": "UC_001",
    "n.name": "User Login"
  }
]
```

### Derived from

- [UC_CLI_ANALYSIS (CLI Traceability Analysis)](../../usecases/cli-analysis.md#UC_CLI_ANALYSIS)
