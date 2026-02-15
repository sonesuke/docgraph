# CLI Analysis Use Cases

<a id="UC_CLI_ANALYSIS"></a>

## CLI Traceability Analysis

The developer runs the `docgraph` CLI to analyze the documentation graph.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_CLI (Command Line Interface)](../requirements/interfaces/interfaces.md#IF_CLI)

### Requirements

- [FR_CLI_LINT (Lint Command)](../requirements/functional/cli.md#FR_CLI_LINT) Core CLI action for validating
  documentation
- [FR_CLI_GRAPH (Graph Command)](../requirements/functional/cli.md#FR_CLI_GRAPH) Visualizing traceability relationships
- [FR_CORE_AUDIT (Audit Logging)](../requirements/functional/core.md#FR_CORE_AUDIT) Tracking CLI usage for compliance
  monitoring
- [FR_CLI_TRACE (Trace Command)](../requirements/functional/cli.md#FR_CLI_TRACE) Analyzing dependency paths between
  nodes (using `query`)
- [FR_CLI_QUERY (Query Command)](../requirements/functional/cli.md#FR_CLI_QUERY) Advanced pattern matching and graph
  analysis
- [FR_CLI_DESCRIBE (Describe Command)](../requirements/functional/cli.md#FR_CLI_DESCRIBE) Showing detailed metadata for
  a specific node
- [FR_CLI_TYPE (Type Command)](../requirements/functional/cli.md#FR_CLI_TYPE) Filtering nodes by their defined types
- [FR_CLI_LIST (List Command)](../requirements/functional/cli.md#FR_CLI_LIST) Listing all nodes found in the workspace
  (using `query`)
- [FR_CLI_VERSION (Version Command)](../requirements/functional/cli.md#FR_CLI_VERSION) Displaying the current version of
  the tool
- [FR_CLI_HELP (Help Command)](../requirements/functional/cli.md#FR_CLI_HELP) Providing usage guidance for CLI commands

### Flow

1. Developer opens a terminal in the project root.
2. Developer runs `docgraph check .`.
3. CLI performs structural validation and reports errors.
