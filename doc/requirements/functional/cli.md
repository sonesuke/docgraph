# Functional Requirements: CLI

<a id="FR_CLI_GRAPH"></a>

## Graph Generation

The system SHALL generate a JSON representation of the document graph, including all nodes and edges.

**Realized by:**

- [MOD_CLI (CLI Modules)](../../architecture/view/module.md#MOD_CLI)
- [MOD_CORE (Core Modules)](../../architecture/view/module.md#MOD_CORE)

**Derives:**

- [IF_CLI_GRAPH (Command: `graph`)](../interfaces/cli-specs.md#IF_CLI_GRAPH)

<a id="FR_CLI_LSP"></a>

## LSP Server

The system SHALL provide a Language Server Protocol server to support interactive editing.

**Realized by:**

- [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

**Derives:**

- [IF_CLI_LSP (Command: `lsp`)](../interfaces/cli-specs.md#IF_CLI_LSP)

<a id="FR_CLI_TYPE"></a>

## Type Information

The system SHALL provide a command to list and describe available node types and their validation rules.

**Realized by:**

- [MOD_CLI (CLI Modules)](../../architecture/view/module.md#MOD_CLI)

**Derives:**

- [IF_CLI_TYPE (Command: `type`)](../interfaces/cli-specs.md#IF_CLI_TYPE)

<a id="FR_CLI_LIST"></a>

## List Specifications

The system SHALL provide a command to list spec blocks matching a query pattern.

**Realized by:**

- [MOD_CLI (CLI Modules)](../../architecture/view/module.md#MOD_CLI)

**Derives:**

- [IF_CLI_LIST (Command: `list`)](../interfaces/cli-specs.md#IF_CLI_LIST)

<a id="FR_CLI_TRACE"></a>

## Trace Relationships

The system SHALL provide a command to trace and display paths between document nodes.

**Realized by:**

- [MOD_CLI (CLI Modules)](../../architecture/view/module.md#MOD_CLI)

**Derives:**

- [IF_CLI_TRACE (Command: `trace`)](../interfaces/cli-specs.md#IF_CLI_TRACE)

<a id="FR_CLI_DESCRIBE"></a>

## Describe Node

The system SHALL provide a command to show detailed information and relationships of a specific node.

**Realized by:**

- [MOD_CLI (CLI Modules)](../../architecture/view/module.md#MOD_CLI)

**Derives:**

- [IF_CLI_DESCRIBE (Command: `describe`)](../interfaces/cli-specs.md#IF_CLI_DESCRIBE)
