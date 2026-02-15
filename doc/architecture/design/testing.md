# Testing Strategy

<a id="CC_TESTING_STRATEGY"></a>

## Testing Strategy

We employ a pyramid testing strategy to ensure reliability and velocity.

**Pyramid:**

1.  **Unit Tests (Bottom)**: Fast, isolated, high coverage.
2.  **Integration Tests (Middle)**: Verify component interactions.
3.  **E2E Tests (Top)**: Verify full system behavior via LSP.

---

<a id="CC_UNIT_TESTING"></a>

## Unit Testing

Unit tests should cover all public functions and complex private logic.

**Principles:**

- **Isolation**: Tests must not depend on external systems (filesystem, network).
- **Speed**: Tests must run per commit.
- **Coverage**: Aim for high branch coverage in core logic.

---

<a id="CC_E2E_TESTING"></a>

## End-to-End (E2E) Testing

E2E tests verify the system from the user's perspective (LSP client).

**Scope:**

- **LSP Lifecycle**: Initialize, Shutdown, Exit.
- **Diagnostics**: Verify reporting of graph errors.
- **Resilience**: Verify recovery from invalid states.

**Tools:**

- Rust standard test framework with a mock LSP client.

---

<a id="CC_COVERAGE"></a>

## 3. Code Coverage Standards

We use `cargo-llvm-cov` to measure test effectiveness.

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

**Derived from:** [ADR_PERF (Performance Logic)](../../decisions/perf.md#ADR_PERF)

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
