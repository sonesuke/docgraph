---
name: query
description: Documentation Graph Query & Analysis - Explore, investigate, and assess the graph using semantic tools and Cypher.
---

# Documentation Graph Query & Analysis

This skill provides a unified methodology for exploring the documentation graph, answering specific questions, and
analyzing the impact or traceability of specifications. It leverages `docgraph` specific tools to navigate the semantic
structure of the graph.

> [!TIP] Use this skill for any graph-related investigation: from finding a requirement to analyzing the ripple effect
> of a change or verifying that an intent is fully realized.

## Prerequisites

- **`docgraph` CLI must be installed as a system binary**
  - Install via: `curl -fsSL https://raw.githubusercontent.com/sonesuke/docgraph/main/install.sh | bash`
  - Or build from source: `cargo install --path .`
- **Installation Verification**: Run `docgraph --version` to confirm the binary is available

## Tools & Techniques

### 1. `docgraph query` (Powerful Search)

Uses Cypher-like syntax for precise, attribute-aware, and relational searching. **This is the preferred tool for complex
searches.**

- **Usage**: `docgraph query "<CYPHER_QUERY>"`
- **Examples**:
  - `docgraph query "MATCH (n:FR) RETURN n.id"` - List all functional requirements.
  - `docgraph query "MATCH (n) WHERE n.name CONTAINS 'Auth' RETURN n.id, n.name"` - Search by name.
  - `docgraph query "MATCH (fr:FR)-[]->(uc:UC) WHERE uc.id = 'UC_001' RETURN fr.id"` - Find requirements deriving from a
    use case.

### 2. `docgraph describe` (Exploration)

Deep dive into a single node's content and immediate relations. `docgraph describe <ID>`
- _Benefit_: Use this to see direct inbound/outbound links and the local "Why/How" context.

### 3. `docgraph trace` (Lineage & Realization)

Follow chains of relationships to verify traceability or global impact.

- **Usage**: `docgraph trace <START_ID> --direction <up|down>`
- **Realization Check (Down)**: Verify if an abstract intent (FR) reaches a terminal implementation (MOD, IF).
- **Justification Check (Up)**: Verify the origin/intent of a low-level specification.

---

## Workflow: Impact Analysis

Use this workflow before renaming IDs, moving files, or changing core requirement logic.

1. **Local Impact**: Use `docgraph describe <ID>` to list direct relations.
2. **Global Ripple**: Use `docgraph trace <ID> --direction down` (to see what is broken downstream) and `--direction up`
   (to see what loses its justification upstream).
3. **Cross-Cutting**: Search for related concepts using `docgraph query` or `grep`.
4. **Report**: List affected IDs and categorize impact level (HIGH/MED/LOW).

## Workflow: Traceability Verification

Use this workflow to ensure a node’s responsibility is fully traceable and ultimately realizable.

1. **Downstream Traversal**: Use `trace <ID> --direction down`.
2. **Terminal Reachability**: Ensure at least one path reaches a realizable terminal (MOD, IF, CODE).
3. **Abstraction Monotonicity**: Ensure abstraction decreases (FR > UC > MOD > CODE).
4. **Semantic Continuity**: Verify that the core intent survives through the transitions without "semantic fog" or
   drift.

---

## Query & Analysis Report

Structure your findings to provide clear insights:

### Findings Summary
- **Scope/Target**: [ID or Query]
- **Key Relationships Found**: [e.g., "FR_001 realized by MOD_AUTH"]
- **Impact/Traceability Result**: [PASS/FAIL or Risk Level]

### Detailed Analysis
Provide the detailed answer or assessment built from the gathered context.

### Navigation Trace (optional)
`FR_001` -> `IF_001` -> `MOD_001`
