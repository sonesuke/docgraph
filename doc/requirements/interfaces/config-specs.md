<a id="IF-CONFIG"></a>

# docgraph.toml Configuration

The `docgraph.toml` file at the project root defines the validation rules and workspace behavior.

## Basic Structure

```toml
[graph]
strict_node_types = true
strict_relations  = true

[node_types]
REQ = { desc = "Requirement" }
TC  = { desc = "Test Case" }

[references.REQ]
rules = [
  { dir = "to", targets = ["TC"], min = 1, desc = "Requirements must be verified by a test case" }
]
```

---

<a id="IF-GRAPH-CONFIG"></a>

## 1. Global Graph Behavior (`[graph]`)

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `strict_node_types` | `Boolean` | If true, all IDs must match a defined node type in `[node_types]`. |
| `strict_relations` | `Boolean` | If true, all outgoing/incoming edges must match defined `[references]` rules. |
| `doc_types` | `List<String>` | Node types that are exempt from strict relation checks (typically for auxiliary docs). |

---

<a id="IF-NODE-TYPE"></a>

## 2. Defining Node Types (`[node_types]`)

Defines the allowed prefixes for spec blocks.

```toml
[node_types]
UC   = { desc = "Use Case" }
FR   = { desc = "Functional Requirement" }
```

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `desc` | `String` | Human-readable description of the type prefix. |

---

<a id="IF-REF-RULES"></a>

## 3. Relationship Rules (`[references]`)

Defines constraints on how different node types can connect.

```toml
[references.UC]
rules = [
  { dir = "to", targets = ["FR"], min = 1, desc = "Use cases must derive functional requirements" }
]
```

### Rule Configuration

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `dir` | `String` | Direction: `from` (incoming) or `to` (outgoing). |
| `targets` | `List<String>` | Allowed node type prefixes for the other end of the link. |
| `min` | `usize` (Optional) | Minimum number of relationships required. |
| `max` | `usize` (Optional) | Maximum number of relationships allowed. |
| `desc` | `String` (Optional) | Explanation displayed in lint error messages when the rule is violated. |
