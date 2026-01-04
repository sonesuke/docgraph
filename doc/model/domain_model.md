
# Domain Model

```{document} Document Block
:id: DOM-DOC
:kind: model

A structural unit in the graph, defined by a MyST directive block.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `id` | `String` | Unique identifier (required). |
| `kind` | `Option<String>` | Type of document (e.g., `requirement`, `test`). |
| `edges` | `List<Edge>` | Outgoing typed relationships defined by options. |
| `refs` | `List<Reference>` | Outgoing references defined in the body text. |
```

```{document} Edge
:id: DOM-EDGE
:kind: model
:depends_on: DOM-DOC

A directed relationship originating from a Document Block.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `edge_type` | `String` | Relationship type (e.g., `verifies`, `depends_on`). |
| `target_id` | `String` | The ID of the target block. |
```

```{document} Graph
:id: DOM-GRAPH
:kind: model
:depends_on: DOM-DOC

The collection of all Document Blocks and Edges.
```

```{document} Reference
:id: DOM-REF
:kind: model
:depends_on: DOM-DOC

An inline reference to another Document Block using the `{ref}` syntax.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `target_id` | `String` | The ID of the referenced block. |
```
