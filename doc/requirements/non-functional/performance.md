# Performance Requirements

<a id="NFR_PERF"></a>

## High Performance

The system must be highly performant to ensure a smooth user experience, even with large documentation sets.

**Criteria:**

- Linting 1000 nodes should take less than 1 second.
- Graph analysis operations should be O(N) where possible.

### Codified in (Optional)

- [CC_PERF_TESTING (4. Performance Testing)](../../architecture/design/testing.md#CC_PERF_TESTING) Defines performance
  testing standards

---

<a id="NFR_AVAILABILITY"></a>

## System Availability

The docgraph tool should be available and operational at least 99.9% of the time in CI environments.

- [MOD_CORE (Core Library)](../../architecture/view/module.md#MOD_CORE)

---

<a id="NFR_LATENCY"></a>

## System Latency

Interactive operations in the VS Code extension (hover, completion) should respond in less than 100ms.

- [MOD_LSP (LSP Library)](../../architecture/view/module.md#MOD_LSP)
