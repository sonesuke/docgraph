# Module View

The `docgraph` project is organized into several distinct modules, separating the core domain logic, the command-line interface, the language server, and the editor integration.

## Module Structure Overview

```mermaid
graph TD
    subgraph "Editor Layer"
        VSEX["vsix (VS Code Extension)"]
    end

    subgraph "Application Layer"
        CLI["cli (Command Line Interface)"]
        LSP["lsp (Language Server)"]
    end

    subgraph "Domain Layer (core)"
        CORE["core (Business Logic)"]
        PARSE["parse (Markdown Parsing)"]
        LINT["lint (Validation Rules)"]
        COLL["collect (Block Collection)"]
    end

    VSEX -- LSP --> LSP
    CLI --> CORE
    LSP --> CORE
    CORE --> COLL
    COLL --> PARSE
    COLL --> LINT
```

<a id="MOD_CORE"></a>

## Core Modules

The `core` module contains the fundamental logic for analyzing documentation graphs.

- **`types`**: Defines the data structures for `Node`, `RefUse`, and `Graph`.
- **`parse`**: Responsible for extracting `{document}` blocks and HTML anchors from Markdown files.
- **`collect`**: Orchestrates the scanning of directories and aggregation of nodes.
- **`lint`**: Implements the validation engine and individual rules (DG001-DG006).

<a id="MOD_CLI"></a>

## CLI Modules

The `cli` module handles user interaction via the terminal.

- **`handlers`**: Contains the logic for each CLI command (`check`, `list`, `describe`, `graph`, `trace`, `rule`).
- **`output`**: Manages the formatting of results (Text, JSON).

<a id="MOD_LSP"></a>

## LSP Modules

The `lsp` module provides the Language Server Protocol implementation.

- **`handlers`**: Implements LSP capabilities such as `textDocument/definition`, `textDocument/references`, and `textDocument/hover`.

<a id="MOD_VSEX"></a>

## VS Code Extension

The `vsix` module is a TypeScript-based project that acts as the LSP client.

- **`src/extension.ts`**: Manages the lifecycle of the `docgraph` language server and registers editor-specific commands.

<a id="MOD_EXT_ZED"></a>

## Zed Editor Extension

The `zed-extension` is a WASM-based extension for the Zed editor.

- **`zed-extension/src/lib.rs`**: Implements the `zed::Extension` trait and the `language_server_command` to bridge Zed and the `docgraph` binary.

<a id="MOD_CICD"></a>

## CI/CD Modules

The `.github` directory contains the CI/CD pipeline configuration.

- **`.github/workflows/ci.yml`**: Automated validation and testing on pull requests.
- **`.github/workflows/codeql.yml`**: Static application security testing.
- **`.github/dependabot.yml`**: Automated dependency updates.

<a id="MOD_PLUGIN"></a>

## Claude Code Plugin

The `docgraph-plugin` provides a Model Context Protocol (MCP) server for Claude Code.

- **`skills/docgraph`**: Defines the tools and resources available to the AI agent.
- **`.claude-plugin`**: Contains the plugin manifest and capability definitions.

<a id="MOD_DEV_CONTAINER"></a>

## Dev Container Modules

The `.devcontainer` directory contains the development environment configuration.

- **`devcontainer.json`**: Defines the container image, features, and tool installations.
- **`postCreateCommand`**: Installs additional tools (docgraph, Claude Code).
- **Extensions**: Pre-configured VS Code extensions for Rust, TOML, debugging, and AI assistance.
