
# CLI Specifications

<a id="SPEC-CLI-LINT"></a>

## Command: `lint`

The `lint` command shall parse all Markdown files in the target directory, build the graph, and report any violations of validation rules.

Verifies: [UC-LINT (Lint Documents)](../usecases/core_workflows.md#UC-LINT)
Depends on: [RULE-UNIQUE (Unique IDs)](../rules/validation_rules.md#RULE-UNIQUE), [RULE-VALID-REF (Valid References)](../rules/validation_rules.md#RULE-VALID-REF)

<a id="SPEC-CLI-GRAPH"></a>

### Command: `graph`

The `graph` command shall output the graph structure in JSON format.

Verifies: [UC-GRAPH (Generate Graph)](../usecases/core_workflows.md#UC-GRAPH)
Depends on: [DOM-GRAPH (Graph)](../model/domain_model.md#DOM-GRAPH)
