# Testing & Coverage Standards

We prioritize code quality and correctness. This document outlines our testing strategies and how we measure coverage.

## Testing Strategy

### 1. Unit Tests

Located within the source files (usually in a `#[cfg(test)]` module). Use these for isolated logic like parsing unique patterns or validating specific rule logic.

### 2. Integration Tests

Located in `tests/`. These test the behavior of the CLI and core components as a whole, often using real or mock Markdown files.

## Running Tests

Run the full suite using standard cargo commands:

```bash
cargo test
```

## Code Coverage

We use `cargo-llvm-cov` to measure the effectiveness of our tests.

### Running Coverage Locally

1. Install the tool: `cargo install cargo-llvm-cov`
2. Install the LLVM tools preview: `rustup component add llvm-tools-preview`
3. Generate a summary:

   ```bash
   cargo llvm-cov
   ```

4. View a detailed HTML report:

   ```bash
   cargo llvm-cov --html
   open target/llvm-cov/html/index.html
   ```

## CI/CD Pipeline

Every Pull Request is subjected to:

- **Clippy**: Linting checks.
- **Cargo Audit**: Security checks for dependencies.
- **Cargo Test**: Full test suite execution.
- **Cargo llvm-cov**: Coverage report generation.

We strive for high coverage, especially in the `core` logic responsible for parsing and graph validation.
