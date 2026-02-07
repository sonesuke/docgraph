# Non-Functional Requirements: VS Code Extension

These requirements define the quality attributes and constraints of the `docgraph` VS Code extension.

<a id="NFR_VSCODE_PORTABILITY"></a>

## Cross-platform Portability

The extension (LSP client) SHALL be written in TypeScript/JavaScript to ensure it runs natively on all platforms supported by VS Code (macOS, Linux, Windows).

### Realized by (Optional)

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)

<a id="NFR_VSCODE_PACKAGING"></a>

## Lightweight Packaging

The bundled `.vsix` file SHALL be minimized and excluded of unnecessary development dependencies to keep the download size small and the extension footprint low.

### Realized by (Optional)

- [MOD_VSEX (VS Code Extension)](../../architecture/view/module.md#MOD_VSEX)
