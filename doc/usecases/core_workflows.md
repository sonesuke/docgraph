
# Core Use Cases

```{document} Write Specifications
:id: UC-WRITE
:kind: usecase
:depends_on: ACT-USER

The {ref}`ACT-USER` writes specifications in Markdown using MyST format.
```

```{document} Lint Documents
:id: UC-LINT
:kind: usecase
:depends_on: ACT-USER
:depends_on: UC-WRITE

The {ref}`ACT-USER` checks for errors in the document graph.

**Steps:**
1. User navigates to the project directory.
2. User runs the lint command ({ref}`SPEC-CLI-LINT`).
3. User reviews any error messages regarding duplicate IDs or missing references.
```

```{document} Generate Graph
:id: UC-GEN
:kind: usecase
:depends_on: ACT-USER
:depends_on: UC-WRITE

The {ref}`ACT-USER` generates a JSON representation of the document graph.

**Steps:**
1. User navigates to the project directory.
2. User runs the gen command ({ref}`SPEC-CLI-GEN`).
3. User redirects the JSON output to a file or pipe.
```

```{document} Automate Checks
:id: UC-CI-CHECK
:kind: usecase
:depends_on: ACT-CI
:depends_on: UC-WRITE

The {ref}`ACT-CI` automatically verifies the document graph on every push.
```

```{document} Build Knowledge Graph
:id: UC-BUILD-KNOWLEDGE
:kind: usecase
:depends_on: ACT-AGENT
:depends_on: UC-GEN

The {ref}`ACT-AGENT` builds a GraphRAG knowledge base from the generated JSON graph.
```

```{document} Suggest Usage
:id: UC-SUGGEST-USAGE
:kind: usecase
:depends_on: ACT-AGENT
:depends_on: UC-BUILD-KNOWLEDGE

The {ref}`ACT-AGENT` suggests how to use the system based on the constructed knowledge.
```
