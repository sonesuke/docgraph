<a id="CC_TESTING_STRATEGY"></a>

# Testing Strategy

We prioritize code quality and correctness through a multi-layered testing approach. This document outlines our strategies for Unit, E2E, Performance testing, and Code Coverage.

**Derived from:** [ADR_TESTING (Context)](../../decisions/testing.md#ADR_TESTING)

---

<a id="CC_UNIT_TESTING"></a>

## 1. Unit Testing Standards

**Principles:**

- **Isolation**: Tests should run independently.
- **Speed**: Unit tests must be fast to encourage frequent execution.
- **Coverage**: Core logic must have high validation coverage.

**Implementation:**

- **Unit Tests**: Located within source files (usually in `#[cfg(test)]` modules). Use for parsing unique patterns or validating specific rule logic.
- **Integration Tests**: Located in `tests/`. Test the behavior of CLI and core components as a whole set.

**Running Tests:**

```bash
cargo test
```

---

<a id="CC_E2E_TESTING"></a>

## 2. E2E Testing Design

**Overview:**

This outlines the design for End-to-End testing of the Docgraph CLI and LSP server using `tests/` directory structure.

**Directory Structure:**

```text
tests/
  cli.rs        # CLI test entry point
  cli/          # CLI test modules
  lsp.rs        # LSP test entry point
  lsp/          # LSP test modules
```

**CLI Testing:**

- Uses `assert_cmd` to test the CLI binary.
- Each test runs in a temporary directory with fresh configuration.

**LSP Testing:**

- Spawns LSP server as a child process.
- Communicates via `LspClient` harness over stdio.
- Scenarios cover completion, diagnostics, rename, etc.

---

<a id="CC_COVERAGE"></a>

## 3. Code Coverage Standards

We use `cargo-llvm-cov` to measure test effectiveness.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)

**Running Coverage Locally:**

1. Install: `cargo install cargo-llvm-cov`
2. Add component: `rustup component add llvm-tools-preview`
3. Generate report:

   ```bash
   cargo llvm-cov --html
   open target/llvm-cov/html/index.html
   ```

**CI/CD Pipeline:**

Every PR includes a `Cargo llvm-cov` step. We strive for high coverage in `core` logic.

---

<a id="CC_PERF_TESTING"></a>

## 4. Performance Testing

**Derived from:** [ADR_PERF (Context)](../../decisions/perf.md#ADR_PERF)

**Strategy:**

Performance is critical for recursive graph validation. We use `criterion` for micro-benchmarks.

- **Location**: `benches/` directory.
- **Targets**: Critical paths in `core` logic (parsing, linting).

**Running Benchmarks:**

```bash
cargo bench
```

**Goals:**

- **Linting**: < 50ms for typical workspaces.
- **Graph Generation**: Scalable to 1000+ nodes.
