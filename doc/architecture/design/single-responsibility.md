# Cross-Cutting Concept: Single Responsibility Principle

<a id="CC_SINGLE_RESPONSIBILITY"></a>

## Overview

The Single Responsibility Principle (SRP) states that **each module should have one, and only one, reason to change**. In `docgraph`, this means each file/module is responsible for exactly one thing.

## Application in docgraph

### Handler Modules

Each handler handles exactly one command or request:

- `check.rs`: Only the `check` command
- `rule.rs`: Only the `rule` command
- `hover.rs`: Only hover requests
- `completion.rs`: Only completion requests

### Core Modules

Each core module has a single, well-defined responsibility:

- `parse.rs`: Extract structure from Markdown
- `collect.rs`: Gather SpecBlocks across workspace
- `lint.rs`: Orchestrate validation
- `walk.rs`: Traverse file system
- `config.rs`: Load and manage configuration

### Rule Modules

Each rule validates exactly one aspect:

- `dg001.rs`: Anchor must be followed by heading
- `dg002.rs`: No duplicate anchor IDs
- `dg003.rs`: No broken links
- `dg004.rs`: Link text matches heading
- `dg005.rs`: Known node type prefixes
- `dg006.rs`: Strict relation enforcement

## Related

- [ADR_SINGLE_RESPONSIBILITY (Single Responsibility Principle)](../../decisions/single-responsibility.md#ADR_SINGLE_RESPONSIBILITY)
