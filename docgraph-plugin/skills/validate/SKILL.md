---
name: validate
description: Validation Quality Gate - Ensure project integrity and consistency.
---

# Validation Quality Gate

This skill serves as the **definitive quality threshold** for the documentation graph. It must be executed strictly to
ensure that all semantic, structural, and consistency rules are met before finalizing changes.

> [!IMPORTANT] This skill must perform checks **strictly and rigorously**. Accuracy in ID naming, responsibility scope,
> and template adherence is critical for the integrity of the documentation graph. **Even cosmetic inconsistencies must
> be flagged.**

> [!NOTE] For detailed usage and available options of any `docgraph` subcommand, always refer to `docgraph --help` or
> `docgraph <SUBCOMMAND> --help`.

## Prerequisites

- **`docgraph` CLI must be installed as a system binary**
  - Install via: `curl -fsSL https://raw.githubusercontent.com/sonesuke/docgraph/main/install.sh | bash`
  - Or build from source: `cargo install --path .`
- **This is NOT an npm or Python package** - do NOT use `npx` or `pipx`
- **Installation Verification**: Run `docgraph --version` to confirm the binary is available

## Workflow Steps

### 0. Automated Check Pre-requisites

- **Level**: STRICT
- **Failure Policy**: FAIL immediately if any check fails.

Before performing manual semantic checks, ensure all automated validations pass.

1. **Formatting**: Identify the appropriate Markdown formatter for the project (e.g., Prettier, Biome) and verify
   Markdown style (e.g., `npm run format:md -- --check`).
2. **Consistency**: Run `docgraph check` to ensure the internal graph logic is sound.
3. **Rust Integrity**: Run `cargo test` and `cargo clippy` if logic changes are involved.

### 1. Scope Analysis

- **Level**: PREPARATION
- **Purpose**:
  - Identify peer nodes for comparison (naming, placement, structure).
  - Establish the dominant convention (baseline) for the specific node type.

List elements within the target scope to understand the current state. Use `docgraph list`.

### 2. ID and Title Correspondence

- **Level**: STRICT
- **Failure Policy**: FAIL immediately.

Verify if the ID mnemonic matches the Title content.

- `FR_LOGIN` -> "User Login" (Correct)
- `FR_AUTH` -> "User Login" (Incorrect - too broad or mismatched)

### 3. Prefix Consistency

- **Level**: STRICT
- **Failure Policy**: FAIL immediately.

Verify if elements in the same category share the same prefix:

- `FR_` (Functional Requirements)
- `NFR_` (Non-Functional Requirements)
- `UC_` (Use Cases)
- `MOD_` (Modules)
- `IF_` (Interfaces)
- `CC_` (Cross-cutting Concepts)
- `ADR_` (Architecture Decision Records)
- `BB_` (Building Blocks)

### 4. File Placement and Categorization

- **Level**: STRICT
- **Failure Policy**: FAIL immediately.
- **Rule of Thumb**: Generally, nodes with the same prefix should be grouped in the same file or a specific directory
  structure. **This is not an exception mechanism.**

Verify if the file location is appropriate for the ID prefix and consistent with baseline nodes identified in Step 1.

- `FR_LOGIN` should be in `doc/requirements/functional/...`

### 5. Template and Structure Validation

- **Level**: STRICT
- **Failure Policy**: FAIL immediately.

Verify if the node's content follows the defined template and structure rules.

1. **Retrieve Template**: Use `docgraph type <TYPE>`.
2. **Retrieve Content**: Use `docgraph describe <ID>`.
3. **Compare**:
   - Are all required sections (headers) present?
   - Do list items (dependencies) match the expected patterns?

### 6. Single Responsibility Principle (SRP) Check

- **Level**: HEURISTIC (Judgment required)
- **Failure Policy**: FAIL with remediation proposal if violated.

Verify if the node represents a single responsibility at the appropriate granularity for its type.

- **Goal**: One ID should correspond to one clear concept or requirement.
- **Check**: Does this node try to address multiple unrelated issues?

### 7. Remediation Safety Rules

- **Level**: MANDATORY

Any change to ID or file location **MUST** follow these safety rules:

1. **Reference Check**: Run `docgraph describe <ID>` and enumerate all inbound/outbound relations.
2. **Impact Assessment**: Verify that proposed changes do not break existing references unless explicitly handled.

### 8. Remediation Proposals

If issues are found, propose:

- **Rename ID**: Suggest a better ID (after Safety Rules).
- **Move File**: Suggest moving the definition (after Safety Rules).
- **Fix Structure**: Propose adding missing sections or correcting links.
- **Split Node**: Propose splitting the node into multiple IDs with narrower scopes.

## Validation Report

You **must** provide the evaluation results in the following format:

### Target

- **Scope**: [e.g., FR_LOGIN]
- **Baseline Category**: [e.g., FR_ nodes in doc/requirements/functional/]

### Findings

| Category  | Evaluated Item        | Result    | Notes/Details |
| :-------- | :-------------------- | :-------- | :------------ |
| Automated | format / check        | PASS/FAIL |               |
| ID Naming | Mnemonic consistency  | PASS/FAIL |               |
| Structure | Template adherence    | PASS/FAIL |               |
| SRP       | Single responsibility | PASS/FAIL |               |

### Quality Gate Checklist

- [ ] **Automated Checks**: Markdown format, `docgraph check`, etc.
- [ ] **ID Naming**: ID is underscore-separated and mnemonic.
- [ ] **File Placement**: Consistent with peer baseline.
- [ ] **Template Adherence**: All sections from `docgraph type` present.
- [ ] **SRP Compliance**: Single responsibility at node level.

## Final Decision

### Decision Semantics

- **PASS**: Node meets all strict criteria and may be merged/applied.
- **FAIL**: Node MUST NOT be merged. Remediation MUST be completed and re-validated.

**FINAL DECISION: [PASS/FAIL]**
