mod cli;
mod collect;
mod lint;
mod parse;
mod types;
mod walk;

use clap::Parser;
use cli::{Cli, Commands};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Cli::parse();

    match args.command {
        Commands::Lint { path, json } => {
            let diagnostics = lint::check_workspace(&path);
            
            if json {
                let json_out = serde_json::to_string_pretty(&diagnostics).unwrap();
                println!("{}", json_out);
            } else {
                for d in &diagnostics {
                    // Simple human readable format
                    // error[CODE] path:line:col: message
                    println!(
                        "{}[{}] {}:{}:{}: {}",
                        match d.severity {
                            types::Severity::Error => "error",
                            types::Severity::Warning => "warning",
                        },
                        d.code,
                        d.path.display(),
                        d.range.start_line,
                        d.range.start_col,
                        d.message
                    );
                }
            }

            if diagnostics.iter().any(|d| matches!(d.severity, types::Severity::Error)) {
                return ExitCode::FAILURE;
            }
        }
        Commands::Gen { path, json } => {
            let blocks = collect::collect_workspace(&path);
             if json {
                let json_out = serde_json::to_string_pretty(&blocks).unwrap();
                println!("{}", json_out);
            } else {
                // Currently only JSON is supported/requested. 
                // Fallback to JSON or print nothing/error?
                // User asked for `docgraph gen --json`. If no json flag, maybe print count or simple list?
                // Let's print JSON by default if strictly following "gen --json" request implies flag is needed, 
                // but usually gen implies output.
                // However, I'll respect the flag. If no flag, maybe print summary.
                println!("Found {} documents.", blocks.len());
            }
        }
    }

    ExitCode::SUCCESS
}
