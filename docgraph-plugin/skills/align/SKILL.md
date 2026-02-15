---
name: align
description: Deep Consistency Gate - Verify Vertical and Horizontal relationship integrity.
---

# Deep Consistency Gate (Architecture & Meaning)

This skill serves as the **gate for depth and relationship integrity** within the documentation graph. It focuses on
ensuring that nodes are not only correct in isolation (as verified by `validate`) but also perfectly aligned with their
context (Vertical) and their peers (Horizontal).

> [!IMPORTANT] This is an **Architecture & Meaning Gate**. Flag any semantic "fog" (unclear boundaries, implicit
> assumptions, or overloaded terms). Every node must be fully justified by its context.

## Prerequisites

- **`docgraph` CLI must be installed as a system binary**
  - Install via: `curl -fsSL https://raw.githubusercontent.com/sonesuke/docgraph/main/install.sh | bash`
  - Or build from source: `cargo install --path .`
- **This is NOT an npm or Python package** - do NOT use `npx` or `pipx`
- **Installation Verification**: Run `docgraph --version` to confirm the binary is available

## Workflow Steps

### 0. Structural & Validation Pre-requisite

- **Level**: STRICT
- **Policy**:
  1. **Schema Context**: Read `docgraph.toml` to understand the valid node types and relationship rules for the target scope.
  2. **Validation Status**: If `validate` status is unknown or FAIL -> **STOP** and return FAIL.
  - Do not re-evaluate surface items (naming, templates) already covered by `validate`.

### 1. Vertical Consistency (Traceability & Context)

- **Level**: STRICT for missing links, HEURISTIC for semantic clarity.
- **Vertical Expectations**:
  - **Parents (Inbound)**: Define the "Why" (intent, requirement, or goal).
  - **This Node**: Defines the "What" at its specific abstraction level.
  - **Children (Outbound)**: Define the "How" (realization, implementation, or breakdown).

1. **Context Check**: Use `docgraph describe <ID>` or `docgraph query` and verify:
   - Does the parent node explicitly justify the existence of this node?
   - Is there any gap in how the parent's intent is carried over?
2. **Realization Check**: Verify child nodes:
   - Is this node's responsibility fully and exclusively covered by its children?

### 2. Horizontal Consistency (Peer Alignment & MECE)

- **Level**: HEURISTIC
- **Baseline Rule**: Use the dominant pattern among existing peer nodes. Do not invent new abstraction levels unless
  proposing an explicit refactor.

1. **Peer Identification**: Use `docgraph query`.
   - Example: `docgraph query "MATCH (n:FR) WHERE n.id STARTS WITH 'FR_AUTH' RETURN n.id"`
2. **Overlap Check**: Verify Mutually Exclusive and Collectively Exhaustive (MECE) status.
   - Does this node's responsibility overlap with peer nodes?
   - Is the granularity consistent with the peer baseline?

### 3. Structural SRP Check

- **Level**: HEURISTIC
- **Note**: `validate` checks surface SRP; `align` checks structural SRP (depth, cohesion, and abstraction fit).

1. **Cohesion**: Are all elements within this node tightly related to the "What" definition?
2. **Abstraction**: Is the node at the correct level relative to its parents and peers?

### 4. Proposals & Impact Analysis

- **Level**: MANDATORY

When proposing changes (Clarify Context, Split, Merge, or Move):

1. **Affected Nodes**: List all nodes (parents, peers, children) that will be affected.
2. **Re-validation**: Indicate whether `validate` must be re-run for any affected nodes.
3. **Safety**: Ensure no existing references are broken without a remediation plan.

## Workflow Cases

### Case 1: TYPE_ID (e.g., FR, MOD)

- Perform a full graph consistency review for the given type.

### Case 2: NODE_ID (e.g., FR_LOGIN)

- **Status**: Focused Refinement.
- Perform a deep analysis specifically for the node and its immediate relations. Do not scan the entire graph unless
  necessary for baseline identification.

## Alignment Analysis Report

You **must** provide the analysis in the following format:

### Target

- **Node/Scope**: [ID or Type]
- **Baseline Peer Pattern**: [Description of dominant convention]

### Consistency Analysis

| Dimension        | Check Item      | Result    | Analysis / Evidence |
| :--------------- | :-------------- | :-------- | :------------------ |
| **Prerequisite** | Validate PASS   | PASS/FAIL |                     |
| **Vertical**     | Parents (Why)   | PASS/FAIL |                     |
| **Vertical**     | Children (How)  | PASS/FAIL |                     |
| **Horizontal**   | Peer MECE       | PASS/FAIL |                     |
| **SRP**          | Abstraction Fit | PASS/FAIL |                     |

### Refinement Proposals

- **Proposal**: [Description]
- **Affected IDs**: [List]
- **Re-validate Required**: [Yes/No]

### Quality Gate Checklist

In your final report, you **must** include this checklist to demonstrate deep architectural verification:

- [ ] **Prerequisite PASS**: The node has successfully cleared the `validate` skill (Quality Gate).
- [ ] **Vertical Alignment**: Why (Parent), What (This Node), and How (Child) are semantically consistent.
- [ ] **Horizontal MECE**: Responsibility is mutually exclusive and follows the dominant peer baseline.
- [ ] **Semantic Clarity**: No "semantic fog" or ambiguous boundaries identified.
- [ ] **Impact Analysis**: All affected nodes are listed, and re-validation needs are clearly stated.

## Final Decision

### Decision Semantics

- **PASS**: Node shows deep integrity and may be merged/applied.
- **FAIL**: Structural or semantic issues identified. MUST NOT be merged.

**FINAL DECISION: [PASS/FAIL]**
