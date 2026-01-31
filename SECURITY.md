# Security Policy

## Supported Versions

We only support the latest version of docgraph. We recommend always keeping your installation up to date to ensure you have the latest security patches.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

If you find a security vulnerability, please report it privately to the project maintainers. Do not open a public issue.

You can report vulnerabilities by:

1. Opening a draft security advisory on GitHub.
2. Contacting the maintainers directly via their preferred contact method (if available).

We aim to acknowledge receipt within 48 hours and provide a resolution plan shortly after.

## Commitment to Dependency Security

We use automated tools to monitor our dependencies for known vulnerabilities:

- **Dependabot**: Automatically scans for outdated or vulnerable crates.
- **Cargo Audit**: Integrated into our CI pipeline to block merges that introduce known security risks.

### Known Issues
As of January 2026, we are aware of vulnerabilities in `serde_yml` (a transitive dependency). We are actively monitoring the upstream linter for a fix and planning a migration if necessary.
