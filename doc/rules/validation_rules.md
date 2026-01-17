
# Validation Rules

<a id="RULE-UNIQUE"></a>

## Unique IDs

Every Document Block must have a unique identifier across the entire graph.

Depends on: [DOM-DOC (Document Block)](../model/domain_model.md#DOM-DOC)

<a id="RULE-VALID-REF"></a>

## Valid References

Every ID referenced in an Edge or inline Reference must exist in the Graph.

Depends on: [DOM-EDGE (Edge)](../model/domain_model.md#DOM-EDGE), [DOM-REF (Reference)](../model/domain_model.md#DOM-REF)
