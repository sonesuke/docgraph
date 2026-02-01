# AI-Assisted Use Cases

<a id="UC_AI_ASSISTANCE"></a>

## AI-Assisted Documentation

The [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT) leverages the Docgraph plugin to provide intelligent assistance for documentation development, including knowledge base construction, usage suggestions, automated fixes, rule explanations, and workflow guidance.

**Capabilities:**

- **Knowledge Graph**: Builds a GraphRAG knowledge base from the generated JSON graph.
- **Usage Suggestions**: Suggests how to use the system based on the constructed knowledge.
- **Automated Fixes**: Automatically fixes documentation errors (formatting issues, lint violations).
- **Rule Explanation**: Explains validation rules and architectural constraints to ensure documentation quality.
- **Workflow Guidance**: Assists in document-driven development by proposing missing requirements or next architectural steps.

**Derives:**

- [IF_CLAUDE_CODE (Interface: Claude Code Plugin)](../requirements/interfaces/interfaces.md#IF_CLAUDE_CODE)
- [IF_CLAUDE_MARKETPLACE (Claude Marketplace)](../requirements/interfaces/interfaces.md#IF_CLAUDE_MARKETPLACE)
- [FR_CLAUDE_RAG (Retrieval-Augmented Generation)](../requirements/functional/claude.md#FR_CLAUDE_RAG)
- [FR_CLAUDE_SUGGEST (Usage Suggestions)](../requirements/functional/claude.md#FR_CLAUDE_SUGGEST)
- [FR_CLAUDE_FIX (Automated Fixes)](../requirements/functional/claude.md#FR_CLAUDE_FIX)
- [FR_CLAUDE_EXPLAIN (Rule Explanation)](../requirements/functional/claude.md#FR_CLAUDE_EXPLAIN)
- [FR_CLAUDE_WORKFLOW (Workflow Assistance)](../requirements/functional/claude.md#FR_CLAUDE_WORKFLOW)
- [FR_CLAUDE_MARKETPLACE (Claude Marketplace Support)](../requirements/functional/claude.md#FR_CLAUDE_MARKETPLACE)
- [FR_CLAUDE_INSTALL (Claude Plugin Installation)](../requirements/functional/claude.md#FR_CLAUDE_INSTALL)
- [FR_CLI_TYPE (Type Command)](../requirements/functional/cli.md#FR_CLI_TYPE)
