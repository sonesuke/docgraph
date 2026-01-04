
# Validation Rules

```{document} Unique IDs
:id: RULE-UNIQUE
:kind: rule
:depends_on: DOM-DOC

Every Document Block must have a unique identifier across the entire graph.
```

```{document} Valid References
:id: RULE-VALID-REF
:kind: rule
:depends_on: DOM-EDGE
:depends_on: DOM-REF

Every ID referenced in an Edge or inline Reference must exist in the Graph.
```
