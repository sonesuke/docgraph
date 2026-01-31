use std::process::ExitCode;
use docgraph::cli;

#[tokio::main]
async fn main() -> ExitCode {
    cli::run().await
}
