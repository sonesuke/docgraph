# Architecture Decision Records

<a id="ADR-THIN-HANDLERS"></a>

## Thin Handlers

### Status

Accepted

### Context

In a layered architecture with Core and Handler layers, we need to decide how much logic should reside in handlers versus the Core. Putting too much logic in handlers leads to duplication across interfaces (CLI, LSP) and makes testing harder.

### Decision

We adopt the **Thin Handlers** principle: handlers should contain no business logic. They serve only as adapters between the user interface and the Core layer.

Handlers perform exactly three tasks:

1. **Input transformation**: Convert UI-specific types to Core types
2. **Core invocation**: Call Core logic functions
3. **Output transformation**: Convert Core types back to UI-specific types

### Rationale

#### Easy Testing

Handlers are thin enough that integration tests are sufficient. Complex logic is tested in Core unit tests, which are faster and more reliable.

#### Centralized Logic

All business logic lives in Core, making it easy to find and modify. Developers don't need to search through multiple handler files to understand validation rules or parsing logic.

#### Interface Independence

The same Core logic can be called from CLI, LSP, or any future interface without duplication. Adding a new interface requires only writing a thin adapter layer.

### Consequences

#### Positive

- Business logic is centralized and easy to test
- No code duplication across interfaces
- New interfaces are easy to add
- Clear separation between UI concerns and business logic

#### Negative

- Requires discipline to avoid adding logic to handlers
- May feel verbose for simple operations

### Examples

#### CLI Handler

```rust
// src/cli/handlers/check.rs
pub fn handle_check(path: PathBuf, json: bool, fix: bool, rule: Option<Vec<String>>) -> ExitCode {
    // 1. Input transformation
    let config = config::Config::load(&path).unwrap_or_default();

    // 2. Core invocation
    let diagnostics = lint::check_workspace(&path, fix, rule, true, &config);

    // 3. Output transformation
    if json {
        println!("{}", serde_json::to_string_pretty(&diagnostics).unwrap());
    } else {
        print_diagnostics(&diagnostics);
    }

    // Return exit code
    if diagnostics.iter().any(|d| matches!(d.severity, Severity::Error)) {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
```

#### LSP Handler

```rust
// src/lsp/handlers/hover.rs
pub async fn handle_hover(
    backend: &Backend,
    params: HoverParams,
) -> Result<Option<Hover>> {
    // 1. Input transformation
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;
    let path = uri.to_file_path().ok()?;

    // 2. Core invocation
    let content = backend.get_document_content(&uri)?;
    let blocks = parse::extract_spec_blocks(&content);
    let block = find_block_at_position(&blocks, position)?;

    // 3. Output transformation
    Ok(Some(Hover {
        contents: HoverContents::Scalar(MarkedString::String(
            format!("**{}**\n\n{}", block.id, block.heading)
        )),
        range: Some(block.range),
    }))
}
```

### Anti-Patterns

#### Business Logic in Handler

**Bad**:

```rust
// DON'T put validation logic in handlers
pub fn handle_check(...) {
    for file in files {
        if !file.has_anchor() {
            // Validation logic here - WRONG!
        }
    }
}
```

**Good**:

```rust
// Put validation logic in Core
pub fn handle_check(...) {
    let diagnostics = lint::check_workspace(...); // Core does validation
    print_diagnostics(&diagnostics);
}
```

#### Data Processing in Handler

**Bad**:

```rust
// DON'T parse or transform data in handlers
pub fn handle_check(...) {
    let content = fs::read_to_string(path)?;
    let blocks = extract_blocks(&content); // Parsing logic - WRONG!
}
```

**Good**:

```rust
// Let Core handle data processing
pub fn handle_check(...) {
    let blocks = parse::extract_spec_blocks(&content); // Core does parsing
}
```

### Related

- [ADR-LAYERED-ARCH (Layered Architecture: Core, CLI Handlers, LSP Handlers)](./layered-architecture.md#ADR-LAYERED-ARCH)
