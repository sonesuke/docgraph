# Docgraph VS Code Extension

Interactive modeling support for `docgraph`.

## Features

- **Real-time Diagnostics**: Identification of duplicate IDs, missing headings, and broken references.
- **Go to Definition**: Jump from a reference to the spec block definition.
- **Find References**: See all references to a spec block ID.
- **Symbol Rename**: Rename a SpecBlock ID and update all references.

## Configuration

- `docgraph.binaryPath`: Absolute path to the `docgraph` binary (defaults to `docgraph`).

## Requirements

The `docgraph` binary must be installed and available in your `PATH` or configured via `docgraph.binaryPath`.
