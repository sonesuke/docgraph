<a id="UC_CLAUDE_INSTALL"></a>

# Install Claude Plugin

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) installs the Docgraph plugin in Claude Desktop using a two-step process: adding the marketplace and then installing the plugin. This enables document-driven development assistance and GraphRAG-based knowledge construction.

## Procedure

The Docgraph plugin is distributed via the `sonesuke/docgraph` repository.

### 1. Add Marketplace

First, add the official marketplace to your Claude instance by running:

```text
/plugin marketplace add sonesuke/docgraph
```

### 2. Install the Plugin

After adding the marketplace, install the Docgraph plugin by running:

```text
/plugin install docgraph-plugin@docgraph-claude-plugins
```

## Goals

- Enable the AI agent to understand the documentation graph.
- Provide real-time linting and fixing suggestions within the AI chat interface.
- Assist in complex traceability analysis across large documentation sets.

**Derives:**

- [FR_CLAUDE_PLUGIN (Functional Requirements: Claude Plugin)](../requirements/functional/claude-plugin.md#FR_CLAUDE_PLUGIN)
- [FR_CLAUDE_MARKETPLACE (Claude Marketplace Support)](../requirements/functional/claude-plugin.md#FR_CLAUDE_MARKETPLACE)
- [FR_CLAUDE_INSTALL (Claude Plugin Installation)](../requirements/functional/claude-plugin.md#FR_CLAUDE_INSTALL)

Depends on: [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)
