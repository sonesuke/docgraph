# Common Workspace Requirements

<a id="FR_CORE_VALID_REF"></a>

## Valid References

The core engine MUST validate that all internal links reference existing node IDs.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

---

<a id="FR_CORE_UNIQUE"></a>

## Unique Node IDs

The system MUST ensure that every node ID in the documentation remains unique across the entire workspace.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

---

<a id="FR_CORE_AUDIT"></a>

## Audit Logging

The system MUST record all validation results and structural changes in a persistent audit log for traceability and compliance.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

---

<a id="FR_CORE_AUTH"></a>

## Authentication

The system SHOULD provide mechanisms to authenticate users before allowing certain operations, especially when interacting with remote marketplaces.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

---

<a id="FR_CORE_TEMPLATE_VALIDATION"></a>

## Template Validation

The system MUST validate that documentation nodes conform to the structure defined in their associated Markdown templates.

**Validation Rules:**

- **Structure Matching**: The hierarchy of headers and lists in the document must strictly match the template.
- **Text Patterns**: Use `{...}` placeholders in templates to match variable content. Fixed text outside placeholders must match exactly.
- **Wildcards**: Supports `*` for ID matching and substring matching in links.
- **Optional Sections**: Use `(Optional)` in headers to mark sections that can be omitted.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

---

<a id="FR_CORE_CONFIGURATION"></a>

## Configuration

The system MUST allow users to customize behavior via a configuration file (`docgraph.toml`) at the project root.
The structure and available settings are defined in [IF_CONFIG](../../requirements/interfaces/interfaces.md#IF_CONFIG).

**Settings Requirements:**

The configuration MUST support the following settings:

| Attribute      | Type           | Description                                                  |
| :------------- | :------------- | :----------------------------------------------------------- |
| `doc_types`    | `List<String>` | Node types that are exempt from strict relation checks.      |
| `[node_types]` | `Table`        | Defines allowed prefixes for nodes and their templates.      |
| `[references]` | `Table`        | Defines constraints on how different node types can connect. |

**Structure Example:**

```toml
[graph]
doc_types = ["ACT", "DAT", "IF", "ADR"]

[node_types]
UC = { desc = "Use Case" }

[references.UC]
rules = [
  { dir = "to", targets = ["FR"], min = 1 }
]
```

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)
