<a id="CC_LAYERED_ARCH"></a>

## Layered Architecture

The system follows a strict layered architecture to separate concerns and ensure testability.

**Layers:**

1.  **Core (src/core/)**:
    - Domain logic, graph representation, validation rules.
    - Dependencies: None (Pure Rust).
2.  **Handlers (src/cli/, src/lsp/)**:
    - Translating external requests (CLI args, JSON-RPC) into Core calls.
    - Dependencies: Core.
3.  **Infrastructure (src/main.rs, vsix/)**:
    - Bootstrapping, I/O, VS Code integration.
    - Dependencies: Handlers, Core.

### Decided by

- [ADR_LAYERED_ARCH (Layered Architecture)](../../decisions/layered-architecture.md#ADR_LAYERED_ARCH) To organize code
  structure.
