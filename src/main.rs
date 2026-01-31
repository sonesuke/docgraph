use docgraph::cli;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    cli::run().await
}
