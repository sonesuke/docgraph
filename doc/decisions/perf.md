<a id="ADR_PERF"></a>

# Performance Logic

Establishes performance goals and benchmarking strategies for the Docgraph workspace validation.

## Decision

We use `criterion` for micro-benchmarks and set performance targets:

- **Linting**: < 50ms for typical workspaces
- **Graph Generation**: Scalable to 1000+ nodes

## Rationale

- Benchmarks located in `benches/` directory
- Performance tests run in CI/CD pipeline
- Critical paths in `core` logic are optimized

## Context

Performance is critical for recursive graph validation. Users expect fast feedback during development.
