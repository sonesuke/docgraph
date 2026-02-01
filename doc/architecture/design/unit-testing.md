<a id="CC_UNIT_TESTING"></a>

# Unit Testing Standards

## Principles

- **Isolation**: Tests should run independently.
- **Speed**: Unit tests must be fast to encourage frequent execution.
- **Coverage**: Core logic must have high validation coverage.

## Unit Tests

Located within the source files (usually in a `#[cfg(test)]` module). Use these for isolated logic like parsing unique patterns or validating specific rule logic.

## Integration Tests

Located in `tests/`. These test the behavior of the CLI and core components as a whole, often using real or mock Markdown files. While broader than unit tests, they are treated as part of the developer's local verification loop.

## Running Tests

Run the full suite using standard cargo commands:

```bash
cargo test
```
