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
   - Are there overlaps in responsibilities?
   - Are there gaps in coverage?

   If overlaps are found, list them in pairs or groups like this:
   ```
   ID_A, ID_B
   ID_C, ID_D
   ```

4. Refine structure
   If overlaps are found, propose merging or splitting elements to achieve MECE.
