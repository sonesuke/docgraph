# Documentation Overview

## Project Context

Docgraph is a specialized linter and graph generator for product documentation managed with standard Markdown. It extracts a directed graph from document blocks and ensures traceability across various design layers—from business actors and use cases down to technical specifications and architecture decisions.

### Core Concepts

- **SpecBlock**: A documented unit identified by a unique ID using the `<a id="ID_HERE"></a>` syntax on its own line.
- **Edges**: Typed relationships between SpecBlocks (e.g., `verifies`, `depends_on`, `realized_by`).
- **Graph Validation**: Automated checks to ensure all references exist and follow the relationship rules defined in `docgraph.toml`.

### Documentation Map

| Category | Description | Key Documents |
| :--- | :--- | :--- |
| **Actors** | Systems and users interacting with the system. | [ACT_USER (User)](./actors/users.md#ACT_USER), [ACT_AGENT (AI Agent)](./actors/systems.md#ACT_AGENT) |
| **Use Cases** | Core workflows and user goals. | [UC_WRITE (Write Specifications)](./usecases/authoring.md#UC_WRITE), [UC_BUILD_KNOWLEDGE (Build Knowledge Graph)](./usecases/agent.md#UC_BUILD_KNOWLEDGE), [UC_FIX_DOC (Fix Documentation)](./usecases/agent.md#UC_FIX_DOC), [UC_EXPLAIN_RULES (Explain Rules)](./usecases/agent.md#UC_EXPLAIN_RULES), [UC_CLAUDE_INSTALL (Install Claude Plugin)](./usecases/claude-plugin.md#UC_CLAUDE_INSTALL) |
| **Requirements** | Functional and verification rules. | [FR_UNIQUE (Unique IDs)](./requirements/functional/verification.md#FR_UNIQUE), [FR_CLAUDE_MARKETPLACE (Claude Marketplace Support)](./requirements/functional/claude-plugin.md#FR_CLAUDE_MARKETPLACE), [NFR_PERF (High Performance)](./requirements/non-functional/performance.md#NFR_PERF), [NFR_EXT (Modular Design)](./requirements/non-functional/extensibility.md#NFR_EXT) |
| **Constraints** | System-wide limitations and technology choices. | [CON_PERF (High Performance)](./constraints/development.md#CON_PERF), [CON_SOLO (Solo Development)](./constraints/development.md#CON_SOLO), [CON_EXT (Extensibility)](./constraints/development.md#CON_EXT) |
| **Specifications**| CLI behavior and interfaces. | [IF_CLI_LINT (Command: `lint`)](./requirements/interfaces/cli-specs.md#IF_CLI_LINT), [IF_LSP (Language Server Protocol (LSP) Support)](./requirements/interfaces/lsp-specs.md#IF_LSP), [IF_CONFIG (docgraph.toml Configuration)](./requirements/interfaces/config-specs.md#IF_CONFIG), [IF_CLAUDE_CODE (Interface: Claude Code Plugin)](./requirements/interfaces/claude-plugin.md#IF_CLAUDE_CODE) |
| **Architecture**  | Design decisions and rationale. | [ADR_MARKDOWN_FORMAT (Choice of Plain Markdown and HTML Anchors)](./decisions/markdown-format.md#ADR_MARKDOWN_FORMAT), [ADR_LAYERED_ARCH (Layered Architecture: Core, CLI Handlers, LSP Handlers)](./decisions/layered-architecture.md#ADR_LAYERED_ARCH) |
| **Development** | Onboarding guide and architecture for developers. | [Developer Guide](./architecture/view/guide.md) |

### Getting Started

To verify the current documentation graph, run:

```bash
docgraph check doc
```
