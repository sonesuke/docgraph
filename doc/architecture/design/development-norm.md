# Development Norms

This document defines standardized development practices and environments for the project.

<a id="CC_DEV_ENV"></a>

## Development Environment

We provide a standardized development environment using VS Code Dev Containers to ensure consistency across all
contributors and the CI pipeline.

Defined in [.devcontainer/devcontainer.json](../../../.devcontainer/devcontainer.json).

**Coding Standards:**

- **Formatting**: `cargo fmt` (Rust), `npm run format` (VSIX), `prettier` (Markdown/JSON/TOML).
- **Linting**: `cargo clippy` (Rust), `npm run lint` (VSIX).
- **Pre-commit**: `prek` (Automates fmt, clippy, prettier, tests, and docgraph check).
- **Security**: `cargo audit`.

**Performance:**

- **Image Size**: The Dev Container image should be kept as small as possible to ensure fast CI/CD execution and
  minimize local disk usage.
  - Use minimal base images (e.g., `slim` variants).
  - Remove unnecessary build artifacts and caches (e.g., `apt-get clean`).

### Decided by

- [ADR_CI_ENV_PARITY (CI Environment Parity)](../../decisions/ci-env-parity.md#ADR_CI_ENV_PARITY) To ensure consistent
  development experience.

### Realized by

- [MOD_DEV_CONTAINER (Dev Container)](../view/module.md#MOD_DEV_CONTAINER)

---

<a id="CC_CONVENTIONAL_COMMITS"></a>

## Commit Messages

We follow the **Conventional Commits** specification for all commit messages. This enables automated changelog
generation and versioning.

**Format:**

`<type>[optional scope]: <description>`

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools and libraries

### Realized by

- [MOD_CICD (CI/CD Pipelines)](../view/module.md#MOD_CICD)

---

<a id="CC_SEMANTIC_VERSIONING"></a>

## Versioning

The project follows **Semantic Versioning (SemVer)**.

**Rules:**

Given a version number **MAJOR.MINOR.PATCH**, increment the:

1. **MAJOR** version when you make incompatible API changes.
2. **MINOR** version when you add functionality in a backwards compatible manner.
3. **PATCH** version when you make backwards compatible bug fixes.

### Realized by

- [MOD_CICD (CI/CD Pipelines)](../view/module.md#MOD_CICD)
