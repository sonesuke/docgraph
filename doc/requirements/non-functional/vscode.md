# Non-Functional Requirements: VS Code Extension

These requirements define the quality attributes and constraints of the `docgraph` VS Code extension.

<a id="NFR_VSCODE_PORTABILITY"></a>

## Cross-platform Portability

The extension (LSP client) SHALL be written in TypeScript/JavaScript to ensure it runs natively on all platforms
supported by VS Code (macOS, Linux, Windows).

### Qualifies (Optional)

- [FR_VSC_LSP_CLIENT (LSP Client Integration)](../../requirements/functional/vscode.md#FR_VSC_LSP_CLIENT)

<a id="NFR_VSCODE_PACKAGING"></a>

## Lightweight Packaging

The bundled `.vsix` file SHALL be minimized and excluded of unnecessary development dependencies to keep the download
size small and the extension footprint low.

### Qualifies (Optional)

- [FR_VSC_BINARY_PATH (Binary Path Configuration)](../../requirements/functional/vscode.md#FR_VSC_BINARY_PATH)
