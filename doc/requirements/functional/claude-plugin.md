<a id="FR_CLAUDE_PLUGIN"></a>

# Functional Requirements: Claude Plugin

The Docgraph Claude plugin enables seamless integration with Claude Desktop, allowing the AI agent to interact with the documentation graph.

**Derives:**

- [IF_CLAUDE_CODE (Interface: Claude Code Plugin)](../interfaces/claude-plugin.md#IF_CLAUDE_CODE)

**Realized by:**

- [MOD_CORE (Core Modules)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_CLAUDE_MARKETPLACE"></a>

## Claude Marketplace Support

The plugin SHALL support distribution via the Claude marketplace mechanism. This includes providing a valid `.claude-plugin/marketplace.json` file in the repository root that defines the plugin and its source.

**Realized by:**

- [marketplace.json (Marketplace Configuration)](../../../.claude-plugin/marketplace.json)
- [MOD_CORE (Core Modules)](../../architecture/view/module.md#MOD_CORE)

<a id="FR_CLAUDE_INSTALL"></a>

## Claude Plugin Installation

The plugin SHALL be installable via the Claude Desktop `/plugin` commands. Specifically, it SHALL support being added as a marketplace and then installed as a local plugin.

**Realized by:**

- [plugin.json (Plugin Definition)](../../../docgraph-plugin/.claude-plugin/plugin.json)
- [MOD_CORE (Core Modules)](../../architecture/view/module.md#MOD_CORE)

**Derives:**

- [IF_LSP (Language Server Protocol (LSP) Support)](../interfaces/lsp-specs.md#IF_LSP)
