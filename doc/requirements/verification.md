# Verification Requirements

<a id="FR-UNIQUE"></a>

## Unique IDs

Every Document Block must have a unique identifier across the entire graph.

Depends on: [DAT-DOC (Document Block)](../model/domain_model.md#DAT-DOC)
Realized by: [IF-CLI-LINT (Command: `lint`)](../specs/cli_specs.md#IF-CLI-LINT)

<a id="FR-VALID-REF"></a>

## Valid References

Every ID referenced in an Edge or inline Reference must exist in the Graph.

Depends on: [DAT-EDGE (Edge)](../model/domain_model.md#DAT-EDGE), [DAT-REF (Reference)](../model/domain_model.md#DAT-REF)
Realized by: [IF-CLI-LINT (Command: `lint`)](../specs/cli_specs.md#IF-CLI-LINT)

<a id="FR-STRICT-NODES"></a>

## Strict Node Types

If `strict_node_types` is enabled, all SpecBlock IDs must start with a recognized node type prefix defined in the configuration.

Depends on: [DAT-NODE-TYPE (Node Type)](../model/config_model.md#DAT-NODE-TYPE)
Realized by: [IF-CLI-LINT (Command: `lint`)](../specs/cli_specs.md#IF-CLI-LINT)

<a id="FR-RELATION-RULES"></a>

## Relation Rules

If `strict_relations` is enabled, all outgoing Edges from a SpecBlock must match one of the allowed target types defined for its node type.
Minimum and maximum counts for both incoming (`from`) and outgoing (`to`) relationships are enforced.

Depends on: [DAT-RULE-CONFIG (Rule Config)](../model/config_model.md#DAT-RULE-CONFIG)
Realized by: [IF-CLI-LINT (Command: `lint`)](../specs/cli_specs.md#IF-CLI-LINT)
