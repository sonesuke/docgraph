# Cross-Cutting Concept: Dependency Rule

<a id="CC-DEPENDENCY-RULE"></a>

## Overview

The Dependency Rule is a fundamental architectural principle in `docgraph`: **dependencies always point inward toward the Core layer**.

## The Rule

```text
CLI Handlers → Core ← LSP Handlers
```

- **Core** has no knowledge of outer layers (CLI, LSP)
- **Handlers** depend on Core, but Core does not depend on Handlers
- This allows Core to be reused across multiple interfaces

## Implementation

### Core Layer Independence

The `src/core/` module contains only:

- Domain types (`SpecBlock`, `Diagnostic`, etc.)
- Business logic (parsing, validation, graph building)
- No references to CLI or LSP types

### Handler Layer Dependency

CLI and LSP handlers import from Core:

```rust
// src/cli/handlers/check.rs
use crate::core::{config, lint};

// src/lsp/handlers/hover.rs
use crate::core::{parse, types};
```

But Core never imports from handlers.

## Related

- [ADR-LAYERED-ARCH (Layered Architecture: Core, CLI Handlers, LSP Handlers)](../../decisions/layered-architecture.md#ADR-LAYERED-ARCH)
