---
description: Ensure ID naming conventions, consistency, and correct file placement
---

1. List elements within the target scope

   ```bash
   docgraph list "{{TARGET_QUERY}}"
   # e.g., "FR-*" or path like "doc/requirements/"
   ```

2. Check ID and Title Correspondence Verify if the ID is mnemonic and easily associated with the Title.
   - **Good**: `ID: FR-LOGIN, Title: Login Function`
   - **Bad**: `ID: FR-005, Title: Login Function` (if unreadable)
   - **Bad**: `ID: FR-LOGIN, Title: Export API` (mismatched)

3. Check Prefix Consistency Verify if elements in the same category share the same prefix.
   - Do all requirements start with `FR-` or `NFR-`?
   - Do all use cases start with `UC-`?

4. Check File Placement Verify if the file location matches the ID prefix.
   - **Correct**: `FR-LOGIN` is in `doc/requirements/functional/...`
   - **Incorrect**: `FR-LOGIN` is in `doc/architecture/...` or `doc/usecases/...`
   - **Incorrect**: `UC-LOGIN` is in `doc/requirements/...`

5. Tidy Up If issues are found:
   - **Rename ID**: Suggest a better ID (use `docgraph describe` to verify dependencies first).
   - **Move File**: Suggest moving the definition to the correct file.
   - **Fix Prefix**: Suggest correcting the prefix or the file path.
