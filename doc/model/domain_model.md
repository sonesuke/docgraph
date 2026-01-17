
# Domain Model

<a id="DOM-DOC"></a>

## Document Block

A structural unit in the graph, defined by an anchor heading.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `id` | `String` | Unique identifier (required). |
| `kind` | `Option<String>` | Type of document (e.g., `requirement`, `test`). |
| `edges` | `List<Edge>` | Outgoing typed relationships defined by options. |
| `refs` | `List<Reference>` | Outgoing references defined in the body text. |

<a id="DOM-EDGE"></a>

## Edge

A directed relationship originating from a Document Block.

Depends on: [DOM-DOC (Document Block)](#DOM-DOC)

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `edge_type` | `String` | Relationship type (e.g., `verifies`, `depends_on`). |
| `target_id` | `String` | The ID of the target block. |

<a id="DOM-GRAPH"></a>

## Graph

The collection of all Document Blocks and Edges.

Depends on: [DOM-DOC (Document Block)](#DOM-DOC)

<a id="DOM-REF"></a>

## Reference

An inline reference to another Document Block using the Markdown link syntax.

Depends on: [DOM-DOC (Document Block)](#DOM-DOC)

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `target_id` | `String` | The ID of the referenced block. |
