# Verification Requirements

<a id="FR-UNIQUE"></a>

## Unique IDs

Every Document Block must have a unique identifier across the entire graph.

Realized by: [IF-CLI-LINT (Command: `lint`)](../interfaces/cli_specs.md#IF-CLI-LINT), [TEST-INT-LINT (Lint Integration Test)](../../tests/integration_metrics.md#TEST-INT-LINT)

<a id="FR-VALID-REF"></a>

## Valid References

Every ID referenced in an Edge or inline Reference must exist in the Graph.

Realized by: [IF-CLI-LINT (Command: `lint`)](../interfaces/cli_specs.md#IF-CLI-LINT), [TEST-INT-LINT (Lint Integration Test)](../../tests/integration_metrics.md#TEST-INT-LINT)

<a id="FR-STRICT-NODES"></a>

## Strict Node Types

If `strict_node_types` is enabled, all SpecBlock IDs must start with a recognized node type prefix defined in the configuration.

Depends on: [IF-CONFIG (docgraph.toml Configuration)](../interfaces/config_specs.md#IF-CONFIG)
Realized by: [IF-CLI-LINT (Command: `lint`)](../interfaces/cli_specs.md#IF-CLI-LINT), [TEST-INT-LINT (Lint Integration Test)](../../tests/integration_metrics.md#TEST-INT-LINT)

<a id="FR-RELATION-RULES"></a>

## Relation Rules

If `strict_relations` is enabled, all outgoing Edges from a SpecBlock must match one of the allowed target types defined for its node type.
Minimum and maximum counts for both incoming (`from`) and outgoing (`to`) relationships are enforced.

Depends on: [IF-CONFIG (docgraph.toml Configuration)](../interfaces/config_specs.md#IF-CONFIG)
Realized by: [IF-CLI-LINT (Command: `lint`)](../interfaces/cli_specs.md#IF-CLI-LINT), [TEST-INT-GEN (Gen Integration Test)](../../tests/integration_metrics.md#TEST-INT-GEN)
