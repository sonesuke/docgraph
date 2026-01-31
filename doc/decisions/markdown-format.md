
# Architecture Decision Records

<a id="ADR-MARKDOWN-FORMAT"></a>

## Choice of Plain Markdown and HTML Anchors

### Status

Accepted

### Context

We need a structural format for documentation that allows Docgraph to extract unique IDs and build a relationship graph.
Initially, reStructuredText (reST) and complex MyST directives were considered.

### Decision

We will use **Plain Markdown** with **HTML anchor tags** (`<a id="EXAMPLE-ID-2"></a>`) placed immediately before headings to define document IDs.

### Rationale

- **GitHub Compatibility**: Ensures that standard Markdown renderers (like GitHub's) can display the documentation properly without broken directives.
- **Ease of Use**: Markdown is the industry standard for lightweight documentation, while reST has a higher learning curve.
- **Tool Simplicity**: Parsing standard HTML tags is robust and doesn't require complex MyST-specific libraries for basic ID extraction.

### Consequences

- The Docgraph parser must handle HTML tag extraction properly.
- All documentation files must follow this convention to be included in the graph.
