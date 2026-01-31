# Cross-Cutting Concept: Error Handling Boundaries

<a id="CC-ERROR-HANDLING"></a>

## Overview

Error handling in `docgraph` follows a **boundary-based strategy**: typed errors in the core library, contextual errors in binaries.

## The Rule

```text
Core (lib)  : thiserror → typed errors
CLI (bin)   : anyhow    → contextual reporting
LSP (server): anyhow    → convert to JSON-RPC at boundary
```

## Core Library: thiserror

Use `thiserror` to define typed error enums:

```rust
#[derive(Debug, Error)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),

    #[error("parse error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

## CLI Binary: anyhow

Use `anyhow` with `.context()` for rich error messages:

```rust
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let config = load_config()
        .context("failed to load config")?;
    Ok(())
}
```

## LSP Server: anyhow + Conversion

Use `anyhow` internally, convert to JSON-RPC errors at the handler boundary:

```rust
async fn handle_request(...) -> Result<Response, jsonrpc::Error> {
    let result = core_function()
        .context("operation failed")
        .map_err(to_jsonrpc_error)?;
    Ok(result)
}
```

## Boundary Conversion

Errors automatically convert across boundaries:

- `core::Error` → `anyhow::Error` (automatic via `?`)
- `anyhow::Error` → `jsonrpc::Error` (explicit conversion)

## Related

- [ADR-ERROR-HANDLING (Error Handling Strategy: thiserror for Core, anyhow for Binaries)](../../decisions/error-handling.md#ADR-ERROR-HANDLING)
