# Functional Requirements: CLI

<a id="FR_CLI_LINT"></a>

## Lint Command

The `lint` command shall parse all Markdown files in the target directory, build the graph, and report any violations of
validation rules via the [Command Line Interface](../../requirements/interfaces/interfaces.md#IF_CLI).

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

<a id="FR_CLI_GRAPH"></a>

## Graph Command

The `graph` command shall output the graph structure in JSON format.

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)
- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_CLI_LIST"></a>

## List Command

The `list` command shall output nodes matching a specific query with their names.

The query can contain wildcards (`*` and `?`). If no wildcards are present, the command performs a prefix match (forward
match).

**Usage:**

```bash
docgraph list "FR-*"
docgraph list FR
```

**Output format:**

```text
ID : Description
```

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

<a id="FR_CLI_TRACE"></a>

## Trace Command

The `trace` command shall find and display all paths between a start ID and target IDs matching a query.

**Usage:**

```bash
docgraph trace <from> <to> [--direction <down|up>]
```

- `<from>`: The starting Node ID.
- `<to>`: Target ID or prefix (supports wildcards).
- `--direction`:
  - `down` (default): Follow outgoing links (references).
  - `up`: Follow incoming links (reverse references).

**Output format:**

```text
ID1 -> ID2 -> ID3
```

(Using `<-` for `up` direction)

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

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

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

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

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

<a id="FR_CLI_VERSION"></a>

## Version Command

The `version` command shall display the current version of the `docgraph` tool.

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)

<a id="FR_CLI_HELP"></a>

## Help Command

The `help` command shall display usage information for `docgraph` and its subcommands.

### Realized by

- [MOD_CLI (CLI Application)](../../architecture/view/module.md#MOD_CLI)
