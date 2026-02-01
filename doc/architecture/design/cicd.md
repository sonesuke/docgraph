<a id="CC_CICD"></a>

# CI/CD Pipeline

The project uses GitHub Actions for Continuous Integration and Deployment.

## Pipeline Structure

### 1. Verification (On Pull Request)

Triggered on every PR targeting `main`. Defined in [.github/workflows/ci.yml](../../../.github/workflows/ci.yml).

- **Lint & Security**:
  - `cargo fmt` and `cargo clippy`.
  - `cargo audit` (Dependency Security).
  - Biome Lint for VSIX (`npm run lint`).
- **Test & Coverage**:
  - `cargo llvm-cov` (Unit/Integration coverage).
  - `docgraph check ./doc` (Integration test).
- **CodeQL**: SAST analysis (via [.github/workflows/codeql.yml](../../../.github/workflows/codeql.yml)).

### 2. Deployment (On Push to Main)

Triggered when PR is merged to `main`.

- **Publish**: (Planned) Publish binary or crate to registry.
- **Docs**: (Planned) Deploy documentation graph.

**Realization:**

- [ADR_LAYERED_ARCH (Layered Architecture: Core, CLI Handlers, LSP Handlers)](../../decisions/layered-architecture.md#ADR_LAYERED_ARCH)
