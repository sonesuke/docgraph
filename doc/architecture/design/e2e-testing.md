# Cross-Cutting Concept: E2E Testing

<a id="CC-E2E-TESTING"></a>

## Overview

E2E (End-to-End) testing in `docgraph` verifies CLI behavior by executing the actual compiled binary, complementing unit tests with integration-level validation.

## The Rule

```text
Testing Strategy:
- CLI: E2E tests (assert_cmd + predicates + tempfile)
- LSP: Unit tests with mock data
- Core: Unit tests
- Coverage: Measured with cargo llvm-cov
```

## Test Structure

### File Organization

```text
tests/
├── e2e_<command>.rs  # One file per CLI command
└── common/
    └── mod.rs        # Shared test fixtures
```

### Test Pattern

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn command_success_case() {
    let tmp = TempDir::new().unwrap();

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("command")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("expected output"));
}

#[test]
fn command_failure_case() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("command")
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}
```

## Test Categories

For each CLI command, test:

1. **Success cases**: Valid inputs, expected outputs
2. **Failure cases**: Invalid inputs, error messages
3. **File I/O**: Reading/writing files, directory handling
4. **Edge cases**: Empty inputs, missing files, etc.

## Coverage Measurement

```bash
# Run all tests with coverage
cargo llvm-cov --tests

# Generate HTML report
cargo llvm-cov --tests --html

# Generate LCOV for CI
cargo llvm-cov --tests --lcov --output-path lcov.info
```

## Relationship to Unit Tests

| Test Type | Purpose | Target |
|-----------|---------|--------|
| **Unit Tests** | Core business logic | Core layer |
| **E2E Tests (CLI)** | CLI behavior, I/O | CLI handlers |
| **Unit Tests (LSP)** | LSP handler logic | LSP handlers |

E2E tests complement, not replace, unit tests.

### LSP Testing

LSP handlers use **unit tests**, not E2E tests:

**Rationale:**

- LSP E2E testing is complex (requires client simulation, JSON-RPC)
- LSP handlers are thin wrappers (similar to CLI handlers)
- Core logic is already unit tested
- Unit tests with mock data are sufficient

**Coverage Strategy:**

- Delegation-only files (backend.rs, call_hierarchy.rs): No tests needed
- Handler files: Unit tests with mock SpecBlock/RefUse data
- Focus on edge cases and error handling

## Guidelines

### DO

- ✅ Test actual binary execution
- ✅ Verify error messages shown to users
- ✅ Test file I/O and directory handling
- ✅ Use `tempfile` for isolated test environments
- ✅ Assert on stdout, stderr, and exit codes

### DON'T

- ❌ Duplicate core logic tests in E2E tests
- ❌ Test implementation details
- ❌ Share state between E2E tests
- ❌ Hardcode file paths (use `tempfile`)

## Example: Complete E2E Test

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn check_reports_missing_heading() {
    // Arrange: Create test document
    let tmp = TempDir::new().unwrap();
    let doc = tmp.path().join("test.md");
    fs::write(&doc, "<a id=\"TEST-01\"></a>\n").unwrap();

    // Act: Run docgraph check
    let mut cmd = Command::cargo_bin("docgraph").unwrap();
    cmd.arg("check").arg(tmp.path());

    // Assert: Verify error output
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("DG001"))
        .stderr(predicate::str::contains("is not followed by a heading"));
}
```

## Related

- [ADR-E2E-TESTING (E2E Testing Strategy: assert_cmd with cargo llvm-cov)](../../decisions/e2e-testing.md#ADR-E2E-TESTING)
