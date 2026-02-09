# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2026-02-08

### Bug Fixes

- _(test)_ Resolve clippy cloned_ref_to_slice_refs warnings
- _(parser)_ Prevent block name overwrite by subsequent headings
- _(vsix)_ Resolve npm vulnerabilities and enable ci check (#72)
- _(devcontainer)_ Propagate CI env var for startup optimization (#84)
- Add tmux to devcontainer and fix remoteUser (#93)
- Improve devcontainer configuration (#98)

### Features

- _(docker)_ Optimize image size & fix CI environment (#86)
- Enhance template validation (DG007) and standardize use cases (#87)
- _(core)_ Implement DG007 template validation and fix templates (#88)
- Refactor Configuration Specs and Enforce Strict Dependencies (#90)
- Semantic error messages and traceability refactoring (#94)
- _(cli)_ Improve type, describe, and list command output (#95)
- Enhance validate skill with template and structure checks
- Enhance file placement validation with consistency checks
- Add SRP check to validate skill
- Redefine validate skill as Validation Quality Gate
- Add Quality Gate Checklist to validate skill
- Evolve validate skill into a structured Semantic Linter
- Evolve refine skill into Deep Consistency Gate
- Rename refine skill to align
- Refine align skill as an Architecture and Meaning Gate
- Add Quality Gate Checklist to align skill
- Implement trace skill as Realization and Flow Gate
- Add Quality Gate Checklist to trace skill
- Final polish for trace skill (Architecture & Meaning)
- Migrate slash commands to antigravity skills (#97)

### Miscellaneous Tasks

- Update dependencies
- _(deps)_ Bump actions/download-artifact from 4 to 7 (#73)
- _(deps)_ Bump actions/checkout from 4 to 6 (#74)
- _(deps)_ Bump actions/upload-artifact from 4 to 6 (#78)
- _(deps-dev)_ Bump @types/node from 25.1.0 to 25.2.1 in /vsix (#77)
- _(deps)_ Bump github/codeql-action from 3 to 4 (#76)
- _(deps-dev)_ Bump @biomejs/biome from 2.3.13 to 2.3.14 in /vsix (#79)
- _(ci)_ Suppress git init default branch warning (conflict resolved) (#83)
- _(devcontainer)_ Wrap dev setup in CI check for startup optimization (#82)
- _(deps)_ Bump the dependencies group with 2 updates (#80)
- _(deps-dev)_ Bump @types/vscode from 1.108.1 to 1.109.0 in /vsix (#75)
- _(deps)_ Bump lsp-types from 0.94.1 to 0.97.0 in the dependencies group (#89)
- Integrate prettier for markdown formatting (#96)
- Migrate slash commands to antigravity skills
- Remove redundant docgraph skill and simplify align/refine skills
- Update example IDs in skills to use underscores
- Rename align skill to validate
- Remove unnecessary (NEW) labels from SKILL.md
- Emphasize strictness in validate skill
- Correct skill name and report headers to align
- Polish plugin description to reflect architectural governance
- Bump version to 0.3.0 and update CHANGELOG

### Performance

- _(ci)_ Implement granular steps with persistent workspace cache (#85)

### Refactor

- Replace rumdl with custom pulldown-cmark parser and restore rules
- _(lsp)_ Remove tower-lsp and optimize dependencies (#69)

### Ci

- Use devcontainers/ci for release workflow (#70)
- _(dependabot)_ Add zed-extension config (#71)

## [0.2.1] - 2026-02-06

### Bug Fixes

- Check for code command in post-create.sh to fix CI failure
- Use more robust check for code command in post-create.sh
- Pin rumdl to 0.1.10 and fix compilation error
- Install tools as root in Dockerfile and simplify devcontainer.json
- Correct relative path for MOD_CICD link and fix Dockerfile permissions
- Use sudo for cargo install in post-create.sh
- Dummy bench file and doc lints
- Move CARGO_TARGET_DIR to user home to fix permission denied in CI
- Move CARGO_HOME to user home to resolve registry permission errors
- _(ci)_ Add packages:read permission to CI workflow for GHCR cache access
- _(ci)_ Add explicit ghcr login and use full image URL for cache
- _(ci)_ Pre-build debug dependency artifacts in Dockerfile to speed up CI
- _(zed)_ Update zed_extension_api to 0.7.0 and bind to Markdown language
- Optimize devcontainer build config (#65)

### Documentation

- _(zed)_ Add zed extension design and update installation guide
- _(zed)_ Separate zed installation into dedicated use case and add settings guide
- _(readme)_ Update zed installation guide and reorder options

### Features

- Consolidate development norms and add OSS constraint
- Use GHCR for pre-built dev container for faster setup
- _(lsp)_ Implement real-time diagnostics and decouple core
- _(zed)_ Add zed extension, update docs and workflow

### Miscellaneous Tasks

- Set default user to vscode in Dockerfile
- Remove zed-extension/target from git and add .gitignore
- Cleanup zed-extension artifacts and update .gitignore
- _(release)_ Automate CHANGELOG.md generation and commitment
- _(release)_ Bump version to 0.2.1 (#62)
- _(release)_ Allow release to continue if changelog push fails (#63)
- _(release)_ Remove zed extension from release and ci (#64)
- Refine release workflow and documentation (#66)
- _(release)_ Prepare v0.2.1 (#67)

### Performance

- Pre-build dependencies in Dockerfile for faster Dev Container startup
- Fix dependency caching in Dockerfile and resolve doc lints

### Refactor

- Eliminate sudo from post-create.sh by fixing Dockerfile user permissions
- Reordered Dockerfile to run cargo build as vscode user for cleaner permissions

## [0.2.0] - 2026-02-01

### Bug Fixes

- Ci failures (rustfmt and package-lock.json)
- _(vscode)_ Update @types/node to 20 and sync lockfile
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
- _(cli)_ Resolve clippy warning in glob_to_regex
- Format code with cargo fmt
- Ignore IDs and references inside code blocks
- Format code with cargo fmt
- Remove deprecated root_path usage in LSP server
- Resolve clippy lint errors and borrow checker issues
- Split cargo llvm-cov to support both lcov and html
- Resolve TOML parse error in docgraph.toml
- _(lsp)_ Pass ignore config and add SECURITY.md to ignore list
- Resolve clippy and fmt issues
- _(ci)_ Fix clippy warnings and include all files
- Apply cargo fmt to CLI handlers
- Allow dead_code in common test utilities to fix CI failure
- Consolidate LSP tests and fix formatting
- Resolve unused code warnings by moving helpers and removing allow(dead_code)
- Resolve deprecation warnings for assert_cmd and fix CI failure
- Resolve all E2E test failures and deprecation warnings
- _(core)_ Ignore anchors and links within inline code and code blocks
- Resolve clippy warnings and formatting issues
- Resolve clippy manual_pattern_char_comparison lints
- Robust rename handler logic
- _(lsp)_ Resolve overlapping edits and canonicalize paths in rename
- Resolve clippy lints and syntax errors in LSP handlers
- _(core)_ Resolve collapsible_if clippy warning in walk.rs
- _(lsp)_ Include ID in symbol names for workspace/document symbol search
- _(test)_ Update DG004 unit tests to match new function signature
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
- _(dg003)_ Add strict file link validation with auto-fix
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
- _(cli)_ Add type subcommand to display node types
- Optimize lint performance by refactoring DG004
- Add CLI wrapper commands to docgraph-plugin
- Add verify workflow command
- Add refine-type workflow command
- Add tidy workflow command
- Add dev container configuration and update developer guide
- _(devcontainer)_ Add Claude Code installation and configuration
- Add github release workflow and changelog configuration
- Add installation scripts for macOS/Linux and Windows

### Miscellaneous Tasks

- Revert CON ref_min to 1 (strict check)
- Final formatting and lint fixes for describe command
- Update Cargo.lock for LSP dependencies
- Security hardening (CI audit, permissions, policy)
- _(deps)_ Bump softprops/action-gh-release from 1 to 2
- Refactor ci.yml for efficiency and speed
- Parallelize ci jobs (lint and test)
- _(deps)_ Bump toml in the dependencies group
- Cleanup redundant release workflow and refine coverage visibility
- _(deps)_ Bump actions/checkout from 4 to 6
- Relocate security policy to root as SECURITY.md
- Rename doc/devel/README.md to guide.md
- Split core_workflows.md and remove tests
- Restore CON type and remove ERR type
- Finalize docgraph.toml (restore CON, remove ERR)
- Cleanup redundant comments in docgraph.toml
- Rename markdown files to use kebab-case
- Replace test_data with real docs in CI and guide
- Remove empty tests directory
- _(core)_ Improve coverage and fix warnings
- Add blank lines for readability in SECURITY.md
- Rename all IDs to underscore-separated format
- Sync remaining LSP handlers with ID renaming
- _(deps)_ Bump actions/upload-artifact from 4 to 6
- _(deps-dev)_ Bump @types/node from 16.18.126 to 25.1.0 in /vsix
- _(deps)_ Bump vscode-languageclient from 8.1.0 to 9.0.1 in /vsix
- _(deps-dev)_ Bump typescript from 4.9.5 to 5.9.3 in /vsix
- _(deps-dev)_ Bump @vscode/vsce from 2.32.0 to 3.7.1 in /vsix
- Cleanup unused shell scripts and update documentation
- Enforce strict consistency rules for Use Cases in docgraph.toml
- Run docgraph check --fix
- _(devcontainer)_ Optimize startup speed using cargo-binstall
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
