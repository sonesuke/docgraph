---
name: validate
description: Ensure ID naming conventions, consistency, and correct file placement.
---

# Validation Skill

This skill provides a structured workflow to ensure that elements in the documentation graph are correctly named and placed.

> [!NOTE]
> For detailed usage and available options of any `docgraph` subcommand, always refer to `docgraph --help` or `docgraph <SUBCOMMAND> --help`.

## Workflow Steps

### 1. Scope Analysis
List elements within the target scope to understand the current state. Use `docgraph list`.

### 2. ID and Title Correspondence
Verify if each ID is mnemonic and easily associated with its Title.
- **Good**: `ID: FR_LOGIN, Title: Login Function`
- **Bad**: `ID: FR_005, Title: Login Function` (if unreadable)
- **Bad**: `ID: FR_LOGIN, Title: Export API` (mismatched)

### 3. Prefix Consistency
Verify if elements in the same category share the same prefix (e.g., `FR_`, `NFR_`, `UC_`, `MOD_`, `IF_`).

### 4. File Placement
Verify if the file location matches the ID prefix.
- `FR_LOGIN` should be in `doc/requirements/functional/...`

### 5. Template and Structure Validation (NEW)
Verify if the node's content follows the defined template and structure rules.
1. **Retrieve Template**: Use `docgraph type <TYPE>` (e.g., `docgraph type FR`) to see the required structure and template file path.
2. **Retrieve Content**: Use `docgraph describe <ID>` (e.g., `docgraph describe FR_LOGIN`) to see the full content of the node.
3. **Compare**:
   - Are all required sections (headers) present?
   - Do list items (dependencies) match the expected patterns?
   - Is the overall content consistent with the type's purpose?

### 6. Remediation
If issues are found, propose:
- **Rename ID**: Suggest a better ID (verify dependencies with `docgraph describe` first).
- **Move File**: Suggest moving the definition to the correct file.
- **Fix Prefix**: Correct the prefix or the file path.
- **Fix Structure**: Propose adding missing sections or correcting the link format to match the template.
