---
name: validate
description: Graph Integrity Gate - Ensure nodes and relations can exist in the causal graph.
---

# Graph Integrity Gate

This skill determines whether a node and its local causal subgraph can exist.

Validation does not evaluate quality or design semantics. It verifies structural conditions required for causal
traversal.

> [!IMPORTANT] A node is valid only if its presence does not create a logical contradiction in the graph. Cosmetic or
> stylistic issues must NOT cause failure.

---

## Core Principle

Validation checks **existence**, not interpretation.

- If validation fails → the graph contains a structural contradiction
- If validation passes → reasoning may safely operate

Validation rejects impossible structures, not incomplete ones.

Validation guarantees the graph is a possible world, not a complete world.

---

## 0. Schema Topology Understanding

Before running validation, the agent MUST read `docgraph.toml` to establish the allowed structural topology.

`docgraph.toml` is the sole normative source. If unread, the agent MUST stop.

The agent MUST extract:

- Allowed node types and their identifiers
- Allowed relation types (`r.type`) and their directionality
- Allowed target types for each relation
- Required and forbidden relation rules

All validation decisions MUST be justified against this topology.

---

## 1. Automated Structural Checks

Run automatic checks before manual inspection.

```bash
docgraph check
```

If this fails → **FAIL immediately**

---

## 2. Schema Integrity

All nodes and relations must conform to `docgraph.toml`.

Failure cases:

- Undefined node type
- Undefined relation type
- Disallowed relation direction

Result: FAIL

---

## 3. Referential Integrity

All references must resolve.

- inbound references resolve
- outbound references resolve

Dangling references → FAIL

---

## 4. Identity Uniqueness

Node identifiers must be globally unique.

Duplicate ID → FAIL

---

## 5. Causal Contradictions

The local graph must not contain logical contradictions that prevent traversal.

A structure is invalid only if realization is impossible, not merely incomplete.

Examples:

- constraints block all realizations
- cyclic dependencies preventing resolution
- mutually exclusive requirements
- relation semantics conflict
- schema makes realization unreachable

Any detected → FAIL

---

## Minimal Tooling

### `docgraph check`

```bash
docgraph check
```

### `docgraph describe` (optional)

Use only when confirming unresolved references around a specific node.

```bash
docgraph describe <ID>
```

---

## What Validation Does NOT Check

The following MUST NOT cause failure:

- naming elegance
- directory placement
- template formatting
- SRP granularity
- documentation completeness

These belong to alignment or hygiene review.

---

## Validation Report

### Target

- **Node**: [ID]

### Structural Results

| Check                 | Result    | Notes |
| :-------------------- | :-------- | :---- |
| Schema topology       | READ      |       |
| Schema integrity      | PASS/FAIL |       |
| Referential integrity | PASS/FAIL |       |
| Identity uniqueness   | PASS/FAIL |       |
| Causal contradictions | PASS/FAIL |       |

---

## Final Decision

**PASS** → Node may exist in the graph **FAIL** → Graph integrity would be broken

**FINAL DECISION: [PASS/FAIL]**
