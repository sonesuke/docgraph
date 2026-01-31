# Analysis Use Cases

<a id="UC_GRAPH"></a>

## Generate Graph

The [ACT_USER (User)](../actors/users.md#ACT_USER) generates a JSON representation of the document graph.

**Steps:**

1. User navigates to the project directory.
2. User runs the graph command ([IF_CLI_GRAPH (Command: `graph`)](../requirements/interfaces/cli-specs.md#IF_CLI_GRAPH)).
3. User redirects the JSON output to a file or pipe.

Depends on: [ACT_USER (User)](../actors/users.md#ACT_USER), [UC_WRITE (Write Specifications)](./authoring.md#UC_WRITE)

<a id="UC_SEARCH"></a>

## Search Spec Blocks

The [ACT_USER (User)](../actors/users.md#ACT_USER) searches for spec blocks by ID or wildcards.

**Steps:**

1. User runs the list command ([IF_CLI_LSP (Command: `lsp`)](../requirements/interfaces/cli-specs.md#IF_CLI_LSP)).
2. User reviews the list of matching IDs and their descriptions.

Depends on: [ACT_USER (User)](../actors/users.md#ACT_USER), [UC_WRITE (Write Specifications)](./authoring.md#UC_WRITE)

<a id="UC_TRACE"></a>

## Trace Relationships

The [ACT_USER (User)](../actors/users.md#ACT_USER) visualizes paths between two spec blocks.

**Steps:**

1. User identifies a starting ID and a target ID/pattern.
2. User runs the trace command ([IF_CLI_TRACE (Command: `trace`)](../requirements/interfaces/cli-specs.md#IF_CLI_TRACE)).
3. User analyzes the displayed paths to understand dependencies or verified coverage.

Depends on: [ACT_USER (User)](../actors/users.md#ACT_USER), [UC_WRITE (Write Specifications)](./authoring.md#UC_WRITE)

<a id="UC_DESCRIBE"></a>

## Describe Spec Block

The [ACT_USER (User)](../actors/users.md#ACT_USER) inspects the detailed relationships of a specific block.

**Steps:**

1. User runs the describe command ([IF_CLI_DESCRIBE (Command: `describe`)](../requirements/interfaces/cli-specs.md#IF_CLI_DESCRIBE)).
2. User reviews the bidirectional relationship information (outgoing references and incoming dependencies).

Depends on: [ACT_USER (User)](../actors/users.md#ACT_USER), [UC_WRITE (Write Specifications)](./authoring.md#UC_WRITE)
