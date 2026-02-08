---
name: validate
description: Validation Quality Gate - Ensure project integrity and consistency.
---

# Validation Quality Gate

This skill serves as the **definitive quality threshold** for the documentation graph. It must be executed strictly to ensure that all semantic, structural, and consistency rules are met before finalizing changes.

> [!IMPORTANT]
> This skill must perform checks **strictly and rigorously**. Accuracy in ID naming, responsibility scope, and template adherence is critical for the integrity of the documentation graph. Do not overlook minor inconsistencies.

> [!NOTE]
> For detailed usage and available options of any `docgraph` subcommand, always refer to `docgraph --help` or `docgraph <SUBCOMMAND> --help`.

## Workflow Steps

### 0. Automated Check Pre-requisites
Before performing manual semantic checks, ensure all automated validations pass.
1. **Formatting**: Run `npm run format:md -- --check` (or similar) to verify Markdown style.
2. **Consistency**: Run `docgraph check` to ensure the internal graph logic is sound.
3. **Rust Integrity** (if applicable): Run `cargo test` and `cargo clippy`.

If any of these fail, stop here and fix the issues first.

### 1. Scope Analysis
List elements within the target scope to understand the current state. Use `docgraph list`.

### 2. ID and Title Correspondence
Verify if each ID is mnemonic and easily associated with its Title.
- **Good**: `ID: FR_LOGIN, Title: Login Function`
- **Bad**: `ID: FR_005, Title: Login Function` (if unreadable)
- **Bad**: `ID: FR_LOGIN, Title: Export API` (mismatched)

### 3. Prefix Consistency
Verify if elements in the same category share the same prefix (e.g., `FR_`, `NFR_`, `UC_`, `MOD_`, `IF_`).

### 4. File Placement and Categorization
Verify if the file location is appropriate for the ID prefix and consistent with similar nodes.
1. **Consistency Check**: Use `docgraph list "<PREFIX>_*"` (e.g., `docgraph list "FR_*"`) to see where other nodes of the same type are located. 
2. **Rule of Thumb**: Generally, nodes with the same prefix should be grouped in the same file or a specific directory structure.
3. **Validation**: 
   - `FR_LOGIN` should be in `doc/requirements/functional/...`
   - If most `FR_` nodes are in `requirements.md`, this node should likely be there too, or in a logically related file.

### 5. Template and Structure Validation
Verify if the node's content follows the defined template and structure rules.
1. **Retrieve Template**: Use `docgraph type <TYPE>` (e.g., `docgraph type FR`) to see the required structure and template file path.
2. **Retrieve Content**: Use `docgraph describe <ID>` (e.g., `docgraph describe FR_LOGIN`) to see the full content of the node.
3. **Compare**:
   - Are all required sections (headers) present?
   - Do list items (dependencies) match the expected patterns?
   - Is the overall content consistent with the type's purpose?

### 6. Single Responsibility Principle (SRP) Check
Verify if the node represents a single responsibility at the appropriate granularity for its type.
- **Goal**: One ID should correspond to one clear concept or requirement.
- **Check**: Does this node try to address multiple unrelated issues? 
- **Check**: Is the description or title too broad (e.g., "System Management" instead of "User Authentication")?

### 7. Remediation
If issues are found, propose:
- **Rename ID**: Suggest a better ID (verify dependencies with `docgraph describe` first).
- **Move File**: Suggest moving the definition to the correct file.
- **Fix Prefix**: Correct the prefix or the file path.
- **Fix Structure**: Propose adding missing sections or correcting the link format to match the template.
- **Split Node**: If SRP is violated, propose splitting the node into multiple IDs with narrower scopes.

## Quality Gate Checklist
In your final report, you **must** include this checklist to demonstrate thorough verification:

- [ ] **Automated Checks**: `npm run format:md`, `docgraph check`, and `cargo test/clippy` (if applicable) all pass.
- [ ] **ID Naming**: ID is underscore-separated (e.g., `TYPE_DESC`), mnemonic, and matches the prefix rules.
- [ ] **File Placement**: File location is appropriate for the node type and consistent with similar nodes.
- [ ] **Template Adherence**: All required sections and formats defined in `docgraph type` are present.
- [ ] **SRP Compliance**: The node addresses only one clear responsibility at the correct granularity.

## Final Decision
- **PASS**: All items in the checklist are confirmed.
- **FAIL**: One or more items in the checklist failed. Provide remediation steps.
