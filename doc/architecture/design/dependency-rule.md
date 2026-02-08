<a id="CC_DEPENDENCY_RULE"></a>

## The Dependency Rule

Dependencies must point inwards, towards high-level policies.

**Direction:**

- **Outer Layers** (Infrastructure, adapters) depend on **Inner Layers** (Use Cases, Entities).
- **Inner Layers** MUST NOT depend on **Outer Layers**.

**Core Layer Independence:**

The `src/core/` module contains only domain types and business logic. It has NO references to CLI or LSP types.

**Handler Layer Dependency:**

CLI and LSP handlers import from Core, but Core never imports from handlers.

### Decided by

- [ADR_LAYERED_ARCH (Layered Architecture)](../../decisions/layered-architecture.md#ADR_LAYERED_ARCH) To enforce
  directional dependency.
