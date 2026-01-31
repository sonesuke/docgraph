# Non-Functional Requirements: VS Code Extension

These requirements define the quality attributes and constraints of the `docgraph` VS Code extension.

<a id="NFR_VSC_001"></a>

## Fast Activation

The extension SHALL activate in less than 1 second on a standard development machine to ensure it does not negatively impact the VS Code startup time or workspace loading.

**Realized by:**

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

<a id="NFR_VSC_002"></a>

## Cross-platform Portability

The extension (LSP client) SHALL be written in TypeScript/JavaScript to ensure it runs natively on all platforms supported by VS Code (macOS, Linux, Windows).

**Realized by:**

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

<a id="NFR_VSC_003"></a>

## Lightweight Packaging

The bundled `.vsix` file SHALL be minimized and excluded of unnecessary development dependencies to keep the download size small and the extension footprint low.

**Realized by:**

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)
