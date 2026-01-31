<a id="ADR_E2E_TESTING"></a>

# Testing Strategy for CLI and LSP

## Status

Accepted

## Context

Docgraph has two main interfaces: the CLI and the LSP server. Both interact significantly with the file system and external inputs (args, stdio).
Previously, we attempted to test internal handlers using unit tests, but this required mocking complex contexts and often duplicated the logic found in integration tests.

## Decision

We will adopt an End-to-End (E2E) testing strategy for both CLI and LSP interfaces.

- **CLI Tests**: Located in `tests/cli/`. We use `assert_cmd` to invoke the binary and verify exit codes, stdout, and stderr.
- **LSP Tests**: Located in `tests/lsp/`. We use a custom `LspClient` harness (over stdio) to spawn the server binary and communicate via JSON-RPC.
- **Core Unit Tests**: Core logic (parsing, graph construction, rules) will still be tested via unit tests in `src/core/`.

## Consequences

- **Pros**:
  - Tests reflect real-world usage.
  - No need to mock internal server state or stdio.
  - Refactoring implementation details doesn't break tests as long as the external behavior remains the same.
- **Cons**:
  - Tests are slightly slower than unit tests (process spawning).
  - Debugging failures requires inspecting process output or logs.
