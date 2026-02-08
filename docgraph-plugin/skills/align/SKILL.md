---
name: align
description: Ensure ID naming conventions, consistency, and correct file placement.
---

# Alignment Skill

This skill provides a structured workflow to ensure that elements in the documentation graph are correctly named and placed.

## Workflow Steps

### 1. Scope Analysis
List elements within the target scope to understand the current state.
```bash
docgraph list "{{TARGET_QUERY}}"
# e.g., "FR-*" or path like "doc/requirements/"
```

### 2. ID and Title Correspondence
Verify if each ID is mnemonic and easily associated with its Title.
- **Good**: `ID: FR-LOGIN, Title: Login Function`
- **Bad**: `ID: FR-005, Title: Login Function` (if unreadable)
- **Bad**: `ID: FR-LOGIN, Title: Export API` (mismatched)

### 3. Prefix Consistency
Verify if elements in the same category share the same prefix.
- Requirements should start with `FR-` or `NFR-`.
- Use cases should start with `UC-`.
- Architecture components should follow their respective prefixes (e.g., `MOD-`, `IF-`).

### 4. File Placement
Verify if the file location matches the ID prefix.
- **Correct**: `FR-LOGIN` is in `doc/requirements/functional/...`
- **Incorrect**: `FR-LOGIN` is in `doc/architecture/...` or `doc/usecases/...`

### 5. Remediation
If issues are found:
- **Rename ID**: Suggest a better ID (use `docgraph describe` to verify dependencies first).
- **Move File**: Suggest moving the definition to the correct file.
- **Fix Prefix**: Suggest correcting the prefix or the file path.
