
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
