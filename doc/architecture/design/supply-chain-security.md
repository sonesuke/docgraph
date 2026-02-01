<a id="CC_SUPPLY_CHAIN_SECURITY"></a>

# Supply Chain Security

We employ multiple layers of protection to secure our software supply chain and dependencies.

**Derived from:** [ADR_SECURITY (Context)](../../decisions/security.md#ADR_SECURITY)

## 1. Dependency Updates

**Tool**: GitHub Dependabot

**Configuration**: `.github/dependabot.yml`

Automatically scans for outdated or vulnerable packages:

- **Cargo crates**: Weekly updates
- **GitHub Actions**: Weekly updates
- **NPM packages** (VSIX): Weekly updates

## 2. Dependency Vulnerability Scanning

**Tool**: cargo-audit

**Integration**: CI/CD Pipeline (`.github/workflows/ci.yml`)

Blocks merges that introduce dependencies with known security vulnerabilities by checking against the RustSec advisory database.
