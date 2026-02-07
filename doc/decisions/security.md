<a id="ADR_SECURITY"></a>

# Security Architecture

## Decision

We implement a comprehensive security strategy:

1. **SAST**: GitHub CodeQL for static analysis
2. **Supply Chain**: Dependabot for automated updates
3. **Vulnerability Scanning**:
    - **Core & Zed Extension**: `cargo-audit` in CI/CD
    - **VSIX**: `npm audit` in CI/CD

## Rationale

- CodeQL runs on push to main, PRs, and weekly schedule
- Dependencies are updated weekly
- Merges with known vulnerabilities in ANY component (Rust or Node.js) are blocked

## Context

Security is critical for a developer tool that processes user code. We need multiple layers of protection.
