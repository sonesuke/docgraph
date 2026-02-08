<a id="ADR_ERROR_HANDLING"></a>

# Error Handling Strategy: thiserror for Core, anyhow for Binaries

Defines the error handling strategy across the Docgraph architecture, using `thiserror` for the core library and
`anyhow` for binary crates (CLI, LSP).

## Decision

We adopt a **boundary-based error handling strategy**:

1. **Core library**: Use `thiserror` for typed errors
2. **CLI binary**: Use `anyhow` for contextual error reporting
3. **LSP server**: Use `anyhow` internally, convert to JSON-RPC errors at the boundary

### Implementation Example

#### Core Module

```rust
// src/core/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),

    #[error("parse error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

```rust
// src/core/parse.rs
use crate::core::error::{Error, Result};

pub fn parse_file(path: &Path) -> Result<Vec<Block>> {
    let content = std::fs::read_to_string(path)?; // io::Error auto-converts
    parse_content(&content)
        .map_err(|e| Error::Parse(e.to_string()))
}
```

#### CLI Handler

```rust
// src/cli/handlers/check.rs
use anyhow::Context;

pub fn handle_check(...) -> anyhow::Result<()> {
    let config = config::Config::load(&path)
        .context("failed to load config")?; // core::Error → anyhow::Error

    let diagnostics = lint::check_workspace(...)
        .context("workspace check failed")?;

    Ok(())
}
```

#### LSP Handler

```rust
// src/lsp/handlers/hover.rs
use anyhow::Context;
use tower_lsp::jsonrpc;

pub async fn handle_hover(...) -> Result<Option<Hover>, jsonrpc::Error> {
    let blocks = parse::extract_spec_blocks(&content)
        .context("failed to parse blocks")
        .map_err(to_jsonrpc_error)?; // Convert at boundary

    Ok(Some(hover))
}

fn to_jsonrpc_error(err: anyhow::Error) -> jsonrpc::Error {
    jsonrpc::Error {
        code: jsonrpc::ErrorCode::InternalError,
        message: err.to_string(),
        data: None,
    }
}
```

## Rationale

We selected this hybrid approach to balance type safety in the core library with development velocity and error context
in the application layers.

### 1. Core Library: thiserror (Typed Errors)

**Why**: The core library is called by other crates (CLI, LSP), so errors should be **typed** for programmatic handling.

**Benefits**:

- Callers can match on specific error variants
- Error types are self-documenting
- Automatic conversion with `#[from]` attribute
- No runtime overhead

**Example**:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("io error")]
    Io(#[from] std::io::Error),

    #[error("parse error at {path}:{line}: {message}")]
    Parse {
        path: String,
        line: usize,
        message: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
```

#### 2. CLI Binary: anyhow (Contextual Reporting)

**Why**: CLI is the final consumer - errors are displayed to users and then the program exits. We need **rich context**
more than type safety.

**Benefits**:

- `.context()` adds contextual information at each layer
- Automatic conversion from any error type
- Backtrace support for debugging
- Simpler error propagation with `?`

**Example**:

```rust
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let config = config::Config::load(&path)
        .context("failed to load docgraph.toml")?;

    let diagnostics = lint::check_workspace(&path, fix, rule, true, &config)
        .context("failed to check workspace")?;

    Ok(())
}
```

Core's `Error` automatically converts to `anyhow::Error` via `?` operator.

#### 3. LSP Server: anyhow + JSON-RPC Conversion

**Why**: LSP handles requests asynchronously and needs to convert errors to JSON-RPC error responses.

**Strategy**:

- Use `anyhow` internally for flexibility
- Convert to `tower_lsp::jsonrpc::Error` at the handler boundary
- Map error types to appropriate JSON-RPC error codes

**Example**:

```rust
use anyhow::Context;
use tower_lsp::jsonrpc;

async fn handle_hover(...) -> Result<Option<Hover>, jsonrpc::Error> {
    let content = backend.get_document_content(&uri)
        .context("failed to read document")
        .map_err(|e| jsonrpc::Error {
            code: jsonrpc::ErrorCode::InternalError,
            message: e.to_string(),
            data: None,
        })?;

    // ...
}
```

### Trade-offs

- **Two error crates**: Need to maintain both `thiserror` and `anyhow`
- **Conversion overhead**: LSP needs explicit conversion to JSON-RPC errors
- **Learning curve**: Developers need to understand when to use which crate

## Context

The `docgraph` project has a layered architecture with:

- **Core library** (`src/core/`, `src/lib.rs`) - Reusable logic exposed to other crates
- **CLI binary** (`src/main.rs`, `src/cli/`) - Command-line interface
- **LSP server** (`src/lsp/`) - Language Server Protocol implementation

We need a consistent error handling strategy that:

1. Provides typed errors for library APIs
2. Offers rich context for end-user error messages
3. Converts errors appropriately at layer boundaries

## Related

- [ADR_LAYERED_ARCH (Layered Architecture)](./layered-architecture.md#ADR_LAYERED_ARCH)
