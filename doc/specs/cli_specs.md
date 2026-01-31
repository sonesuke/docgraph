# CLI Specifications

```{document} CLI Lint Specification
:id: SPEC-CLI-LINT
:kind: specification

The `lint` command shall parse all Markdown files in the target directory, build the graph, and report any violations of validation rules.
```

```{document} CLI Gen Specification
:id: SPEC-CLI-GEN
:kind: specification

The `gen` command shall output the graph structure in JSON format.
```

```{document} CLI List Specification
:id: SPEC-CLI-LIST
:kind: specification

The `list` command shall output spec blocks matching a specific query with their names.
The query can contain wildcards (`*` and `?`).
If no wildcards are present, the command performs a prefix match (forward match).

Usage:
```bash
docgraph list "FR-*"
docgraph list FR
```

Output format:
```text
ID : Description
```
```

```{document} CLI Trace Specification
:id: SPEC-CLI-TRACE
:kind: specification

The `trace` command shall find and display all paths between a start ID and target IDs matching a query.

Usage:
```bash
docgraph trace <from> <to> [--direction <down|up>]
```
- `<from>`: The starting SpecBlock ID.
- `<to>` : Target ID or prefix (supports wildcards).
- `--direction`:
  - `down` (default): Follow outgoing links (references).
  - `up`: Follow incoming links (reverse references).

Output format:
```text
ID1 -> ID2 -> ID3
```
(Using `<-` for `up` direction)
```

```{document} CLI Describe Specification
:id: SPEC-CLI-DESCRIBE
:kind: specification

The `describe` command shall display the details and relationships of a specific SpecBlock.

Usage:
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
...
```
```
