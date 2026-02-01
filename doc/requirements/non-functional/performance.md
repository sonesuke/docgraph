# Performance Requirements

<a id="NFR_PERF"></a>

## High Performance

The system must be highly performant to ensure a smooth user experience, even with large documentation sets.

**Criteria:**

- Linting 1000 nodes should take less than 1 second.
- Graph analysis operations should be O(N) where possible.

**Realized by:**

- [CON_PERF (High Performance)](../../constraints/development.md#CON_PERF)
- [CC_PERF_TESTING (Performance Testing)](../../architecture/design/performance-testing.md#CC_PERF_TESTING)
