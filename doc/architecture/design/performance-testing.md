<a id="CC_PERF_TESTING"></a>

# Performance Testing

## Strategy

Performance is a critical non-functional requirement for Docgraph, especially given the recursive nature of graph validation.

### Benchmarking

We use `criterion` for micro-benchmarks.

- **Location**: `benches/` directory.
- **Targets**: Critical paths in `core` logic (e.g., parsing, linting).

### Running Benchmarks

```bash
cargo bench
```

## Goals

- **Linting**: < 50ms for typical workspaces.
- **Graph Generation**: Scalable to 1000+ nodes.
