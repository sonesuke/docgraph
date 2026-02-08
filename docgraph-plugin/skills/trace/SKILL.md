---
name: trace
description: Realization & Flow Gate - Verify Downstream Traceability and Realizability.
---

# Realization & Flow Gate

This skill verifies that a node’s responsibility is fully traceable downstream and ultimately realizable. It focuses on **paths**, ensuring that abstract intent can be traced to concrete realization.

> [!IMPORTANT]
> A node is considered valid only if its intent can be traced through coherent responsibility transitions to at least one **realizable terminal**. Flag any path that stalls, breaks, or drifts semantically.

## Workflow Steps

### 0. Pre-requisites
- **Level**: STRICT
- **Policy**: 
  - The target node **MUST** have passed `validate` and `align`.
  - Do not re-check hygiene (naming, structure) or peer alignment.

### 1. Downstream Expansion (Graph Traversal)
- **Goal**: Enumerate all possible downstream realization paths.
- **Process**:
  1. Starting from the target node, traverse all outbound relations.
  2. Traversal **MUST** be recursive.
  3. Traversal stops when a **terminal type** (BB, MOD, IF, DEP, CODE) is reached or a cycle is detected.

### 2. Terminal Reachability Check
- **Purpose**: Ensure at least one path reaches a realizable endpoint.
- **Terminal Types**: `BB`, `MOD`, `IF`, `DEP`, `CODE`.
- **Fail Rule**: FAIL if all paths terminate at abstract nodes (FR, UC, ADR) without reaching a terminal.

### 3. Abstraction Level Monotonicity
- **Goal**: Ensure abstraction decreases (or stays same) as we go downstream.
- **Check**: No "jump back" to higher abstraction levels is allowed (e.g., `BB` -> `FR` is a failure).
- **Metric**: `FR` (High) > `ADR/UC` (Mid) > `MOD/BB` (Low) > `CODE/IF` (Terminal).

### 4. Responsibility Preservation (Semantic Continuity)
- **Goal**: Ensure the core intent survives the journey.
- **Semantic fog check**: Detect:
  - **Semantic dilution**: Intent becomes too vague.
  - **Responsibility drift**: Intent changes into something else.
  - **Silent renaming**: Key terms change without justification.

### 5. Realizability Check (Terminal Adequacy)
- **Goal**: Confirm that the terminal node can actually realize the responsibility.
- **Rule**: Terminal nodes must define concrete behavior or interfaces. "Concept-only" terminals are failures.

### 6. Path Sufficiency & Redundancy
- **Check**: At least one complete and coherent path exists.
- **Check**: Are multiple paths alternative realizations or accidental duplications?

## Trace Analysis Report
You **must** provide the analysis in the following format:

### Target
- **Node**: [ID]
- **Terminal Goal**: [e.g., BB_ nodes]

### Trace Summary
| Path | Reaches Terminal | Abstraction OK | Semantic Continuity | Realizable |
|:---|:---|:---|:---|:---|
| Path A | PASS/FAIL | PASS/FAIL | PASS/FAIL | PASS/FAIL |
| Path B | ... | ... | ... | ... |

### Findings
- **Continuity**: [Evidence of drift or preservation]
- **Traversal**: [List identified paths]

## Final Decision
### Decision Semantics
- **PASS**: At least one path is complete, monotonic, semantically coherent, and realizable.
- **FAIL**: All paths break, stall, or drift.

**FINAL DECISION: [PASS/FAIL]**
