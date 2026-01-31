<a id="IF_CLAUDE_CODE"></a>

# Interface: Claude Code Plugin

The Docgraph Claude plugin provides a specialized interface for Claude Code, enabling the AI agent to perform complex documentation analysis and manipulation.

## Overview

The interface acts as a high-level wrapper around the Docgraph core functions, tailored for the agentic capabilities of Claude. It leverages the Model Context Protocol (MCP) to expose tools and resources.

## Exposed Capabilities

The interface MUST expose the following capabilities to the AI agent:

1. **Knowledge Extraction**: Tools to retrieve the structured graph from Markdown files.
2. **Traceability Analysis**: Tools to trace dependencies and impacts across the graph.
3. **Linting & Fixing**: Tools to run validation rules and apply automated fixes.
4. **Rule Explanation**: Tools to retrieve human-readable descriptions of architectural rules.

## Implementation Details

The interface is realized through the MCP server defined in the `docgraph-plugin` directory.

**Realized by:**

- [plugin.json (Plugin Definition)](../../../docgraph-plugin/.claude-plugin/plugin.json)
- [SKILL.md (Plugin Skill Definition)](../../../docgraph-plugin/skills/docgraph/SKILL.md)

**Depends on:**

- [IF_LSP (Language Server Protocol (LSP) Support)](./lsp-specs.md#IF_LSP)
- [IF_CLI_LINT (Command: `lint`)](./cli-specs.md#IF_CLI_LINT)
