
# Core Use Cases

<a id="UC-WRITE"></a>

## Write Specifications

The [ACT-USER (User)](../actors/system_users.md#ACT-USER) writes specifications in Markdown using anchor heading format.

Depends on: [ACT-USER (User)](../actors/system_users.md#ACT-USER)

<a id="UC-LINT"></a>

## Lint Documents

The [ACT-USER (User)](../actors/system_users.md#ACT-USER) checks for errors in the document graph.

**Steps:**

1. User navigates to the project directory.
2. User runs the lint command ([SPEC-CLI-LINT (Command: `lint`)](../specs/cli_specs.md#SPEC-CLI-LINT)).
3. User reviews any error messages regarding duplicate IDs or missing references.

Depends on: [ACT-USER (User)](../actors/system_users.md#ACT-USER), [UC-WRITE (Write Specifications)](#UC-WRITE)

<a id="UC-GRAPH"></a>

## Generate Graph

The [ACT-USER (User)](../actors/system_users.md#ACT-USER) generates a JSON representation of the document graph.

**Steps:**

1. User navigates to the project directory.
2. User runs the graph command ([SPEC-CLI-GRAPH (Command: `graph`)](../specs/cli_specs.md#SPEC-CLI-GRAPH)).
3. User redirects the JSON output to a file or pipe.

Depends on: [ACT-USER (User)](../actors/system_users.md#ACT-USER), [UC-WRITE (Write Specifications)](#UC-WRITE)

<a id="UC-CI-CHECK"></a>

## Automate Checks

The [ACT-CI (CI System)](../actors/system_users.md#ACT-CI) automatically verifies the document graph on every push.

Depends on: [ACT-CI (CI System)](../actors/system_users.md#ACT-CI), [UC-WRITE (Write Specifications)](#UC-WRITE)

<a id="UC-BUILD-KNOWLEDGE"></a>

## Build Knowledge Graph

The [ACT-AGENT (AI Agent)](../actors/system_users.md#ACT-AGENT) builds a GraphRAG knowledge base from the generated JSON graph.

Depends on: [ACT-AGENT (AI Agent)](../actors/system_users.md#ACT-AGENT), [UC-GRAPH (Generate Graph)](#UC-GRAPH)

<a id="UC-SUGGEST-USAGE"></a>

## Suggest Usage

The [ACT-AGENT (AI Agent)](../actors/system_users.md#ACT-AGENT) suggests how to use the system based on the constructed knowledge.

Depends on: [ACT-AGENT (AI Agent)](../actors/system_users.md#ACT-AGENT), [UC-BUILD-KNOWLEDGE (Build Knowledge Graph)](#UC-BUILD-KNOWLEDGE)
