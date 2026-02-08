# Release Process

This project follows a "Release PR" workflow to manage revisions and changelogs. Direct pushes of versions or changelogs
to the `main` branch are blocked by protection rules.

## 1. Preparation Phase (Pull Request)

1. **Create a Branch**: Start a new branch from `main` (e.g., `chore/release-v0.2.1`).

2. **Generate Changelog**: Run `git cliff` locally to update `CHANGELOG.md`.

   ```bash
   git cliff -o CHANGELOG.md
   ```

   Review the generated entries and manually adjust if necessary.

3. **Bump Version**: Update the version number in the following files:
   - `Cargo.toml`
   - `vsix/package.json`
   - Other relevant manifests

4. **Create & Merge PR**:
   - Commit the changes (`chore(release): prepare v0.2.1`).
   - Create a Pull Request to `main`.
   - Get approval and merge it.

## 2. Execution Phase (Tagging)

After the Release PR is merged into `main`:

1. **Pull Latest Main**:

   ```bash
   git checkout main
   git pull origin main
   ```

2. **Create Tag**: Create a tag for the release version.

   ```bash
   git tag v0.2.1
   ```

3. **Push Tag**: Push the tag to GitHub.

   ```bash
   git push origin v0.2.1
   ```

4. **Wait for CI**: The [Release Workflow](../../.github/workflows/release.yml) will trigger automatically on the pushed
   tag. It will:
   - Build artifacts for Linux/macOS/Windows.
   - Build the VSIX package.
   - Create a GitHub Release with the generated changelog (Release Notes) and attached assets.

> [!NOTE] The workflow will **not** commit `CHANGELOG.md` to the repository. It only populates the release body on
> GitHub.
