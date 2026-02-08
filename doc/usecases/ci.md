# CI/CD Use Cases

<a id="UC_CI_VALIDATION"></a>

## Continuous Integration Validation

The CI System automatically validates all documentation changes upon pull request submission.

### Actors

- [ACT_CI (CI System)](../actors/systems.md#ACT_CI)
- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_CLI (Command Line Interface)](../requirements/interfaces/interfaces.md#IF_CLI)

### Requirements

- [FR_CORE_VALID_REF (Valid References)](../requirements/functional/core.md#FR_CORE_VALID_REF) Protecting the
  documentation quality in CI
- [FR_CORE_AUDIT (Audit Logging)](../requirements/functional/core.md#FR_CORE_AUDIT) Capturing security and linting
  trails
- [FR_DEV_CI (Automated Validation)](../requirements/functional/development.md#FR_DEV_CI) Pipeline integration for all
  documentation changes

### Flow

1. CI system triggers on push.
2. CI system runs `docgraph check .` command.
3. CI System reports validation results back to the developer.
