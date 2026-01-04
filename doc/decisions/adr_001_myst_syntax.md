
# Architecture Decision Records

```{document} Use MyST Syntax
:id: ADR-001
:kind: decision
:depends_on: DOM-DOC

**Title:** Use MyST Directives for Document Blocks

**Status:** Accepted

**Context:** We need a way to embed structured metadata in Markdown that is compatible with existing tools.

**Decision:** We will use the MyST (Markedly Structured Text) directive syntax ` ```{document} ... ``` ` because it is a standard extension mechanism for Markdown and is extensible.

**Consequences:**
- Users must learn MyST syntax.
- We need a parser that supports MyST directives.
```
