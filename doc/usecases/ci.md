<a id="UC_CI_VALIDATION"></a>

## CI Validation

Automated validation of documentation and code on every push.

### Actors

- [ACT_CI (CI System)](../actors/systems.md#ACT_CI)
- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_GITHUB_RELEASES (GitHub Releases Interface)](../requirements/interfaces/interfaces.md#IF_GITHUB_RELEASES)

### Flow

1. Developer pushes code to GitHub.
2. GitHub Actions triggers the CI workflow.
3. Docgraph validation runs.
4. If validation fails, the build fails.
