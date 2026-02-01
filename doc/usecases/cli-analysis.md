# CLI-Based Analysis Use Cases

<a id="UC_CLI_ANALYSIS"></a>

## CLI-Based Documentation Analysis

The [ACT_USER (User)](../actors/users.md#ACT_USER) and [ACT_CI (CI System)](../actors/systems.md#ACT_CI) use the CLI to analyze, validate, and query the documentation graph.

**Capabilities:**

- **Linting**: Checks for errors in the document graph (duplicate IDs, missing references).
- **Automated Checks**: CI automatically verifies the document graph on every push.
- **Graph Generation**: Generates a JSON representation of the document graph.
- **Search**: Searches for nodes by ID or wildcards.
- **Trace**: Visualizes paths between nodes to understand dependencies.
- **Describe**: Inspects detailed relationships of a specific block.

**Derives:**

- [IF_CLI (Command Line Interface)](../requirements/interfaces/interfaces.md#IF_CLI)
- [FR_CLI_LINT (Lint Command)](../requirements/functional/cli.md#FR_CLI_LINT)
- [FR_CLI_GRAPH (Graph Command)](../requirements/functional/cli.md#FR_CLI_GRAPH)
- [FR_CLI_LIST (List Command)](../requirements/functional/cli.md#FR_CLI_LIST)
- [FR_CLI_TRACE (Trace Command)](../requirements/functional/cli.md#FR_CLI_TRACE)
- [FR_CLI_DESCRIBE (Describe Command)](../requirements/functional/cli.md#FR_CLI_DESCRIBE)
- [FR_CORE_UNIQUE (Unique IDs)](../requirements/functional/core.md#FR_CORE_UNIQUE)
- [FR_CORE_VALID_REF (Valid References)](../requirements/functional/core.md#FR_CORE_VALID_REF)
