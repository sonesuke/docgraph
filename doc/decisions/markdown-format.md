<a id="ADR_MARKDOWN_FORMAT"></a>

# Choice of Plain Markdown and HTML Anchors

## Decision

We will use **Plain Markdown** with **HTML anchor tags** (`<a id="..."></a>`) placed immediately before headings to define document IDs.

## Rationale

- **GitHub Compatibility**: Ensures that standard Markdown renderers (like GitHub's) can display the documentation properly without broken directives.
- **Ease of Use**: Markdown is the industry standard for lightweight documentation, while reST has a higher learning curve.
- **Tool Simplicity**: Parsing standard HTML tags is robust and doesn't require complex MyST-specific libraries for basic ID extraction.

### Trade-offs

- The Docgraph parser must implement HTML tag extraction correctly.
- Strict convention required: All documentation files must use this ID format to be included in the graph.

## Context

We need a structural format for documentation that allows Docgraph to extract unique IDs and build a relationship graph.
Initially, reStructuredText (reST) and complex MyST directives were considered.
