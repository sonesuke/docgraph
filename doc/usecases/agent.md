<a id="UC_AGENT"></a>

# Agent Use Cases

<a id="UC_BUILD_KNOWLEDGE"></a>

## Build Knowledge Graph

The [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT) builds a GraphRAG knowledge base from the generated JSON graph.

**Derives:**

- [IF_CLAUDE_CODE (Interface: Claude Code Plugin)](../requirements/interfaces/claude-plugin.md#IF_CLAUDE_CODE)

Depends on: [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT), [UC_GRAPH (Generate Graph)](./analysis.md#UC_GRAPH)

<a id="UC_SUGGEST_USAGE"></a>

## Suggest Usage

The [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT) suggests how to use the system based on the constructed knowledge.

**Derives:**

- [IF_CLAUDE_CODE (Interface: Claude Code Plugin)](../requirements/interfaces/claude-plugin.md#IF_CLAUDE_CODE)

Depends on: [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT), [UC_BUILD_KNOWLEDGE (Build Knowledge Graph)](#UC_BUILD_KNOWLEDGE)

<a id="UC_FIX_DOC"></a>

## Fix Documentation

The [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT) automatically fixes documentation errors (e.g., formatting issues, simple lint violations) using the Docgraph plugin.

**Derives:**

- [IF_CLAUDE_CODE (Interface: Claude Code Plugin)](../requirements/interfaces/claude-plugin.md#IF_CLAUDE_CODE)

Depends on: [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT), [UC_LINT (Lint Documents)](./quality.md#UC_LINT)

<a id="UC_EXPLAIN_RULES"></a>

## Explain Rules

The [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT) explains validation rules and architectural constraints to the user to ensure documentation quality.

**Derives:**

- [IF_CLAUDE_CODE (Interface: Claude Code Plugin)](../requirements/interfaces/claude-plugin.md#IF_CLAUDE_CODE)

Depends on: [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT), [IF_CONFIG (docgraph.toml Configuration)](../requirements/interfaces/config-specs.md#IF_CONFIG)

<a id="UC_ASSIST_WORKFLOW"></a>

## Assist Document-Driven Workflow

The [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT) assists in the document-driven development workflow by proposing missing requirements or next architectural steps based on the current graph state.

**Derives:**

- [IF_CLAUDE_CODE (Interface: Claude Code Plugin)](../requirements/interfaces/claude-plugin.md#IF_CLAUDE_CODE)

Depends on: [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT), [UC_BUILD_KNOWLEDGE (Build Knowledge Graph)](#UC_BUILD_KNOWLEDGE)
