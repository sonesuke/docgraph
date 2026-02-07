# Supply Chain Security

<a id="CC_SUPPLY_CHAIN_SECURITY"></a>

## Supply Chain Security

We enforce strict controls on dependencies and build processes to prevent supply chain attacks.

**Measures:**

- **Dependency Pinning**: All dependencies must be pinned to specific versions (Lockfiles).
- **Vulnerability Scanning**: All dependencies are scanned for known vulnerabilities.
- **Minimal Dependencies**: We avoid adding dependencies unless absolutely necessary.

**1. Dependency Updates**

**Tool**: GitHub Dependabot
**Configuration**: `.github/dependabot.yml`

Automatically scans for outdated or vulnerable packages:
- **Cargo crates**: Weekly updates
- **GitHub Actions**: Weekly updates
- **NPM packages** (VSIX): Weekly updates

**2. Dependency Vulnerability Scanning**

**Tool**: cargo-audit
**Integration**: CI/CD Pipeline (`.github/workflows/ci.yml`)

Blocks merges that introduce dependencies with known security vulnerabilities by checking against the RustSec advisory database.

### Decided by

- [ADR_SECURITY (Security Architecture)](../../decisions/security.md#ADR_SECURITY) To prevent supply chain attacks.
