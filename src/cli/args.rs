use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "docgraph")]
#[command(version, about = "A linter and graph generator for Markdown document graphs", long_about = None)]
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
    /// Describe a spec block and its relationships
    Describe {
        /// The ID of the spec block to describe
        id: String,

        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Show node type information from configuration
    Type {
        /// Type ID to show details for (optional)
        #[arg(index = 1)]
        type_id: Option<String>,
    },
    /// Query the documentation graph using Cypher
    Query {
        /// Cypher query string
        query: String,

        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,

        /// Path to search for markdown files (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Start the language server
    Lsp,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Table,
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_check_default() {
        let cli = Cli::parse_from(["docgraph", "check"]);
        match cli.command {
            Commands::Check {
                path,
                json,
                fix,
                rule,
            } => {
                assert_eq!(path, PathBuf::from("."));
                assert!(!json);
                assert!(!fix);
                assert!(rule.is_none());
            }
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn test_check_flags() {
        let cli = Cli::parse_from([
            "docgraph", "check", "./doc", "--json", "--fix", "--rule", "MD001",
        ]);
        match cli.command {
            Commands::Check {
                path,
                json,
                fix,
                rule,
            } => {
                assert_eq!(path, PathBuf::from("./doc"));
                assert!(json);
                assert!(fix);
                assert_eq!(rule, Some(vec!["MD001".to_string()]));
            }
            _ => panic!("Expected Check command"),
        }
    }
}
