---
name: align
description: Semantic Rigidity Gate - Ensure each node has a single unambiguous causal role.
---

# Semantic Rigidity Gate

This skill determines whether a node has a uniquely interpretable meaning in the graph.

Alignment does not evaluate completeness or quality. It eliminates semantic ambiguity that would cause reasoning
divergence.

> [!IMPORTANT] A node is aligned only if its meaning has exactly one causal role. If multiple interpretations are
> possible, alignment fails.

---

## Core Principle

Validation ensures the graph can exist. Alignment ensures the graph means only one thing.

If alignment fails → different agents may derive different conclusions.

Alignment verifies determinism of interpretation, not properties of the node alone.

Alignment guarantees that traversal meaning is path-independent.

---

## 0. Precondition

The node MUST pass `validate`.

If validation fails → STOP

---

## 1. Semantic Topology from Schema

After confirming `validate` PASS, the agent MUST read `docgraph.toml` to derive the semantic topology used for
alignment.

`docgraph.toml` is the sole normative source. If unread, the agent MUST stop.

The agent MUST map schema elements into interpretation constraints:

- Node type → causal role candidates (e.g., UC=Intent, FR=Responsibility, MOD=Realization, ADR=Rationale)
- Relation type (`r.type`) → allowed causal questions (Why/What/How/Boundary/Justification)
- Which relations define "peerhood" for horizontal comparison

Alignment MUST only judge determinism using this schema-derived topology. If the mapping cannot be established,
alignment MUST fail (unknown semantics).

---

## 2. Role Determinism

The node must belong to exactly one abstraction level:

- Intent (why)
- Responsibility (what)
- Realization (how)
- Constraint (boundary)
- Rationale (justification)

If the node can be interpreted as more than one → FAIL

The role must be inferable from relations, not from description wording. If role changes when ignoring prose, alignment
fails.

---

## 3. Vertical Determinism

Parent and child relations must not redefine the node's role.

Invalid cases:

- Parent treats node as requirement, child treats it as implementation
- Node alternates abstraction level depending on traversal direction

Result: FAIL

---

## 4. Semantic Collision

The graph must not provide multiple answers to the same causal question.

Invalid cases:

- synonymous responsibilities split across nodes
- overlapping definitions describing identical behavior
- two nodes satisfy the same parent relation
- peer boundaries depend on interpretation

Result: FAIL

---

## Minimal Tooling (Observation Only)

Alignment MUST use `docgraph` for observation. Tools materialize evidence; they do not drive exploration.

### `docgraph describe`

Inspect the target node and its immediate inbound/outbound relations.

```bash
docgraph describe <ID>
```

### `docgraph query` (bounded)

Use bounded queries only to detect ambiguity patterns, never to discover new intent.

Typical checks:

- Competing parents (multiple "Why" justifications)

```bash
docgraph query "MATCH (p)-[r]->(n) WHERE n.id='<ID>' RETURN p.id, p.type, r.type"
```

- Competing children (multiple "How" realizations that imply different roles)

```bash
docgraph query "MATCH (n)-[r]->(c) WHERE n.id='<ID>' RETURN c.id, c.type, r.type"
```

- Peer collision (potential synonyms within the same type)

```bash
docgraph query "MATCH (m:<TYPE>) RETURN m.id, m.title"
```

> [!CAUTION] Queries MUST be bounded (1-hop or limited results). If multi-hop traversal is required, switch to
> `reasoning`.

---

## What Alignment Does NOT Check

Alignment does not enforce:

- completeness
- coverage
- implementation presence
- formatting quality
- directory structure

---

## Alignment Report

### Target

- **Node**: [ID]

### Determinism Results

| Check                | Result    | Notes |
| :------------------- | :-------- | :---- |
| Schema topology      | READ      |       |
| Role determinism     | PASS/FAIL |       |
| Vertical determinism | PASS/FAIL |       |
| Semantic collision   | PASS/FAIL |       |

---

## Final Decision

**PASS** → Node meaning is uniquely defined **FAIL** → Node meaning is ambiguous

**FINAL DECISION: [PASS/FAIL]**
