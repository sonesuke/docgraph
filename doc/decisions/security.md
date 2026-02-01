# Security Architecture

<a id="ADR_SECURITY"></a>

## Context

Security is critical for a developer tool that processes user code. We need multiple layers of protection.

## Decision

We implement a comprehensive security strategy:

1. **SAST**: GitHub CodeQL for static analysis
2. **Supply Chain**: Dependabot for automated updates
3. **Vulnerability Scanning**: cargo-audit in CI/CD

## Consequences

- CodeQL runs on push to main, PRs, and weekly schedule
- Dependencies are updated weekly
- Merges with known vulnerabilities are blocked
