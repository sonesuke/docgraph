<a id="CC_SINGLE_RESPONSIBILITY"></a>

## Single Responsibility Principle (SRP)

Each module or struct should have one, and only one, reason to change.

**Handler Modules:**

Handlers in `src/cli/handlers` and `src/lsp/handlers` are responsible ONLY for parsing arguments, calling Core, and
formatting output. They handle exactly one command or request (e.g., `check.rs`, `hover.rs`).

**Core Modules:**

Core modules in `src/core` are responsible ONLY for domain logic and validation. Each module has a single responsibility
(e.g., `parse.rs` for Markdown parsing, `lint.rs` for validation).

**Rule Modules:**

Each validation rule (`src/core/rules/dg*.rs`) implements a single check logic.

- `config.rs`: Load and manage configuration

- `dg001.rs`: Anchor must be followed by heading
- `dg002.rs`: No duplicate anchor IDs
- `dg003.rs`: No broken links
- `dg004.rs`: Link text matches heading
- `dg005.rs`: Known node type prefixes
- `dg006.rs`: Strict relation enforcement

### Decided by

- [ADR_SINGLE_RESPONSIBILITY (Single Responsibility Principle)](../../decisions/single-responsibility.md#ADR_SINGLE_RESPONSIBILITY)
  To decouple modules.
