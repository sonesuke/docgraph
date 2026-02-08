---
name: refine
description: Verify and refine element responsibilities (Supports TYPE_ID or NODE_ID).
---

# Refinement Skill

This skill provides a structured workflow to verify responsibilities and ensure the quality of the documentation graph.

> [!NOTE]
> For detailed usage and available options of any `docgraph` subcommand, always refer to `docgraph --help` or `docgraph <SUBCOMMAND> --help`.

## Workflow

### Case 1: Input is a TYPE_ID (e.g., FR, SYS, ACT, UC)

1. **Show Type Definition**: Understand the rules and description of the type using `docgraph type`.

2. **List All Elements**: Gather all instances of this type using `docgraph list`.

3. **Holistic Analysis (MECE)**: Verify if the elements are Mutually Exclusive and Collectively Exhaustive.
   - Identify potential overlaps and gaps.
   - Use `docgraph describe` to confirm detailed content of suspicious nodes.

4. **Remediation**:
   - **Explain Overlap**: Clearly state what is overlapping and how.
   - **Propose Options**: Suggest merging, deleting, or restructuring.

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
