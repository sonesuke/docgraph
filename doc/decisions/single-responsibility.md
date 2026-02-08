<a id="ADR_SINGLE_RESPONSIBILITY"></a>

# Single Responsibility Principle

Applies the Single Responsibility Principle to Docgraph modules, ensuring each module has one reason to change.

## Decision

We adopt the **Single Responsibility Principle (SRP)**: each module should have one, and only one, reason to change. In
practice, this means each file/module is responsible for exactly one thing.

## Rationale

- **Easier to Understand**: When reading `check.rs`, developers only need to understand the `check` command logic. No
  other command logic is mixed in, reducing cognitive load.
- **Limited Change Impact**: When modifying the `check` command, developers only need to edit `check.rs`. Other commands
  are unaffected, minimizing the risk of unintended side effects.
- **Clear Ownership**: Each file has a clear purpose, making it obvious where to add new functionality or fix bugs. This
  improves developer productivity and code navigation.
- **Better Testing**: Each module can be tested independently with focused test cases, improving test clarity and
  reducing test complexity.

### Trade-offs

- More files to manage (one per responsibility)
- May require more navigation between files when working on related features

## Context

As the `docgraph` codebase grows, we need a clear principle for organizing code into modules and files. Without a
guiding principle, modules can become bloated with multiple responsibilities, making the code harder to understand,
test, and maintain.

### Anti-Pattern Example

**Bad** - Multiple responsibilities in one file:

```rust
// src/cli/commands.rs (DON'T DO THIS)
pub fn handle_check(...) { /* ... */ }
pub fn handle_rule(...) { /* ... */ }
pub fn handle_graph(...) { /* ... */ }
pub fn handle_list(...) { /* ... */ }
```

This violates SRP because the file has multiple reasons to change (any change to any command requires editing this
file).

**Good** - Single responsibility per file:

```rust
// src/cli/handlers/check.rs
pub fn handle_check(...) { /* ... */ }

// src/cli/handlers/rule.rs
pub fn handle_rule(...) { /* ... */ }
```

Each file has exactly one reason to change (changes to that specific command).

## Related

- [ADR_LAYERED_ARCH (Layered Architecture)](./layered-architecture.md#ADR_LAYERED_ARCH)
