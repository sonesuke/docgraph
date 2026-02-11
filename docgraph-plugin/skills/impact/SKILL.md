---
name: impact
description:
  Impact Analysis & Ripple Effect Assessment - Evaluate the consequences of adding or changing specifications.
---

# Impact Analysis & Ripple Effect Assessment

This skill defines the process for analyzing the potential consequences of adding, modifying, or removing a
specification node within the documentation graph. It leverages the graph's traceability to identify "ripple effects"
across architectural layers (Requirements, Business, Architecture).

> [!IMPORTANT] Impact analysis is mandatory before renaming IDs, moving files, or changing core requirement logic to
> ensure the integrity of the entire graph.

## Prerequisites

- **`docgraph` CLI must be installed as a system binary**
  - Install via: `curl -fsSL https://raw.githubusercontent.com/sonesuke/docgraph/main/install.sh | bash`
  - Or build from source: `cargo install --path .`
- **This is NOT an npm or Python package** - do NOT use `npx` or `pipx`
- **Installation Verification**: Run `docgraph --version` to confirm the binary is available

## Assessment Tools

### 1. `docgraph describe` (Local Impact)

Identify immediate connections to the node being changed.

- **Goal**: List all direct inbound and outbound references.
- **Workflow**: `docgraph describe <ID>` and inspect the "Relations" section.

### 2. `docgraph trace` (Global Ripple Effect)

Follow the chain of dependencies to the ends of the graph.

- **Downward Trace**: `docgraph trace <ID> --direction down`
  - _Identifies_: What modules (`MOD`), interfaces (`IF`), or sub-requirements are forced to change or re-validate.
- **Upward Trace**: `docgraph trace <ID> --direction up`
  - _Identifies_: What higher-level requirements (`FR`, `NFR`) or architectural decisions (`ADR`) might be affected or
    lose their justification.

### 3. `docgraph check` (Integrity Verification)

Validate that the proposed change doesn't violate graph rules.

- **Goal**: Detect broken links (`DG003`), template mismatches (`DG007`), or invalid reference directions (`DG006`).

---

## Analysis Workflow

### 1. Identify Entry Point

Define the specific node ID and the nature of the change (Add/Modify/Delete).

### 2. Upward Impact (Dependencies/Justification)

Analyze what this node "satisfies" or "derives from".

- If this node is deleted, are the parent nodes still fully justified?
- If this node is modified, does it still meet the parent's intent?

### 3. Downward Impact (Satisfaction/Implementation)

Analyze what "realizes" or "derives from" this node.

- **Technical Impact**: Which Modules (`MOD`) or Interfaces (`IF`) need implementation changes?
- **Specification Impact**: Are there child requirements that become obsolete or conflicting?

### 4. Cross-Cutting Impact

Check for associations with Cross-cutting Concepts (`CC`).

- Does the change impact global system qualities like "Security" or "Performance"?

---

## Impact Assessment Report

Structure your findings to provide a clear risk assessment:

### Change Summary

- **Target Node**: [ID]
- **Action**: [Add / Modify / Delete]

### Affected Nodes

| Layer        | ID        | Impact Level | Description of Impact                                    |
| :----------- | :-------- | :----------- | :------------------------------------------------------- |
| Requirement  | `FR_...`  | HIGH/MED/LOW | e.g., "Parent requirement becomes partially unsatisfied" |
| Architecture | `MOD_...` | HIGH/MED/LOW | e.g., "Requires logic change in billing handler"         |
| Decision     | `ADR_...` | LOW          | e.g., "Remains consistent with ADR_001"                  |

### Verification Status

- **Result**: [CLEAN / ERRORS FOUND]
- **Broken References**: [List of IDs or "None"]

### Remediation Recommendation

Propose steps to mitigate negative impacts (e.g., "Update `MOD_AUTH` to reflect the new interface").
