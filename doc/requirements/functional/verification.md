# Verification Requirements

<a id="FR_UNIQUE"></a>

## Unique IDs

Every Document Block must have a unique identifier across the entire graph.

**Realized by:**

- [IF_CLI_LINT (Command: `lint`)](../interfaces/cli-specs.md#IF_CLI_LINT)
- [MOD_CORE (Core Modules)](../../architecture/view/module.md#MOD_CORE)
- [MOD_CLI (CLI Modules)](../../architecture/view/module.md#MOD_CLI)

<a id="FR_VALID_REF"></a>

## Valid References

Every ID referenced in an Edge or inline Reference must exist in the Graph.

**Realized by:**

- [IF_CLI_LINT (Command: `lint`)](../interfaces/cli-specs.md#IF_CLI_LINT)
- [MOD_CORE (Core Modules)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_STRICT_NODES"></a>

## Strict Node Types

If `strict_node_types` is enabled, all SpecBlock IDs must start with a recognized node type prefix defined in the configuration.

**Depends on:**

- [IF_CONFIG (docgraph.toml Configuration)](../interfaces/config-specs.md#IF_CONFIG)

**Realized by:**

- [IF_CLI_LINT (Command: `lint`)](../interfaces/cli-specs.md#IF_CLI_LINT)
- [MOD_CORE (Core Modules)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_RELATION_RULES"></a>

## Relation Rules

If `strict_relations` is enabled, all outgoing Edges from a SpecBlock must match one of the allowed target types defined for its node type.
Minimum and maximum counts for both incoming (`from`) and outgoing (`to`) relationships are enforced.

**Depends on:**

- [IF_CONFIG (docgraph.toml Configuration)](../interfaces/config-specs.md#IF_CONFIG)

**Realized by:**

- [IF_CLI_LINT (Command: `lint`)](../interfaces/cli-specs.md#IF_CLI_LINT)
- [MOD_CORE (Core Modules)](../../architecture/view/module.md#MOD_CORE)
