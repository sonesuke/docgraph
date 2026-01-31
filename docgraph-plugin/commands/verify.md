---
description: Verify and refine element responsibilities for a specific type to ensure MECE
---
1. Show type definition
   ```bash
   docgraph type {{TYPE_ID}}
   ```

2. List all elements of this type
   ```bash
   docgraph list "{{TYPE_ID}}*"
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
