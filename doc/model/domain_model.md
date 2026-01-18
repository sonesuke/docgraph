
# Domain Model

<a id="DAT-DOC"></a>

## Document Block

A structural unit in the graph, defined by an anchor heading.

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `id` | `String` | Unique identifier (required). |
| `kind` | `Option<String>` | Type of document (e.g., `requirement`, `test`). |
| `edges` | `List<Edge>` | Outgoing typed relationships defined by options. |
| `refs` | `List<Reference>` | Outgoing references defined in the body text. |

<a id="DAT-EDGE"></a>

## Edge

A directed relationship originating from a Document Block.

Depends on: [DAT-DOC (Document Block)](#DAT-DOC)

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `edge_type` | `String` | Relationship type (e.g., `verifies`, `depends_on`). |
| `target_id` | `String` | The ID of the target block. |

<a id="DAT-GRAPH"></a>

## Graph

The collection of all Document Blocks and Edges.

Depends on: [DAT-DOC (Document Block)](#DAT-DOC)

<a id="DAT-REF"></a>

## Reference

An inline reference to another Document Block using the Markdown link syntax.

Depends on: [DAT-DOC (Document Block)](#DAT-DOC)

| Attribute | Type | Description |
| :--- | :--- | :--- |
| `target_id` | `String` | The ID of the referenced block. |
