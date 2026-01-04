# Docgraph VS Code Preview Extension

This extension provides a transparent preview for `docgraph` blocks (e.g., `{document}`) in the VS Code Markdown preview.

## Features

- Renders `{document}` fenced code blocks as regular Markdown content.
- Hides the fence lines and option lines (lines starting with `:` at the beginning of the block).
- Supports `:class: docgraph` for other blocks (like `{admonition}`).

## Usage

1. Open a Markdown file containing `{document}` blocks.
2. Open the standard VS Code Preview (`Ctrl+Shift+V` or `Cmd+Shift+V`).
3. The `{document}` blocks should be rendered transparently (no code fence, options hidden).

## Installation

### From VSIX
1. Build the package:
   ```bash
   npm install
   npm run package
   ```
2. Install in VS Code:
   - "Extensions" view -> "..." menu -> "Install from VSIX..."
   - Select the generated `.vsix` file.

### For Development
1. Open this folder in VS Code.
2. Press `F5` to launch Extension Development Host.

## Development Commands

- `npm install`: Install dependencies
- `npm test`: Run unit tests
- `npm run lint`: Lint code
- `npm run package`: Create .vsix package
