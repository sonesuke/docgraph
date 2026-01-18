
# CLI Specifications

<a id="IF-CLI-LINT"></a>

## Command: `lint`

The `lint` command shall parse all Markdown files in the target directory, build the graph, and report any violations of validation rules.

Verifies: [UC-LINT (Lint Documents)](../usecases/core_workflows.md#UC-LINT)
Depends on: [FR-UNIQUE (Unique IDs)](../requirements/verification.md#FR-UNIQUE), [FR-VALID-REF (Valid References)](../requirements/verification.md#FR-VALID-REF)

<a id="IF-CLI-GRAPH"></a>

### Command: `graph`

The `graph` command shall output the graph structure in JSON format.

Verifies: [UC-GRAPH (Generate Graph)](../usecases/core_workflows.md#UC-GRAPH)
Depends on: [DAT-GRAPH (Graph)](../model/domain_model.md#DAT-GRAPH)
