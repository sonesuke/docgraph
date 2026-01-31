
# CLI Specifications

<a id="IF_CLI_LINT"></a>

## Command: `lint`

The `lint` command shall parse all Markdown files in the target directory, build the graph, and report any violations of validation rules.

<a id="IF_CLI_GRAPH"></a>

### Command: `graph`

The `graph` command shall output the graph structure in JSON format.

<a id="IF_CLI_LIST"></a>

### Command: `list`

The `list` command shall output spec blocks matching a specific query with their names.
The query can contain wildcards (`*` and `?`).
If no wildcards are present, the command performs a prefix match (forward match).

```bash
docgraph list "FR-*"
docgraph list FR
```

Output format:

```text
ID : Description
```

<a id="IF_CLI_TRACE"></a>

### Command: `trace`

The `trace` command shall find and display all paths between a start ID and target IDs matching a query.

```bash
docgraph trace <from> <to> [--direction <down|up>]
```

- `<from>`: The starting SpecBlock ID.
- `<to>`: Target ID or prefix (supports wildcards).
- `--direction`:
  - `down` (default): Follow outgoing links (references).
  - `up`: Follow incoming links (reverse references).

Output format:

```text
ID1 -> ID2 -> ID3
```

(Using `<-` for `up` direction)

<a id="IF_CLI_DESCRIBE"></a>

### Command: `describe`

The `describe` command shall display the details and relationships of a specific SpecBlock.

```bash
docgraph describe <id>
```

- `<id>`: The ID of the SpecBlock to describe.

Output format:

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

<a id="IF_CLI_LSP"></a>

### Command: `lsp`

The `lsp` command shall start a Language Server Protocol (LSP) server communicating over standard input and output.
It provides real-time diagnostics for document graph violations and broken links.
