# Static Application Security Testing (SAST) & Dependency Scanning

<a id="CC_SAST"></a>

## SAST (CodeQL)

We use GitHub CodeQL to automatically analyze the codebase for security vulnerabilities and coding errors.

**Configuration:**

- **Workflow**: `.github/workflows/codeql.yml`
- **Languages**: Rust (analyzed via build), TypeScript (VSIX)

**Execution:**

- Push to main
- Pull Requests targeting main
- Schedule (Weekly)

**Dependency Scanning:**

We use automated tools to block merges that introduce known vulnerabilities in dependencies.

**Tools:**

- **cargo audit**: Checks Rust dependencies (Core and Zed Extension).
- **npm audit**: Checks Node.js dependencies (VSIX).

**Execution:**

- CI Pipeline (`lint` job) on every Pull Request.

### Decided by

- [ADR_SECURITY (Security Architecture)](../../decisions/security.md#ADR_SECURITY) To ensure secure code lifecycle.
