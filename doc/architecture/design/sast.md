<a id="CC_SAST"></a>

# Static Application Security Testing

We use GitHub CodeQL to automatically analyze the codebase for security vulnerabilities and coding errors.

**Derived from:** [ADR_SECURITY (Context)](../../decisions/security.md#ADR_SECURITY)

**Configuration:**

- **Workflow**: `.github/workflows/codeql.yml`
- **Languages**: Rust (analyzed via build)

**Execution:**

CodeQL runs on:

- Push to main
- Pull Requests targeting main
- Schedule (Weekly)
