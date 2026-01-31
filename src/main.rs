mod cli;
mod config;
mod parse;
mod rules;
mod types;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    
    match args.command {
        Commands::Check { path, json, fix, rule } => {
            // Implementation...
            println!("Checking graph at {:?}...", path);
        }
        Commands::Graph { path } => {
            // Implementation...
            println!("Generating graph for {:?}...", path);
        }
        _ => {
            println!("Command not yet implemented in this preview.");
        }
    }
    
    Ok(())
}
