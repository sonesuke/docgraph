<a id="ADR_E2E_TESTING"></a>

# Testing Strategy for CLI and LSP

Validates the system behavior from an end-user perspective using CLI and LSP end-to-end tests.

## Decision

We will adopt an End-to-End (E2E) testing strategy for both CLI and LSP interfaces.

- **CLI Tests**: Located in `tests/cli/`. We use `assert_cmd` to invoke the binary and verify exit codes, stdout, and
  stderr.
- **LSP Tests**: Located in `tests/lsp/`. We use a custom `LspClient` harness (over stdio) to spawn the server binary
  and communicate via JSON-RPC.
- **Core Unit Tests**: Core logic (parsing, graph construction, rules) will still be tested via unit tests in
  `src/core/`.

## Rationale

- **Tests reflect real-world usage**: We verify the system as a user sees it.
- **No mocking**: We don't need to mock internal server state or stdio.
- **Refactoring safe**: Implementation details can change without breaking tests as long as external behavior is
  preserved.

### Trade-offs

- **Speed**: Tests are slightly slower than unit tests due to process spawning.
- **Debugging**: Failures require inspecting process output or logs rather than just stack traces.

## Context

Docgraph has two main interfaces: the CLI and the LSP server. Both interact significantly with the file system and
external inputs (args, stdio). Previously, we attempted to test internal handlers using unit tests, but this required
mocking complex contexts and often duplicated the logic found in integration tests.
