---
name: ask
description: Documentation Graph Query & Exploration - Find, investigate, and answer questions about the graph.
---

# Documentation Graph Query & Exploration

This skill provides a structured methodology for exploring the documentation graph and answering specific questions
about its contents, relationships, and structure. It emphasizes using `docgraph` specific tools over general text search
(grep) to leverage the semantic structure of the graph.

> [!TIP] Use this skill whenever you need to understand "how things are connected", "where a requirement is defined", or
> "what is the impact of a change".

## Exploration Tools

### 1. `docgraph list` (Discovery)

Use to find nodes matching a pattern or within a scope.

- **Usage**: `docgraph list "<ID_PATTERN>"`
- **Example**: `docgraph list "FR_LOGIN_*"`
- **Benefit**: Quickly identifies relevant IDs without wading through raw file content.

### 2. `docgraph type` (Structural Context)

Use to understand the expected schema and constraints of a specific node type.

- **Usage**: `docgraph type <TYPE>`
- **Example**: `docgraph type FR`
- **Benefit**: Reveals what sections and dependencies are mandatory or optional.

### 3. `docgraph describe` (Deep Dive)

Use to retrieve the full content and immediate relations of a specific node.

- **Usage**: `docgraph describe <ID>`
- **Example**: `docgraph describe FR_LOGIN_001`
- **Benefit**: Provides a unified view of the node's text and its incoming/outgoing edges.

### 4. `docgraph trace` (Impact & Lineage)

Use to follow a chain of relationships (e.g., from requirement to code, or dependency tree).

- **Usage**: `docgraph trace <START_ID> --direction <up|down>`
- **Example**: `docgraph trace FR_LOGIN_001 --direction down`
- **Benefit**: Visualizes the full path of derivation or satisfaction.

### 6. Semantic Inference & Terminology Deduction (Cognitive Tool)

Don't just look for matches; analyze how terms are used.

- **Contextual Deduction**: If a term is used in multiple `FR` nodes related to "Auth", it likely refers to an
  authentication concept.
- **ID Mnemonics**: Use the prefix and mnemonic structure (e.g., `API-BILL-*`) to infer the scope of a term.
- **Relational Meaning**: A node that "realizes" an `IF` (Interface) is likely an implementation detail (Module).

### 5. `grep` (Keyword Fallback)

Use only when you don't have a specific ID or node type to start with.

- **Usage**: `grep -r "keyword" doc/`
- **Benefit**: Finds mentions of terms that aren't formal IDs.

---

## Workflow Steps

### 1. Intent Clarification

Identify the core of the question:

- **Structural**: "What is required for a Functional Requirement?" -> Use `type`.
- **Existence**: "Do we have a requirement for SSO?" -> Use `list` or `grep`.
- **Relationship**: "What modules implement the billing interface?" -> Use `describe` or `trace`.

### 2. Multi-Layer Exploration

Don't stop at the first finding. Follow the links:

1. **Find** the starting node (`list`).
2. **Understand** the starting node (`describe`).
3. **Trace** the connections (`trace`) to see how it fits into the broader system.

### 3. Contextual Synthesis & Inference

Combine findings from the graph logic with the actual Markdown content.

- Use `docgraph describe` to see the semantic links.
- Use `view_file` to read the narrative context.
- **Inference**: If a term is mentioned in the text but has no ID, search for related concepts using `list` or `grep` to
  see if it's an alias or a broader system component.

## Query Resolution Report

When answering, structure your response as follows:

### Findings Summary

- **Primary Node(s)**: [ID(s) found]
- **Key Relationships**: [e.g., "Satisfies ADR_001", "Realized by MOD_AUTH"]

### Detailed Analysis

Provide the detailed answer built from the gathered context.

### Navigation Trace (optional)

If relevant, show the path taken: `FR_001` -> `IF_001` -> `MOD_001`
