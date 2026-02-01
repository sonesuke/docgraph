<a id="CC_CICD"></a>

# CI/CD Pipeline

The project uses GitHub Actions for Continuous Integration and Deployment.

## Pipeline Structure

### 1. Verification (On Pull Request)

Triggered on every PR targeting `main`.

- **Check & Build**: `cargo build` and `cargo check`.
- **Linting**: `cargo clippy` and `cargo fmt`.
- **Testing**: `cargo test` (Unit/Integration) and `tests/lsp` (E2E).
- **Code Coverage**: `cargo llvm-cov` generates report.
- **Security**: CodeQL analysis (SAST) and `cargo audit`.

### 2. Deployment (On Push to Main)

Triggered when PR is merged to `main`.

- **Publish**: (Planned) Publish binary or crate to registry.
- **Docs**: (Planned) Deploy documentation graph.

**Realization:**

- [ADR_LAYERED_ARCH (Layered Architecture: Core, CLI Handlers, LSP Handlers)](../../decisions/layered-architecture.md#ADR_LAYERED_ARCH)
