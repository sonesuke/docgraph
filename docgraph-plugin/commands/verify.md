---
description: Verify and refine element responsibilities (Supports TYPE_ID or NODE_ID)
---

Determine if the input `{{TARGET}}` is a TYPE_ID (e.g., FR, SYS, ACT, UC) or a NODE_ID (e.g., FR-001, UC-LOGIN).

## Case 1: Input is a TYPE_ID

1. Show type definition
   ```bash
   docgraph type {{TARGET}}
   ```

2. List all elements of this type
   ```bash
   docgraph list "{{TARGET}}*"
   ```

3. Analyze for MECE (Mutually Exclusive, Collectively Exhaustive)
   Verify if the listed elements accurately reflect the type definition shown in step 1.
   Check if the responsibilities among the elements with the same type are MECE.
   - Identify potential overlaps based on IDs and Titles.
   - For identified candidates, check detailed content to confirm overlap (use `docgraph describe {{ID}}`).
   - Are there gaps in coverage?

   If overlaps are found, list them in pairs or groups like this:
   ```
   ID_A, ID_B
   ID_C, ID_D
   ```

4. Refine structure
   If overlaps are found:
   1. **Explain the overlap**: Clearly state what is overlapping and how (e.g., "ID_A cover X, while ID_B covers X and Y").
   2. **Propose options**: Suggest ways to integrate them (e.g., "Option 1: Merge A into B", "Option 2: Delete A").
   3. **Ask User for decision**: Wait for the user to select the best approach before proceeding with changes.

## Case 2: Input is a NODE_ID

1. Show detailed information
   ```bash
   docgraph describe {{TARGET}}
   ```

2. Analyze Responsibilities (SRP Check)
   Read the description and content of the node carefully.
   - **Too many responsibilities?**: Does this single node try to do too much? (e.g., "Handle login AND process payments")
   - **Vague definition?**: Is the scope defined clearly?

3. Verify Realizability
   Check the constituent elements (outgoing edges/dependencies).
   - **Sufficient components?**: Do the linked child nodes/dependencies sufficiently realize the responsibilities of this node?
   - **Missing dependencies?**: Are there missing links to necessary components?

4. Refine Node
   If issues are found:
   - **Split Node**: If it has too many responsibilities, propose splitting it into multiple smaller nodes.
   - **Add Dependencies**: If realizability is low, propose adding missing dependencies.
