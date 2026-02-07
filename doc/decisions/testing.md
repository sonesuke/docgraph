<a id="ADR_TESTING"></a>

# Testing

## Decision

We adopt a multi-layered testing approach:

1. **Unit Tests**: Fast, isolated tests for core logic
2. **E2E Tests**: Integration tests for CLI and LSP
3. **Performance Testing**: Benchmarks for critical paths
4. **Code Coverage**: Using `cargo-llvm-cov` for measurement

## Rationale

- **Isolation**: Tests run independently and quickly
- **Coverage Target**: 85%+ for core business logic, 70%+ overall
- **Tools**: `cargo test`, `cargo bench`, `cargo llvm-cov`

## Context

We need a comprehensive testing strategy to ensure code quality and correctness across the entire codebase.
