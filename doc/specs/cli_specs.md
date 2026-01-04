
# CLI Specifications

```{document} CLI Lint Command
:id: SPEC-CLI-LINT
:kind: spec
:verifies: UC-LINT
:depends_on: RULE-UNIQUE
:depends_on: RULE-VALID-REF

The `lint` command shall parse all Markdown files in the target directory, build the graph, and report any violations of validation rules.
```

```{document} CLI Gen Command
:id: SPEC-CLI-GEN
:kind: spec
:verifies: UC-GEN
:depends_on: DOM-GRAPH

The `gen` command shall output the graph structure in JSON format.
```
