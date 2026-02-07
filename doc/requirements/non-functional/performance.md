# Performance Requirements

<a id="NFR_PERF"></a>

## High Performance

The system must be highly performant to ensure a smooth user experience, even with large documentation sets.

**Criteria:**

- Linting 1000 nodes should take less than 1 second.
- Graph analysis operations should be O(N) where possible.

**Realized by:**

- [CC_PERF_TESTING (4. Performance Testing)](../../architecture/design/testing.md#CC_PERF_TESTING)

---

<a id="NFR_AVAILABILITY"></a>

## System Availability

The docgraph tool should be available and operational at least 99.9% of the time in CI environments.

**Realized by**: [MOD_CORE (Core Modules)](../../architecture/view/module.md#MOD_CORE)

---

<a id="NFR_LATENCY"></a>

## System Latency

Interactive operations in the VS Code extension (hover, completion) should respond in less than 100ms.

**Realized by**: [MOD_LSP (LSP Modules)](../../architecture/view/module.md#MOD_LSP)
