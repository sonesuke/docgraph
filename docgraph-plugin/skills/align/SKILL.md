---
name: refine
description: Deep Consistency Gate - Verify Vertical and Horizontal relationship integrity.
---

# Deep Consistency Gate

This skill serves as the **gate for depth and relationship integrity** within the documentation graph. It focuses on ensuring that nodes are not only correct in isolation (as verified by `validate`) but also perfectly aligned with their context (Vertical) and their peers (Horizontal).

> [!IMPORTANT]
> The refinement must ensure that every node's purpose is unambiguous and its place in the hierarchy is fully justified. **Flag any semantic "fog" or responsibility overlap.**

## Workflow Steps

### 0. Validation Pre-requisite
- **Level**: STRICT
- **Policy**: Ensure the node has already passed the `validate` skill (Quality Gate).

### 1. Vertical Consistency (Traceability & Context)
- **Level**: STRICT for missing links, HEURISTIC for semantic clarity.
- **Purpose**: Verify that the node "fits" between its parents and children.

1. **Context Check**: Use `docgraph describe <ID>` to see parent nodes (inbound relations). 
   - Does the parent node explicitly require the responsibility defined in the target node?
   - Is there any gap or ambiguity in how the parent's intent is carried over?
2. **Realization Check**: Check child nodes (outbound relations).
   - Is the target node's responsibility fully realized by its children?
   - Are there any child nodes that seem unrelated to the target's purpose?

### 2. Horizontal Consistency (Peer Alignment & MECE)
- **Level**: HEURISTIC
- **Purpose**: Ensure the node is Mutually Exclusive and Collectively Exhaustive (MECE) with its peers.

1. **Peer Identification**: Use `docgraph list "<PREFIX>_*"` to find nodes of the same type.
2. **Overlap Check**: Compare the target node's responsibility with its peers.
   - Does this node overlap with another existing node?
   - If a responsibility belongs to multiple nodes, propose a split or consolidation.

### 3. Responsibility Refinement (SRP Depth)
- **Level**: HEURISTIC
- **Purpose**: Deepen the SRP check from `validate`.

1. **Granularity Check**: Is the responsibility too high-level or too low-level for this type's baseline?
2. **Cohesion Check**: Are all elements within the node tightly related?

### 4. Refinement Proposals
Propose changes based on analysis:
- **Clarify Context**: Suggest adding or explicitizing links to parents.
- **Adjust Scope**: Propose narrowing or broadening the node to better fit its peers.
- **Split/Merge**: If Horizontal consistency is broken, suggest structural changes.

## Refinement Analysis Report
You **must** provide the analysis in the following format:

### Target
- **Node**: [ID]
- **Type**: [Type Name]

### Consistency Analysis
| Dimension | Check Item | Result | Analysis / Evidence |
|:---|:---|:---|:---|
| **Vertical** | Parent Alignment | PASS/FAIL | |
| **Vertical** | Child Coverage | PASS/FAIL | |
| **Horizontal** | Peer Overlap (MECE) | PASS/FAIL | |
| **Quality** | Semantic Clarity | PASS/FAIL | |

## Final Decision
- **PASS**: The node shows deep integrity and fits perfectly within the graph.
- **FAIL**: Structural or semantic issues identified. Remediation required.

**FINAL DECISION: [PASS/FAIL]**

### Case 2: Input is a NODE_ID (e.g., FR_LOGIN, UC_CHECKOUT)

1. **Show Details**: Use `docgraph describe`.

2. **SRP Check (Single Responsibility Principle)**:
   - **Too many responsibilities?**: Does this single node try to do too much?
   - **Vague definition?**: Is the scope defined clearly?

3. **Realizability Check**:
   - **Sufficient components?**: Do the linked child nodes/dependencies sufficiently realize the responsibilities?
   - **Missing dependencies?**: Identify paths that lack justification or realization.

4. **Remediation**:
   - **Split Node**: If it has too many responsibilities, propose splitting it.
   - **Add Dependencies**: If realizability is low, propose adding missing links.
