---
name: reasoning
description:
  Graph Reasoning - Navigate the documentation graph to progressively refine a hypothesis until a stable conclusion is
  formed.
---

# Graph Reasoning

This skill defines how the agent thinks using the documentation graph.

The graph is not a database to search. It is a reasoning space to traverse.

The agent MUST navigate nodes to progressively refine a hypothesis until a stable conclusion is formed.

> [!IMPORTANT] The agent does not retrieve answers. It stabilizes understanding by traversing causality.

---

## 0. Semantic Topology Understanding

Before any traversal, the agent MUST interpret the meaning of the graph structure.

**Action**: Read `docgraph.toml` in the project root.

The agent reads `docgraph.toml` and classifies each node type into a reasoning role:

| Role                      | Meaning                             | Example Node Types |
| :------------------------ | :---------------------------------- | :----------------- |
| **Intent** (WHY)          | Purpose and motivation              | UC, ACT            |
| **Constraint** (BOUNDARY) | Non-negotiable conditions           | CON                |
| **Responsibility** (WHAT) | Obligations the system must fulfill | FR, NFR            |
| **Realization** (HOW)     | Concrete mechanisms and structures  | MOD, IF, CC        |
| **Evidence** (PROOF)      | Verification artifacts (optional)   | TEST, MET, LOG     |
| **Rationale** (DECISION)  | Design justification                | ADR                |
| **Domain** (CONTEXT)      | Shared data models                  | DAT                |

### Semantic Gravity

The agent MUST never move arbitrarily across node categories.

Movement is constrained by semantic gravity:

- **Intent** pulls toward purpose
- **Responsibility** pulls toward obligation
- **Realization** pulls toward mechanism
- **Evidence** pulls toward validation
- **Constraint** pulls toward non-negotiable invariants
- **Rationale** pulls toward justification

If the agent cannot name the gravitational pull driving a move, the move is invalid.

---

## 1. Reasoning Loop

The agent operates in a continuous loop:

1. **Hold a hypothesis** — a claim about the graph's state
2. **Move to the next node** — following semantic gravity
3. **Update the hypothesis** — based on what was found

This is not search. This is exploration.

The loop terminates when the hypothesis is stable: no further traversal changes the conclusion.

---

## 2. Allowed Reasoning Moves

Every move MUST preserve causal meaning. Each edge carries a question the agent is asking.

```
Intent → Responsibility
    "What must be satisfied?"

Responsibility → Realization
    "How is it achieved?"

Realization → Realization
    "What depends on this?"

Responsibility → Constraint
    "What boundaries apply?"

Realization → Rationale
    "Why was this design chosen?"

Realization → Evidence
    "Is it proven?"

Evidence → Responsibility
    "What failed or needs adjustment?"
```

The `r.type` field in `docgraph.toml` (`rel`) names the semantic relationship.

A move is valid only if `r.type` matches the question of that move. If the mapping is unknown, the agent MUST stop and
re-classify the edge semantics from `docgraph.toml`.

---

## 3. Reasoning Patterns

### 3.1 Closure (Is it complete?)

Start from a node. Attempt to reach a terminal. If no path reaches a Realization or Evidence node, the structure is
incomplete.

```bash
docgraph query "MATCH (n)-[r*1..3]->(m:MOD) WHERE n.id = '<ID>' RETURN m.id, r.type"
```

### 3.2 Forward (How is it realized?)

Start from Intent or Responsibility. Follow semantic gravity downward. Each step asks: "How does this become concrete?"

```bash
docgraph query "MATCH (n)-[r]->(m) WHERE n.id = '<ID>' RETURN m.id, m.type, r.type"
```

### 3.3 Backward (Why does this exist?)

Start from Realization. Follow relationships upward. Each step asks: "What justified this?"

```bash
docgraph query "MATCH (n)-[r]->(m) WHERE m.id = '<ID>' RETURN n.id, n.type, r.type"
```

### 3.4 Counterfactual (What breaks if removed?)

Hypothetically remove a node. Trace all dependents. A node loses its justification when all incoming edges with a
justification `r.type` (e.g., `used_by`, `realized_by`, `constrained_by`) originate from the removed node.

```bash
docgraph query "MATCH (n)-[r]->(m) WHERE m.id = '<ID>' RETURN n.id, r.type"
```

---

## 4. Tools

Tools materialize reasoning decisions. They do not drive them.

> [!CAUTION] Cypher MUST NOT be used to discover what to think. Cypher is only used to materialize a reasoning decision
> already made.
>
> If the agent starts from a query instead of a hypothesis, it is performing search, not reasoning.

### `docgraph query`

Executes a Cypher query to confirm or refute a hypothesis.

```bash
docgraph query "<CYPHER_QUERY>"
```

### `docgraph describe`

Inspects a single node and its immediate relationships.

```bash
docgraph describe <ID>
```

---

## 5. Reasoning Report

Structure findings as reasoning conclusions, not search results.

### Hypothesis

- **Initial**: [What the agent believed before traversal]
- **Final**: [What the agent concluded after traversal]

### Traversal Trace

Show the path taken and the reasoning at each step:

```
UC_001 --(uses)--> FR_001 --(realized_by)--> MOD_001
  "What must be satisfied?"  "How is it achieved?"
```

### Stability Assessment

- **Conclusion Stable**: [Yes/No]
- **Unresolved Nodes**: [List of nodes that could not be reached or justified]
- **Integrity**: [PASS/FAIL]
