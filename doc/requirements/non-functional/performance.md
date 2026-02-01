# Performance Requirements

<a id="NFR_PERF"></a>

## High Performance

The system must be highly performant to ensure a smooth user experience, even with large documentation sets.

**Criteria:**

- Linting 1000 nodes should take less than 1 second.
- Graph analysis operations should be O(N) where possible.

**Realized by:**

- [CC_PERF_TESTING (4. Performance Testing)](../../architecture/design/testing.md#CC_PERF_TESTING)
