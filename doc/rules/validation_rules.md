
# Validation Rules

<a id="CON-UNIQUE"></a>

## Unique IDs

Every Document Block must have a unique identifier across the entire graph.

Depends on: [DAT-DOC (Document Block)](../model/domain_model.md#DAT-DOC)

<a id="CON-VALID-REF"></a>

## Valid References

Every ID referenced in an Edge or inline Reference must exist in the Graph.

Depends on: [DAT-EDGE (Edge)](../model/domain_model.md#DAT-EDGE), [DAT-REF (Reference)](../model/domain_model.md#DAT-REF)
