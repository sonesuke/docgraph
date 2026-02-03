# Functional Requirements: Development

<a id="FR_DEV_CI"></a>

## Automated Validation

The system SHALL provide an automated validation pipeline to ensure document graph integrity on every change.

**Realized by**:

- [MOD_CICD (CI/CD Modules)](../../architecture/view/module.md#MOD_CICD)
- [CC_CICD (CI/CD Pipeline)](../../architecture/design/cicd.md#CC_CICD)
- [CC_DEV_ENV (Development Environment)](../../architecture/design/development-norm.md#CC_DEV_ENV)

<a id="FR_DEV_STANDARDS"></a>

## Development Standards

The project SHALL adhere to standardized development practices to ensure maintainability and automated release management.

- **Commit Messages**: MUST follow the Conventional Commits specification.
- **Versioning**: MUST follow Semantic Versioning (SemVer).

**Realized by**:

- [CC_CONVENTIONAL_COMMITS (Commit Messages)](../../architecture/design/development-norm.md#CC_CONVENTIONAL_COMMITS)
- [CC_SEMANTIC_VERSIONING (Versioning)](../../architecture/design/development-norm.md#CC_SEMANTIC_VERSIONING)
- [MOD_CICD (CI/CD Modules)](../../architecture/view/module.md#MOD_CICD)
