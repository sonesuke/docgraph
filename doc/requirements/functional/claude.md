# Functional Requirements: Claude Plugin

<a id="FR_CLAUDE_RAG"></a>

## Retrieval-Augmented Generation

The agent SHALL use the document graph to perform Retrieval-Augmented Generation, fetching relevant context for user queries.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_CLAUDE_SUGGEST"></a>

## Usage Suggestions

The agent SHALL suggest appropriate commands and workflows based on the current document context.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_CLAUDE_FIX"></a>

## Automated Fixes

The agent SHALL be able to automatically apply fixes to the documentation, such as formatting and structural corrections.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_CLAUDE_EXPLAIN"></a>

## Rule Explanation

The agent SHALL be able to explain validation rules and errors to the user in natural language.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_CLAUDE_WORKFLOW"></a>

## Workflow Assistance

The agent SHALL guide the user through the document-driven development workflow, prompting for next steps.

### Realized by

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_CLAUDE_MARKETPLACE"></a>

## Claude Marketplace Support

The plugin SHALL support distribution via the Claude marketplace mechanism. This includes providing a valid `.claude-plugin/marketplace.json` file in the repository root that defines the plugin and its source.

### Realized by

- [MOD_PLUGIN (Claude Code Plugin)](../../architecture/view/module.md#MOD_PLUGIN)
- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_CLAUDE_INSTALL"></a>

## Claude Plugin Installation

The plugin SHALL be installable via the Claude Desktop `/plugin` commands. Specifically, it SHALL support being added as a marketplace and then installed as a local plugin.

### Realized by

- [MOD_PLUGIN (Claude Code Plugin)](../../architecture/view/module.md#MOD_PLUGIN)
- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)
