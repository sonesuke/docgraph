use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "docgraph")]
#[command(version, about = "A linter and graph generator for MyST document graphs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lint the documentation graph
    Lint {
        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output diagnostics in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Generate graph data
    Gen {
        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output graph content in JSON format
        #[arg(long)]
        json: bool,
    },
}
