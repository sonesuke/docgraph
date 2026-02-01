<a id="CC_COVERAGE"></a>

# Code Coverage Standards

We use `cargo-llvm-cov` to measure the effectiveness of our tests.

## Running Coverage Locally

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

- **Cargo llvm-cov**: Coverage report generation.

We strive for high coverage, especially in the `core` logic responsible for parsing and graph validation.
