# Environment Setup Use Cases

<a id="UC_INSTALL_MANUAL"></a>

## Install Manual Setup

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) sets up the development environment manually on their local machine without using Docker.

**Derives:**

- [IF_CLAUDE_MARKETPLACE (Claude Marketplace)](../requirements/interfaces/interfaces.md#IF_CLAUDE_MARKETPLACE)
- [FR_INSTALL_MANUAL (Manual Installation)](../requirements/functional/installation.md#FR_INSTALL_MANUAL)

## Description

The developer ensures their local environment meets the specifications defined in [FR_INSTALL_MANUAL](../requirements/functional/installation.md#FR_INSTALL_MANUAL).

---

<a id="UC_CLAUDE_INSTALL"></a>

## Install Claude Plugin

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) installs the Docgraph plugin in Claude Desktop using a two-step process: adding the marketplace and then installing the plugin. This enables document-driven development assistance and GraphRAG-based knowledge construction.

### Procedure

The Docgraph plugin is distributed via the `sonesuke/docgraph` repository.

#### 1. Add Marketplace

First, add the official marketplace to your Claude instance by running:

```text
/plugin marketplace add sonesuke/docgraph
```

#### 2. Install the Plugin

After adding the marketplace, install the Docgraph plugin by running:

```text
/plugin install docgraph-plugin@docgraph-claude-plugins
```

### Goals

- Enable the AI agent to understand the documentation graph.
- Provide real-time linting and fixing suggestions within the AI chat interface.
- Assist in complex traceability analysis across large documentation sets.

**Derives:**

- [IF_CLAUDE_MARKETPLACE (Claude Marketplace)](../requirements/interfaces/interfaces.md#IF_CLAUDE_MARKETPLACE)
- [FR_CLAUDE_MARKETPLACE (Claude Marketplace Support)](../requirements/functional/claude.md#FR_CLAUDE_MARKETPLACE)
- [FR_CLAUDE_INSTALL (Claude Plugin Installation)](../requirements/functional/claude.md#FR_CLAUDE_INSTALL)

---

<a id="UC_VSCODE_INSTALL"></a>

## Install VS Code Extension

The [ACT_DEV (Developer)](../actors/users.md#ACT_DEV) installs the `docgraph` VS Code extension to enable rich editing features for Markdown-based specifications.

**Derives:**

- [IF_VSCODE_MARKETPLACE (VS Code Marketplace)](../requirements/interfaces/interfaces.md#IF_VSCODE_MARKETPLACE)
- [FR_VSC_BINARY_PATH (Binary Path Configuration)](../requirements/functional/vscode.md#FR_VSC_BINARY_PATH)
- [FR_VSC_MARKDOWN_ACTIVATION (Markdown Activation)](../requirements/functional/vscode.md#FR_VSC_MARKDOWN_ACTIVATION)
- [FR_VSC_SERVER_LIFECYCLE (Server Lifecycle Commands)](../requirements/functional/vscode.md#FR_VSC_SERVER_LIFECYCLE)
- [NFR_VSCODE_PORTABILITY (Cross-platform Portability)](../requirements/non-functional/vscode.md#NFR_VSCODE_PORTABILITY)
- [NFR_VSCODE_PACKAGING (Lightweight Packaging)](../requirements/non-functional/vscode.md#NFR_VSCODE_PACKAGING)
