# Verification Requirements

```{document} Unique IDs
:id: FR-UNIQUE
:kind: requirement
:depends_on: DAT-DOC

Every Document Block must have a unique identifier across the entire graph.
Realized by: {ref}`SPEC-CLI-LINT`, {ref}`TEST-INT-LINT`
```

```{document} Valid References
:id: FR-VALID-REF
:kind: requirement
:depends_on: DAT-EDGE DAT-REF

Every ID referenced in an Edge or inline Reference must exist in the Graph.
Realized by: {ref}`SPEC-CLI-LINT`, {ref}`TEST-INT-LINT`
```

```{document} Strict Node Types
:id: FR-STRICT-NODES
:kind: requirement

If `strict_node_types` is enabled, all SpecBlock IDs must start with a recognized node type prefix defined in the configuration.
Realized by: {ref}`SPEC-CLI-LINT`, {ref}`TEST-INT-LINT`
```

```{document} Relation Rules
:id: FR-RELATION-RULES
:kind: requirement

If `strict_relations` is enabled, all outgoing Edges from a SpecBlock must match one of the allowed target types defined for its node type.
Minimum and maximum counts for both incoming (`from`) and outgoing (`to`) relationships are enforced.
Realized by: {ref}`SPEC-CLI-LINT`, {ref}`TEST-INT-GEN`
```
