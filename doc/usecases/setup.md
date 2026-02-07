# Environment Setup Use Cases

<a id="UC_INSTALL_MANUAL"></a>

## Install Manual Setup

The developer sets up the development environment manually on their local machine.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)
- [ACT_USER (User)](../actors/users.md#ACT_USER)

### Interfaces

- [IF_GITHUB_RELEASES (GitHub Releases Interface)](../requirements/interfaces/interfaces.md#IF_GITHUB_RELEASES)

### Requirements

- [FR_INSTALL_MANUAL (Manual Installation)](../requirements/functional/installation.md#FR_INSTALL_MANUAL) Required for users choosing not to use automated scripts
- [FR_CORE_AUDIT (Audit Logging)](../requirements/functional/core.md#FR_CORE_AUDIT) Ensures installation events are tracked for security

### Flow

1. Developer ensures local environment meets specifications.
2. Developer manually installs required tools.

---

<a id="UC_INSTALL_BINARY"></a>

## Install via Binary Script

The developer installs the `docgraph` CLI binary using automated scripts.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_GITHUB_RELEASES (GitHub Releases Interface)](../requirements/interfaces/interfaces.md#IF_GITHUB_RELEASES)

### Requirements

- [FR_INSTALL_BINARY (Binary Installation Support)](../requirements/functional/installation.md#FR_INSTALL_BINARY) Provides a fast and automated way to set up the CLI

### Flow

1. Developer runs the platform-specific install script.
2. Script downloads the latest binary from GitHub Releases.

---

<a id="UC_CLAUDE_INSTALL"></a>

## Install Claude Plugin

The developer installs the Docgraph plugin in Claude Desktop.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_CLAUDE_MARKETPLACE (Claude Marketplace)](../requirements/interfaces/interfaces.md#IF_CLAUDE_MARKETPLACE)

### Requirements

- [FR_CLAUDE_INSTALL (Claude Plugin Installation)](../requirements/functional/claude.md#FR_CLAUDE_INSTALL) Mandatory for enabling Docgraph within Claude Desktop

### Flow

1. Developer adds the marketplace to Claude.
2. Developer installs the Docgraph plugin.

---

<a id="UC_VSCODE_INSTALL"></a>

## Install VS Code Extension

The developer installs the `docgraph` VS Code extension.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_VSCODE_MARKETPLACE (VS Code Marketplace)](../requirements/interfaces/interfaces.md#IF_VSCODE_MARKETPLACE)

### Requirements

- [FR_VSC_BINARY_PATH (Binary Path Configuration)](../requirements/functional/vscode.md#FR_VSC_BINARY_PATH) Enables the extension to find the docgraph engine
- [FR_VSC_MARKDOWN_ACTIVATION (Markdown Activation)](../requirements/functional/vscode.md#FR_VSC_MARKDOWN_ACTIVATION) Restricts extension logic to Markdown files
- [FR_VSC_SERVER_LIFECYCLE (Server Lifecycle Commands)](../requirements/functional/vscode.md#FR_VSC_SERVER_LIFECYCLE) Allows manual control over the LSP server
- [FR_CORE_AUTH (Authentication)](../requirements/functional/core.md#FR_CORE_AUTH) Securely identifies the developer

### Flow

1. Developer downloads the VSIX file.
2. Developer installs the extension in VS Code.

### Non-Functional Requirements (Optional)

- [NFR_VSCODE_PACKAGING (Lightweight Packaging)](../requirements/non-functional/vscode.md#NFR_VSCODE_PACKAGING) Ensures the extension is fast to download and install

---

<a id="UC_ZED_INSTALL"></a>

## Install Zed Extension

The developer installs the `docgraph` Zed extension.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_ZED_UI (Zed UI)](../requirements/interfaces/interfaces.md#IF_ZED_UI)

### Requirements

- [FR_INSTALL_EXT_ZED (Zed Editor Extension)](../requirements/functional/installation.md#FR_INSTALL_EXT_ZED) Enables Docgraph assistance within the Zed editor

### Flow

1. Developer builds the extension targetting wasm32-wasip1.
2. Developer installs the dev extension in Zed.
