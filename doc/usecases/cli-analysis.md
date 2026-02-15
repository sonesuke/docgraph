# CLI Analysis Use Cases

<a id="UC_CLI_ANALYSIS"></a>

## CLI Traceability Analysis

The developer runs the `docgraph` CLI to analyze the documentation graph.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_CLI (Command Line Interface)](../requirements/interfaces/interfaces.md#IF_CLI)

### Flow

1. Developer opens a terminal in the project root.
2. Developer runs `docgraph check .`.
3. CLI performs structural validation and reports errors.
