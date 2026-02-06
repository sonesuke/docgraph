# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2026-02-05

### Bug Fixes

- Optimize devcontainer build config (#65)
- Check for code command in post-create.sh to fix CI failure
- Use more robust check for code command in post-create.sh
- Pin rumdl to 0.1.10 and fix compilation error
- Install tools as root in Dockerfile and simplify devcontainer.json
- Correct relative path for MOD_CICD link and fix Dockerfile permissions
- Use sudo for cargo install in post-create.sh
- Dummy bench file and doc lints
- Move CARGO_TARGET_DIR to user home to fix permission denied in CI
- Move CARGO_HOME to user home to resolve registry permission errors
- *(ci)* Add packages:read permission to CI workflow for GHCR cache access
- *(ci)* Add explicit ghcr login and use full image URL for cache
- *(ci)* Pre-build debug dependency artifacts in Dockerfile to speed up CI
- *(zed)* Update zed_extension_api to 0.7.0 and bind to Markdown language

### Miscellaneous Tasks

- *(release)* Allow release to continue if changelog push fails (#63)
- *(release)* Remove zed extension from release and ci (#64)
- Refine release workflow and documentation (#66)
- Set default user to vscode in Dockerfile
- Remove zed-extension/target from git and add .gitignore
- Cleanup zed-extension artifacts and update .gitignore
- *(release)* Automate CHANGELOG.md generation and commitment
- *(release)* Bump version to 0.2.1 (#62)

### Documentation

- *(zed)* Add zed extension design and update installation guide
- *(zed)* Separate zed installation into dedicated use case and add settings guide
- *(readme)* Update zed installation guide and reorder options

### Features

- Consolidate development norms and add OSS constraint
- Use GHCR for pre-built dev container for faster setup
- *(lsp)* Implement real-time diagnostics and decouple core
- *(zed)* Add zed extension, update docs and workflow

### Performance

- Pre-build dependencies in Dockerfile for faster Dev Container startup
- Fix dependency caching in Dockerfile and resolve doc lints

### Refactor

- Eliminate sudo from post-create.sh by fixing Dockerfile user permissions
- Reordered Dockerfile to run cargo build as vscode user for cleaner permissions

## [0.2.0] - 2026-02-01

### Bug Fixes

- Ci failures (rustfmt and package-lock.json)
- *(vscode)* Update @types/node to 20 and sync lockfile
- Resolve unused variable warning
- Ignore anchors and links inside code fences
- Strip ID prefix from node name in headings
- Move architecture types to core dependency model
- Correct architecture relations per user specification
- Allow CTX to reference ACT
- CTX references only ACT
- Count explicit doc type refs, mandate ACT for CTX
- Enforce ACT must be referenced by UC, update UC-WRITE actor
- Clippy warnings
- Run cargo fmt to resolve CI formatting failures
- *(cli)* Resolve clippy warning in glob_to_regex
- Format code with cargo fmt
- Ignore IDs and references inside code blocks
- Format code with cargo fmt
- Remove deprecated root_path usage in LSP server
- Resolve clippy lint errors and borrow checker issues
- Split cargo llvm-cov to support both lcov and html
- Resolve TOML parse error in docgraph.toml
- *(lsp)* Pass ignore config and add SECURITY.md to ignore list
- Resolve clippy and fmt issues
- *(ci)* Fix clippy warnings and include all files
- Apply cargo fmt to CLI handlers
- Allow dead_code in common test utilities to fix CI failure
- Consolidate LSP tests and fix formatting
- Resolve unused code warnings by moving helpers and removing allow(dead_code)
- Resolve deprecation warnings for assert_cmd and fix CI failure
- Resolve all E2E test failures and deprecation warnings
- *(core)* Ignore anchors and links within inline code and code blocks
- Resolve clippy warnings and formatting issues
- Resolve clippy manual_pattern_char_comparison lints
- Robust rename handler logic
- *(lsp)* Resolve overlapping edits and canonicalize paths in rename
- Resolve clippy lints and syntax errors in LSP handlers
- *(core)* Resolve collapsible_if clippy warning in walk.rs
- *(lsp)* Include ID in symbol names for workspace/document symbol search
- *(test)* Update DG004 unit tests to match new function signature
- Replace verify command with type refinement workflow
- Update docgraph.toml to explicitly define UC reference rules
- Add --force to cargo install in devcontainer.json to avoid prompts
- Resolve clippy warnings and format code
- Upgrade node to 20 and add vsix metadata

### Documentation

- Clarify dependency model (derive vs depends)
- Cleanup task.md and register IF requirement
- Sync doc directory with latest features and commands
- Refactor integration_metrics.md and update requirements links
- Add ADR for documentation format and fix parser strictness
- Rebrand to Markdown Graph and add documentation overview
- Make README.md a concise entry point and doc/ master
- Add LSP specifications and update core workflows
- Update README.md to reflect new documentation structure and LSP features
- Finalize path updates and resolve all linter errors
- Fix relative paths in functional/verification.md
- Use CON-SOLO for automation constraint
- Reorganize architecture documentation into design and view
- Fix validation errors for CC-ARCH-OVERVIEW
- Reorganize architecture documentation into ADRs and CCs
- Refine VS Code extension specs and fix duplicate anchor ID example
- Audit anchor placement and refine ADR titles
- Add claude plugin guide and reorganize README for AI-first
- Update LSP specs documentation
- Add overlap listing format to verify command
- Refine overlap analysis steps in verify workflow
- Add user confirmation step to verify workflow
- Extend verify workflow to support NODE_ID verification
- Insert recursive node check in type verification workflow
- Update SKILL.md with align/refine commands and workflow tips
- Clarify align and refine as plugin custom commands in SKILL.md
- Use slash command syntax for align and refine in SKILL.md
- Resolve DG006 errors by linking UCs to FRs and IFs
- Fix absolute file paths to relative links
- Refactor architecture documentation structure
- Refactor testing documentation into distinct strategies (Unit, Perf, Coverage)
- Consolidate testing documentation into single strategy file
- Add SAST section to testing strategy and resolve lint errors
- Add CI/CD pipeline documentation and link to constraints and testing strategy
- Remove outdated tests section from module view and refine CI/CD doc
- Rename developer-guide.md to dev-container.md and add CC_DEV_ENV node
- Restructure dev-container.md and update links
- Separate manual install use case and requirements
- Remove redundant FR_INSTALL grouping node
- Add ADR for CI environment parity and align workflows
- Refine traceability rules and restructure documentation
- Overhaul README and reorganize setup documentation
- Rename SpecBlock to Node for terminology consistency

### Features

- Initial implementation of docgraph linter and generator
- Add vs code extension for previewing {document} blocks
- Add initial docgraph specifications
- Refactor graph output, add DG004 lint, and remove LSP support
- Implement validation rules DG005 and DG006
- Register new rules and dependencies
- Add architecture node types (CTX, BB, RT, DEP, CC)
- Finalize all reference rules and direction alignment
- *(dg003)* Add strict file link validation with auto-fix
- Add list command with prefix matching support
- Add trace command with up/down direction support
- Add describe command for bidirectional relationship view
- Include target ID's name in describe output
- Add optional description field to lint rules
- Improve check command UX and fix master documentation errors
- Include warning counts in check summary and fix suggestion
- Add lsp command for Language Server Protocol support
- Implement Go to Definition, Hover, Completion, Find References, and Rename in LSP
- Implement Call Hierarchy in LSP
- Introduce cargo-llvm-cov for code coverage
- Add developer docs and integrate test run into CI
- Relocate configuration doc to specs/config_specs.md
- Add rationale-based descriptions to docgraph.toml rules
- Add rationale-based descriptions to all docgraph.toml rules
- Align docgraph.toml with actual documentation structure
- Implement error handling strategy with thiserror and anyhow
- Implement E2E testing strategy with assert_cmd
- Introduce Biome for VSIX project and update CI/CD
- *(cli)* Add type subcommand to display node types
- Optimize lint performance by refactoring DG004
- Add CLI wrapper commands to docgraph-plugin
- Add verify workflow command
- Add refine-type workflow command
- Add tidy workflow command
- Add dev container configuration and update developer guide
- *(devcontainer)* Add Claude Code installation and configuration
- Add github release workflow and changelog configuration
- Add installation scripts for macOS/Linux and Windows

### Miscellaneous Tasks

- Revert CON ref_min to 1 (strict check)
- Final formatting and lint fixes for describe command
- Update Cargo.lock for LSP dependencies
- Security hardening (CI audit, permissions, policy)
- *(deps)* Bump softprops/action-gh-release from 1 to 2
- Refactor ci.yml for efficiency and speed
- Parallelize ci jobs (lint and test)
- *(deps)* Bump toml in the dependencies group
- Cleanup redundant release workflow and refine coverage visibility
- *(deps)* Bump actions/checkout from 4 to 6
- Relocate security policy to root as SECURITY.md
- Rename doc/devel/README.md to guide.md
- Split core_workflows.md and remove tests
- Restore CON type and remove ERR type
- Finalize docgraph.toml (restore CON, remove ERR)
- Cleanup redundant comments in docgraph.toml
- Rename markdown files to use kebab-case
- Replace test_data with real docs in CI and guide
- Remove empty tests directory
- *(core)* Improve coverage and fix warnings
- Add blank lines for readability in SECURITY.md
- Rename all IDs to underscore-separated format
- Sync remaining LSP handlers with ID renaming
- *(deps)* Bump actions/upload-artifact from 4 to 6
- *(deps-dev)* Bump @types/node from 16.18.126 to 25.1.0 in /vsix
- *(deps)* Bump vscode-languageclient from 8.1.0 to 9.0.1 in /vsix
- *(deps-dev)* Bump typescript from 4.9.5 to 5.9.3 in /vsix
- *(deps-dev)* Bump @vscode/vsce from 2.32.0 to 3.7.1 in /vsix
- Cleanup unused shell scripts and update documentation
- Enforce strict consistency rules for Use Cases in docgraph.toml
- Run docgraph check --fix
- *(devcontainer)* Optimize startup speed using cargo-binstall
- Bump version to 0.2.0

### Refactor

- Update documentation IDs to match new node types
- Refined dependency model based on feedback
- Separate documentation types from core relation checks
- Convert validation rules to functional requirements
- Rename config keys (relations->references, ref->to)
- Split lsp.rs into multiple files under lsp directory
- Remove old lsp.rs after splitting
- Split handlers.rs into sub-modules for each LSP capability
- Remove old handlers.rs after splitting
- Successfully restructured project into core, cli, and lsp modules
- Decouple core logic from CLI display by removing direct terminal output
- Consolidate E2E tests into a single binary to resolve dead_code warnings
- Rename verify command to verify-structure
- Rename plugin commands to refine and align
- Move devcontainer setup to script

### Styling

- Fix formatting issues to pass CI
- Fix formatting
- Change rule description to single line format
- Apply cargo fmt across the project
- Run cargo fmt to fix CI failures
- Run cargo fmt to fix formatting issues
- Fix formatting after E2E tests consolidation
- Fix formatting (again)
- Format dg006.rs
- Apply cargo fmt
- Format navigation.rs
- Apply cargo fmt to DG004 tests
- Fix trailing newlines in ADR

### Testing

- Add cli and lsp unit tests
- Reorganize E2E tests and chore: rename concept IDs to underscores

### Ci

- Add clippy and fmt checks
- Add explicit CodeQL workflow for Rust and JavaScript
- Split workflow into lint and test jobs for parallelism

<!-- generated by git-cliff -->
