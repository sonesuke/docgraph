
# Core Use Cases

<a id="UC-WRITE"></a>

## Write Specifications

The [ACT-DEV (Developer)](../actors/system_users.md#ACT-DEV) writes specifications in Markdown using anchor heading format ([ADR-MARKDOWN-FORMAT (Choice of Plain Markdown and HTML Anchors)](../decisions/markdown_format.md#ADR-MARKDOWN-FORMAT)).

Depends on: [ACT-DEV (Developer)](../actors/system_users.md#ACT-DEV), [ADR-MARKDOWN-FORMAT (Choice of Plain Markdown and HTML Anchors)](../decisions/markdown_format.md#ADR-MARKDOWN-FORMAT)

<a id="UC-LINT"></a>

## Lint Documents

The [ACT-USER (User)](../actors/system_users.md#ACT-USER) checks for errors in the document graph.

**Steps:**

1. User navigates to the project directory.
2. User runs the lint command ([IF-CLI-LINT (Command: `lint`)](../specs/cli_specs.md#IF-CLI-LINT)).
3. User reviews any error messages regarding duplicate IDs or missing references.

Depends on: [ACT-USER (User)](../actors/system_users.md#ACT-USER), [UC-WRITE (Write Specifications)](#UC-WRITE), [FR-UNIQUE (Unique IDs)](../requirements/verification.md#FR-UNIQUE), [FR-VALID-REF (Valid References)](../requirements/verification.md#FR-VALID-REF), [FR-STRICT-NODES (Strict Node Types)](../requirements/verification.md#FR-STRICT-NODES), [FR-RELATION-RULES (Relation Rules)](../requirements/verification.md#FR-RELATION-RULES)

<a id="UC-GRAPH"></a>

## Generate Graph

The [ACT-USER (User)](../actors/system_users.md#ACT-USER) generates a JSON representation of the document graph.

**Steps:**

1. User navigates to the project directory.
2. User runs the graph command ([IF-CLI-GRAPH (Command: `graph`)](../specs/cli_specs.md#IF-CLI-GRAPH)).
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

<a id="UC-SEARCH"></a>

## Search Spec Blocks

The [ACT-USER (User)](../actors/system_users.md#ACT-USER) searches for spec blocks by ID or wildcards.

**Steps:**

1. User runs the list command ([IF-CLI-LIST (Command: `list`)](../specs/cli_specs.md#IF-CLI-LIST)).
2. User reviews the list of matching IDs and their descriptions.

Depends on: [ACT-USER (User)](../actors/system_users.md#ACT-USER), [UC-WRITE (Write Specifications)](#UC-WRITE)

<a id="UC-TRACE"></a>

## Trace Relationships

The [ACT-USER (User)](../actors/system_users.md#ACT-USER) visualizes paths between two spec blocks.

**Steps:**

1. User identifies a starting ID and a target ID/pattern.
2. User runs the trace command ([IF-CLI-TRACE (Command: `trace`)](../specs/cli_specs.md#IF-CLI-TRACE)).
3. User analyzes the displayed paths to understand dependencies or verified coverage.

Depends on: [ACT-USER (User)](../actors/system_users.md#ACT-USER), [UC-WRITE (Write Specifications)](#UC-WRITE)

<a id="UC-DESCRIBE"></a>

## Describe Spec Block

The [ACT-USER (User)](../actors/system_users.md#ACT-USER) inspects the detailed relationships of a specific block.

**Steps:**

1. User runs the describe command ([IF-CLI-DESCRIBE (Command: `describe`)](../specs/cli_specs.md#IF-CLI-DESCRIBE)).
2. User reviews the bidirectional relationship information (outgoing references and incoming dependencies).

Depends on: [ACT-USER (User)](../actors/system_users.md#ACT-USER), [UC-WRITE (Write Specifications)](#UC-WRITE)

<a id="UC-EDITOR-LSP"></a>

## Interactive Editing via LSP

The [ACT-DEV (Developer)](../actors/system_users.md#ACT-DEV) uses a compatible editor to write and navigate specifications interactively.

**Features used:**
- **Navigation**: Jump to definition and find references.
- **Verification**: Real-time linting diagnostics.
- **Refactoring**: Workspace-wide renaming of SpecBlock IDs.

Depends on: [ACT-DEV (Developer)](../actors/system_users.md#ACT-DEV), [UC-WRITE (Write Specifications)](#UC-WRITE), [IF-LSP (Language Server)](../specs/lsp_specs.md#IF-LSP)
