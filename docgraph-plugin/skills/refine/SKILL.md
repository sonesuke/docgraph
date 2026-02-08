---
name: refine
description: Verify and refine element responsibilities (Supports TYPE_ID or NODE_ID).
---

# Refinement Skill

This skill provides a structured workflow to verify responsibilities and ensure the quality of the documentation graph.

## Workflow

### Case 1: Input is a TYPE_ID (e.g., FR, SYS, ACT, UC)

1. **Show Type Definition**: Understand the rules and description of the type.
   ```bash
   docgraph type {{TARGET}}
   ```

2. **List All Elements**: Gather all instances of this type.
   ```bash
   docgraph list "{{TARGET}}*"
   ```

3. **Holistic Analysis (MECE)**: Verify if the elements are Mutually Exclusive and Collectively Exhaustive.
   - Identify potential overlaps based on IDs and Titles.
   - For overlaps, use `docgraph describe {{ID}}` to confirm detailed content.
   - Check for gaps in coverage relative to the type's purpose.

4. **Remediation**:
   - **Explain Overlap**: Clearly state what is overlapping and how.
   - **Propose Options**: Suggest merging, deleting, or restructuring.

### Case 2: Input is a NODE_ID (e.g., FR-LOGIN, UC-CHECKOUT)

1. **Show Details**:
   ```bash
   docgraph describe {{TARGET}}
   ```

2. **SRP Check (Single Responsibility Principle)**:
   - **Too many responsibilities?**: Does this single node try to do too much?
   - **Vague definition?**: Is the scope defined clearly?

3. **Realizability Check**:
   - **Sufficient components?**: Do the linked child nodes/dependencies sufficiently realize the responsibilities?
   - **Missing dependencies?**: Identify paths that lack justification or realization.

4. **Remediation**:
   - **Split Node**: If it has too many responsibilities, propose splitting it.
   - **Add Dependencies**: If realizability is low, propose adding missing links.
