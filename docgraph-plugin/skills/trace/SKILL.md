---
name: trace
description: Realization & Flow Gate - Verify Downstream Traceability and Realizability.
---

# Realization & Flow Gate

This skill verifies that a node’s responsibility is fully traceable downstream and ultimately realizable. It focuses on
**paths**, ensuring that abstract intent can be traced to concrete realization.

> [!IMPORTANT] A node is considered valid only if its intent can be traced through coherent responsibility transitions
> to at least one **realizable terminal**. A FAIL in `trace` indicates an unrealizable or misleading specification, not
> merely incomplete documentation.

## Prerequisites

- **`docgraph` CLI must be installed as a system binary**
  - Install via: `curl -fsSL https://raw.githubusercontent.com/sonesuke/docgraph/main/install.sh | bash`
  - Or build from source: `cargo install --path .`
- **This is NOT an npm or Python package** - do NOT use `npx` or `pipx`
- **Installation Verification**: Run `docgraph --version` to confirm the binary is available

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
  3. Traversal stops when a **terminal type** (project-defined realization boundaries: `BB`, `MOD`, `IF`, `DEP`, `CODE`)
     is reached or a cycle is detected.

### 2. Terminal Reachability Check

- **Purpose**: Ensure at least one path reaches a realizable endpoint.
- **Fail Rule**: FAIL if all paths terminate at abstract nodes (FR, UC, ADR) without reaching a terminal boundary.

### 3. Abstraction Level Monotonicity

- **Goal**: Ensure abstraction decreases as we go downstream.
- **Rules**:
  - Abstraction levels: `FR` (High) > `ADR/UC` (Mid) > `MOD/BB` (Low) > `CODE/IF` (Terminal).
  - No "jump back" to higher abstraction levels is allowed.
  - Abstraction may stay the same ONLY if the transition represents an explicit **refinement or decomposition** of
    responsibility. Lateral transitions without semantic narrowing MUST be flagged.

### 4. Responsibility Preservation (Semantic Continuity)

- **Goal**: Ensure the core intent survives the journey.
- **Process**: Identify the semantic core from the target node’s title and primary description.
- **Detect**:
  - **Semantic dilution**: Intent becomes too vague.
  - **Responsibility drift**: Intent changes into something else.
  - **Silent renaming**: Key terms change without justification.

### 5. Realizability Check (Terminal Adequacy)

- **Goal**: Confirm that the terminal node can actually realize the responsibility.
- **Rule**: Terminal nodes must define concrete behavior or interfaces. "Concept-only" terminals are failures.

### 6. Path Sufficiency & Redundancy

- **Check**: **PASS requires at least one fully coherent path.**
- **Check**: Multiple paths must be alternative realizations. Additional incomplete or broken paths MUST still be
  reported as findings.

## Trace Analysis Report

You **must** provide the analysis in the following format:

### Target

- **Node**: [ID]
- **Terminal Goal**: [e.g., BB_ nodes]

### Trace Summary

| Path   | Reaches Terminal | Abstraction OK | Semantic Continuity | Realizable | Failure Cause (if any)                |
| :----- | :--------------- | :------------- | :------------------ | :--------- | :------------------------------------ |
| Path A | PASS/FAIL        | PASS/FAIL      | PASS/FAIL           | PASS/FAIL  | [e.g., Stalled at UC, Semantic drift] |
| Path B | ...              | ...            | ...                 | ...        | ...                                   |

### Findings

- **Continuity**: [Evidence of drift or preservation from the semantic core]
- **Traversal**: [Brief list of identified paths]

## Final Decision

### Decision Semantics

- **PASS**: At least one path is complete, monotonic, semantically coherent, and realizable.
- **FAIL**: All paths break, stall, or drift.

**FINAL DECISION: [PASS/FAIL]**
