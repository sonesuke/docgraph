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

- **Dependabot**: Automatically scans for outdated or vulnerable crates and npm packages (including VSIX and Zed extension).
- **Cargo Audit**: Integrated into our CI pipeline to block merges that introduce known security risks in Core and Zed extension.
- **npm audit**: Integrated into our CI pipeline to block merges that introduce known security risks in VSIX.

