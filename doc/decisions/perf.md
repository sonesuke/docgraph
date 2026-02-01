# Performance Requirements

<a id="ADR_PERF"></a>

## Context

Performance is critical for recursive graph validation. Users expect fast feedback during development.

## Decision

We use `criterion` for micro-benchmarks and set performance targets:

- **Linting**: < 50ms for typical workspaces
- **Graph Generation**: Scalable to 1000+ nodes

## Consequences

- Benchmarks located in `benches/` directory
- Performance tests run in CI/CD pipeline
- Critical paths in `core` logic are optimized
