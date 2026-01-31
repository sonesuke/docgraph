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
    /// Check the documentation graph for errors
    Check {
        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output diagnostics in JSON format
        #[arg(long)]
        json: bool,

        /// Automatically fix fixable issues
        #[arg(long)]
        fix: bool,

        /// Run only specific rules (can be specified multiple times)
        #[arg(long)]
        rule: Option<Vec<String>>,
    },
    /// Format the documentation (fix fixable issues)
    Fmt {
        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Run only specific rules (can be specified multiple times)
        #[arg(long)]
        rule: Option<Vec<String>>,
    },
    /// Show information about available rules
    Rule {
        /// Rule name to search for (optional)
        #[arg(index = 1)]
        rule: Option<String>,
    },
    /// Generate graph data
    Graph {
        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// List spec blocks matching a query
    List {
        /// Query pattern (e.g., "FR-*")
        query: String,

        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Trace relationships between spec blocks
    Trace {
        /// Start ID
        from: String,

        /// Target ID or prefix
        to: String,

        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Direction of trace (down: outgoing, up: incoming)
        #[arg(long, default_value = "down")]
        direction: String,
    },
    /// Describe a spec block and its relationships
    Describe {
        /// The ID of the spec block to describe
        id: String,

        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}
