# Architecture Decision Records

<a id="ADR_E2E_TESTING"></a>

## E2E Testing Strategy: assert_cmd with cargo llvm-cov

### Status

Accepted

### Context

The `docgraph` CLI currently has:

- **High unit test coverage** for core logic (86-100%)
- **Zero coverage** for CLI handlers (0%)
- **No E2E tests** that verify actual binary execution

This creates gaps in testing:

1. CLI argument parsing and routing are untested
2. Error messages shown to users are not validated
3. File I/O behavior is not verified end-to-end
4. Exit codes and output formats are not checked

We need an E2E testing strategy that:

- Tests the actual compiled binary (not just library code)
- Measures code coverage including E2E test execution
- Complements existing unit tests without duplication
- Integrates with CI/CD pipeline

### Decision

We adopt an **E2E testing strategy** using:

1. **assert_cmd** - Execute CLI binary in tests
2. **predicates** - Assert on stdout/stderr/exit codes
3. **tempfile** - Create isolated test environments
4. **cargo llvm-cov** - Measure coverage including E2E tests

### Rationale

#### 1. Why assert_cmd?

**Benefits**:

- Executes the actual compiled binary (not library code)
- Provides fluent API for assertions
- Handles process spawning and cleanup
- Works seamlessly with `cargo test`

**Example**:

```rust
use assert_cmd::Command;

#[test]
fn check_help_works() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));
}
```

#### 2. Why predicates?

**Benefits**:

- Composable assertions for strings, JSON, etc.
- Clear error messages on failure
- Type-safe matching

**Example**:

```rust
use predicates::prelude::*;

cmd.assert()
    .failure()
    .stderr(predicate::str::contains("DG001"))
    .stderr(predicate::str::contains("Anchor must be followed"));
```

#### 3. Why tempfile?

**Benefits**:

- Automatic cleanup (RAII)
- Prevents test pollution
- Works across platforms

**Example**:

```rust
use tempfile::TempDir;

let tmp = TempDir::new().unwrap();
let doc = tmp.path().join("test.md");
fs::write(&doc, "content").unwrap();
// tmp is automatically cleaned up
```

#### 4. Why cargo llvm-cov?

**Benefits**:

- Measures coverage for both unit and E2E tests
- Captures coverage from subprocess execution (CLI binary)
- Generates multiple output formats (HTML, LCOV, JSON)
- Integrates with CI/CD (Codecov, Coveralls)

**Commands**:

```bash
# Run all tests with coverage
cargo llvm-cov --tests

# Generate HTML report
cargo llvm-cov --tests --html

# Generate LCOV for CI
cargo llvm-cov --tests --lcov --output-path lcov.info
```

### Test Organization

#### Directory Structure

```text
tests/
├── e2e_check.rs      # Test 'check' command
├── e2e_rule.rs       # Test 'rule' command
├── e2e_graph.rs      # Test 'graph' command
├── e2e_list.rs       # Test 'list' command
├── e2e_trace.rs      # Test 'trace' command
├── e2e_describe.rs   # Test 'describe' command
└── common/
    └── mod.rs        # Shared test fixtures
```

#### Test Categories

For each command, test:

1. **Success cases**: Valid inputs, expected outputs
2. **Failure cases**: Invalid inputs, error messages
3. **File I/O**: Reading/writing files, directory handling
4. **Edge cases**: Empty inputs, missing files, etc.

#### Example: `tests/e2e_check.rs`

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn check_valid_doc_succeeds() {
    let tmp = TempDir::new().unwrap();
    let doc = tmp.path().join("test.md");
    fs::write(&doc, "<a id=\"TEST-01\"></a>\n# Test\n").unwrap();

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No errors"));
}

#[test]
fn check_invalid_doc_fails() {
    let tmp = TempDir::new().unwrap();
    let doc = tmp.path().join("test.md");
    fs::write(&doc, "<a id=\"TEST-01\"></a>\n").unwrap(); // Missing heading

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("DG001"));
}

#[test]
fn check_json_output() {
    let tmp = TempDir::new().unwrap();

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg("--json")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::is_json());
}

#[test]
fn check_with_fix_flag() {
    let tmp = TempDir::new().unwrap();
    let doc = tmp.path().join("test.md");
    fs::write(&doc, "fixable content").unwrap();

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg("--fix")
        .arg(tmp.path())
        .assert()
        .success();

    // Verify file was modified
    let content = fs::read_to_string(&doc).unwrap();
    assert!(content.contains("fixed"));
}
```

### Relationship to Existing Tests

E2E tests **complement** existing unit tests:

| Test Type | Purpose | Coverage Target |
|-----------|---------|----------------|
| **Unit Tests** | Test core business logic | Core layer (86-100%) |
| **E2E Tests** | Test CLI behavior, I/O, error messages | CLI handlers (0% → 80-90%) |

This aligns with the **Thin Handlers** principle:

- Handlers contain minimal logic (input/output transformation)
- E2E tests verify handler behavior without duplicating core logic tests
- Core logic remains thoroughly unit tested

### Coverage Improvement

**Current Coverage**: 68.70%

- Core layer: 86-100%
- CLI handlers: 0%
- LSP handlers: 66-98%

**Expected After E2E Tests**: 75-80%

- Core layer: 86-100% (unchanged)
- CLI handlers: 80-90% (E2E tests)
- LSP handlers: 66-98% (unchanged)

### CI/CD Integration

Update `.github/workflows/ci.yml`:

```yaml
- name: Install llvm-cov
  run: |
    rustup component add llvm-tools-preview
    cargo install cargo-llvm-cov

- name: Run tests with coverage
  run: cargo llvm-cov --tests --lcov --output-path lcov.info

- name: Upload coverage to Codecov
  uses: codecov/codecov-action@v3
  with:
    files: lcov.info
```

### Consequences

#### Positive

- **Complete CLI testing**: All commands tested end-to-end
- **User-facing validation**: Error messages and output formats verified
- **Coverage visibility**: E2E coverage measured and tracked
- **Regression prevention**: CLI behavior changes caught by tests
- **CI integration**: Coverage reports in pull requests

#### Negative

- **Slower tests**: E2E tests are slower than unit tests (subprocess overhead)
- **Additional dependencies**: assert_cmd, predicates, tempfile
- **Maintenance**: E2E tests need updates when CLI changes
- **Flakiness risk**: File I/O and process spawning can be flaky

#### Mitigation

- Run E2E tests in parallel where possible
- Use `tempfile` to prevent test pollution
- Keep E2E tests focused on CLI behavior, not core logic
- Use `cargo llvm-cov --tests` in CI to ensure coverage is measured

### LSP Testing Strategy

LSP handlers require a different testing approach than CLI handlers.

#### Why Unit Tests for LSP, Not E2E?

**Complexity of LSP E2E Testing:**

- Requires LSP client simulation
- JSON-RPC protocol overhead
- Async communication complexity
- Limited tooling support (tower-lsp-testing is experimental)

**Unit Tests Are Sufficient:**

- LSP handlers are thin wrappers (similar to CLI handlers)
- Core logic is already unit tested
- Handler-specific logic can be tested with mock data
- Existing unit tests already cover main functionality

#### LSP Test Coverage Strategy

| Handler | Current Coverage | Strategy |
|---------|-----------------|----------|
| **completion.cs** | 97.92% | ✅ Sufficient |
| **definition.rs** | 89.01% | ✅ Sufficient |
| **hover.rs** | 79.31% | 🔄 Improve with edge cases |
| **references.rs** | 71.67% | 🔄 Add multi-file tests |
| **rename.rs** | 65.83% | 🔄 Add error cases |
| **backend.rs** | 0.00% | ⚠️ Delegation only, no test needed |
| **call_hierarchy.rs** | 0.00% | ⚠️ Delegation only, no test needed |
| **mod.rs** | 0.00% | ⚠️ Module exports only |

**Rationale for 0% Coverage:**

- `backend.rs`: Pure delegation to handlers, no business logic
- `call_hierarchy.rs`: Async wrapper, logic tested via integration
- `mod.rs`: Module exports only

**Action Items:**

1. **Short-term**: Improve unit tests for hover, references, rename
2. **Medium-term**: Add integration tests if needed
3. **Long-term**: Consider LSP E2E framework when tooling matures

### Related
