
# CLI Specifications

<a id="IF-CLI-LINT"></a>

## Command: `lint`

The `lint` command shall parse all Markdown files in the target directory, build the graph, and report any violations of validation rules.

### Command: `graph`

The `graph` command shall output the graph structure in JSON format.

<a id="IF-CLI-LIST"></a>

### Command: `list`

The `list` command shall output spec blocks matching a specific query with their names.
The query can contain wildcards (`*` and `?`).
If no wildcards are present, the command performs a prefix match (forward match).
```
docgraph list "FR-*"
docgraph list FR
```
Output format:
```
ID : Description
```

<a id="IF-CLI-TRACE"></a>

### Command: `trace`

The `trace` command shall find and display all paths between a start ID and target IDs matching a query.
```
docgraph trace <from> <to> [--direction <down|up>]
```
- `<from>`: The starting SpecBlock ID.
- `<to>`: Target ID or prefix (supports wildcards).
- `--direction`:
  - `down` (default): Follow outgoing links (references).
  - `up`: Follow incoming links (reverse references).

Output format:
```
ID1 -> ID2 -> ID3
```
(Using `<-` for `up` direction)
