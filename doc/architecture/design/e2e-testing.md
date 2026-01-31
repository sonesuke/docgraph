<a id="CC_E2E_TESTING"></a>

# E2E Testing Design

## Overview

This document outlines the design for End-to-End testing of the Docgraph CLI and LSP server.

## Directory Structure

Tests are organized under the `tests/` directory:

```text
tests/
  cli.rs        # CLI test entry point
  cli/          # CLI test modules
    common/     # Shared helpers
    check.rs
    graph.rs
    ...
  lsp.rs        # LSP test entry point
  lsp/          # LSP test modules
    support/    # LSP test harness (LspClient)
    diagnostics.rs
    navigation.rs
    ...
```

## CLI Testing

We use `assert_cmd` to test the CLI binary.

- Each test sets up a temporary directory (`tempfile`).
- Creates necessary configuration and markdown files.
- Runs the `docgraph` binary with arguments.
- Asserts success/failure and output content.

## LSP Testing

We test the LSP server by spawning it as a child process and communicating via stdio.

- **Harness**: `tests/lsp/support/lsp_client.rs` implements a minimal LSP client.
  - Handles message framing (Content-Length).
  - Sends Requests and Notifications.
  - Waits for expected Notifications (like `publishDiagnostics`).
- **Scenarios**: Each test file covers a specific feature set (e.g., `completion.rs`, `rename.rs`).
- **Isolation**: Every test runs in a fresh temporary directory with its own `rootUri`.
