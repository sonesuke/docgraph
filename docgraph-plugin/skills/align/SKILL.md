---
name: align
description: Ensure ID naming conventions, consistency, and correct file placement.
---

# Alignment Skill

This skill provides a structured workflow to ensure that elements in the documentation graph are correctly named and placed.

> [!NOTE]
> For detailed usage and available options of any `docgraph` subcommand, always refer to `docgraph --help` or `docgraph <SUBCOMMAND> --help`.

## Workflow Steps

### 1. Scope Analysis
List elements within the target scope to understand the current state. Use `docgraph list`.

### 2. ID and Title Correspondence
Verify if each ID is mnemonic and easily associated with its Title.
- **Good**: `ID: FR-LOGIN, Title: Login Function`
- **Bad**: `ID: FR-005, Title: Login Function` (if unreadable)
- **Bad**: `ID: FR-LOGIN, Title: Export API` (mismatched)

### 3. Prefix Consistency
Verify if elements in the same category share the same prefix (e.g., `FR-`, `NFR-`, `UC-`, `MOD-`, `IF-`).

### 4. File Placement
Verify if the file location matches the ID prefix.
- `FR-LOGIN` should be in `doc/requirements/functional/...`

### 5. Remediation
If issues are found, propose:
- **Rename ID**: Suggest a better ID (verify dependencies with `docgraph describe` first).
- **Move File**: Suggest moving the definition to the correct file.
- **Fix Prefix**: Correct the prefix or the file path.
