# Quality Control Use Cases

<a id="UC_LINT"></a>

## Lint Documents

The [ACT_USER (User)](../actors/users.md#ACT_USER) checks for errors in the document graph.

**Steps:**

1. User navigates to the project directory.
2. User runs the lint command ([IF_CLI_LINT (Command: `lint`)](../requirements/interfaces/cli-specs.md#IF_CLI_LINT)).
3. User reviews any error messages regarding duplicate IDs or missing references.

Depends on: [ACT_USER (User)](../actors/users.md#ACT_USER), [UC_WRITE (Write Specifications)](./authoring.md#UC_WRITE), [FR_UNIQUE (Unique IDs)](../requirements/functional/verification.md#FR_UNIQUE), [FR_VALID_REF (Valid References)](../requirements/functional/verification.md#FR_VALID_REF), [FR_STRICT_NODES (Strict Node Types)](../requirements/functional/verification.md#FR_STRICT_NODES), [FR_RELATION_RULES (Relation Rules)](../requirements/functional/verification.md#FR_RELATION_RULES)

<a id="UC_CI_CHECK"></a>

## Automate Checks

The [ACT_CI (CI System)](../actors/systems.md#ACT_CI) automatically verifies the document graph on every push.

Depends on: [ACT_CI (CI System)](../actors/systems.md#ACT_CI), [UC_WRITE (Write Specifications)](./authoring.md#UC_WRITE), [NFR_CI_AUTO (Automated Validation)](../requirements/non-functional/automation.md#NFR_CI_AUTO)
