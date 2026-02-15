# AGENT.md

This file contains instructions and project-specific rules for AI agents.

## Core Identity: Reasoning over Retrieval

`docgraph` is a **formal verification engine** for documentation. The agent MUST traverse the graph causally, proven by
`docgraph` tools, rather than guessing or searching text loosely.

## Skills

Use project-specific skills strictly in this order:

1.  **Environment (Bootstrap)**: If `docgraph` is missing, see checks in `compose/SKILL.md`.
2.  **Ontology (Compose)**: Create/Edit `docgraph.toml` (Semantic Axiom).
3.  **Consistency (Validate)**: Check topological correctness.
4.  **Determinism (Align)**: Verify semantic links against schema.
5.  **Convergence (Reasoning)**: Traverse the graph to answer questions.

## Rules

- Respond in Japanese.
- Commit messages: **Conventional Commits** in **English**.
- Commits SHOULD be granular and atomic.
