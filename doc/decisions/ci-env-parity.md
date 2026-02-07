<a id="ADR_CI_ENV_PARITY"></a>

# CI Environment Parity

## Decision

We will use the **Dev Container** as the single source of truth for the development and testing environment.

- **Local Development**: Developers use the `.devcontainer` configuration.
- **CI Pipeline**: The CI workflow will use the `devcontainers/ci` action to build and run tests *inside* the exact same container image defined in `.devcontainer/devcontainer.json`.

## Rationale

- **Parity**: Guaranteed environment consistency between local dev and CI.
- **Maintainability**: Dependency updates (e.g., Rust version, Node.js version) only need to be configured in one place (`devcontainer.json`).
- **Onboarding**: "It works in the container" implies it works in CI.

### Trade-offs

- **Build Time**: CI jobs may take slightly longer to start as they need to build/pull the Dev Container image.
- **Complexity**: Debugging CI failures might require understanding Dev Container mechanics.

## Context

We discovered discrepancies between the Continuous Integration (CI) environment (Ubuntu-latest) and the local development environment (Dev Container / Debian Bullseye). Specifically, the Node.js version differed (v18 vs latest LTS), creating a risk of "works on my machine, fails in CI" scenarios.
