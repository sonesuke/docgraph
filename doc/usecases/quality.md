# Quality Control Use Cases

<a id="UC-LINT"></a>

## Lint Documents

The [ACT-USER (User)](../actors/system_users.md#ACT-USER) checks for errors in the document graph.

**Steps:**

1. User navigates to the project directory.
2. User runs the lint command ([IF-CLI-LINT (Command: `lint`)](../requirements/interfaces/cli_specs.md#IF-CLI-LINT)).
3. User reviews any error messages regarding duplicate IDs or missing references.

Depends on: [ACT-USER (User)](../actors/system_users.md#ACT-USER), [UC-WRITE (Write Specifications)](./authoring.md#UC-WRITE), [FR-UNIQUE (Unique IDs)](../requirements/functional/verification.md#FR-UNIQUE), [FR-VALID-REF (Valid References)](../requirements/functional/verification.md#FR-VALID-REF), [FR-STRICT-NODES (Strict Node Types)](../requirements/functional/verification.md#FR-STRICT-NODES), [FR-RELATION-RULES (Relation Rules)](../requirements/functional/verification.md#FR-RELATION-RULES)

<a id="UC-CI-CHECK"></a>

## Automate Checks

The [ACT-CI (CI System)](../actors/system_users.md#ACT-CI) automatically verifies the document graph on every push.

Depends on: [ACT-CI (CI System)](../actors/system_users.md#ACT-CI), [UC-WRITE (Write Specifications)](./authoring.md#UC-WRITE)
