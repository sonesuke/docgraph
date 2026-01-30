
# Configuration Model

<a id="DAT-CONFIG"></a>

## Config

The root configuration structure loaded from `docgraph.toml`.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `node_types` | `Map<String, NodeType>` | Definitions of allowed node types. |
| `graph` | `GraphConfig` | Global graph validation settings. |
| `references` | `Map<String, ReferenceConfig>` | Relationship rules organized by node type prefix. |

<a id="DAT-NODE-TYPE"></a>

## Node Type

Defines a category of spec blocks (e.g., `UC`, `FR`).

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `desc` | `String` | Human-readable description of the type. |

<a id="DAT-GRAPH-CONFIG"></a>

## Graph Config

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `strict_node_types` | `Boolean` | If true, all IDs must match a defined node type. |
| `strict_relations` | `Boolean` | If true, all outgoing edges must match defined rules. |
| `doc_types` | `List<String>` | Node types that are exempt from strict relation checks (e.g., `ACT`). |

<a id="DAT-REF-CONFIG"></a>

## Reference Config

Collection of rules for a specific node type prefix.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `rules` | `List<RuleConfig>` | List of validation rules. |

<a id="DAT-RULE-CONFIG"></a>

## Rule Config

Defines a constraint on relationships.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `dir` | `String` | Direction: `from` (incoming) or `to` (outgoing). |
| `targets` | `List<String>` | Allowed node type prefixes for the other end. |
| `min` | `Option<usize>` | Minimum number of relationships meeting the criteria. |
| `max` | `Option<usize>` | Maximum number of relationships meeting the criteria. |
| `desc` | `Option<String>` | Optional explanation displayed in lint error messages. |
