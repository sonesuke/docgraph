# Test Specifications

<a id="RT-INT-LINT"></a>

## Lint Integration Test

Run `docgraph lint` (defined in [IF-CLI-LINT (Command: `lint`)](../specs/cli_specs.md#IF-CLI-LINT)) on a known defective graph and verify it reports errors.

Depends on: [IF-CLI-LINT (Command: `lint`)](../specs/cli_specs.md#IF-CLI-LINT)

<a id="RT-INT-GEN"></a>

## Gen Integration Test

Run `docgraph graph` (defined in [IF-CLI-GRAPH (Command: `graph`)](../specs/cli_specs.md#IF-CLI-GRAPH)) on a sample graph and verify the JSON output matches the expected structure.

Depends on: [IF-CLI-GRAPH (Command: `graph`)](../specs/cli_specs.md#IF-CLI-GRAPH)
