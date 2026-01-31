# Documentation Overview

<a id="IF-OVERVIEW"></a>

## Project Context

Docgraph is a specialized linter and graph generator for product documentation managed with standard Markdown. It extracts a directed graph from document blocks and ensures traceability across various design layers—from business actors and use cases down to technical specifications and architecture decisions.

### Core Concepts

- **SpecBlock**: A documented unit identified by a unique ID using the `<a id="ID_HERE"></a>` syntax on its own line.
- **Edges**: Typed relationships between SpecBlocks (e.g., `verifies`, `depends_on`, `realized_by`).
- **Graph Validation**: Automated checks to ensure all references exist and follow the relationship rules defined in `docgraph.toml`.

### Documentation Map

| Category | Description | Key Documents |
| :--- | :--- | :--- |
| **Actors** | Systems and users interacting with the system. | [ACT-USER (User)](./actors/system_users.md#ACT-USER) |
| **Use Cases** | Core workflows and user goals. | [UC-WRITE (Write Specifications)](./usecases/core_workflows.md#UC-WRITE) |
| **Requirements** | Functional and verification rules. | [FR-UNIQUE (Unique IDs)](./requirements/functional/verification.md#FR-UNIQUE) |
| **Specifications**| CLI behavior and interfaces. | [IF-CLI-LINT (Command: `lint`)](./requirements/interfaces/cli_specs.md#IF-CLI-LINT), [IF-LSP (Language Server Protocol (LSP) Support)](./requirements/interfaces/lsp_specs.md#IF-LSP), [IF-CONFIG (docgraph.toml Configuration)](./requirements/interfaces/config_specs.md#IF-CONFIG) |
| **Architecture**  | Design decisions and rationale. | [ADR-MARKDOWN-FORMAT (Choice of Plain Markdown and HTML Anchors)](./decisions/markdown_format.md#ADR-MARKDOWN-FORMAT) |
| **Quality** | Integration tests and validation scenarios. | [TEST-INT-LINT (Lint Integration Test)](./tests/integration_metrics.md#TEST-INT-LINT) |
| **Development** | Onboarding guide and architecture for developers. | [Developer Guide](./architecture/guide.md) |

### Getting Started

To verify the current documentation graph, run:

```bash
docgraph check doc
```
