mod cli;
mod collect;
mod config;
mod lint;

mod parse;
mod rules;
mod types;
mod walk;

use clap::Parser;
use cli::{Cli, Commands};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Cli::parse();

    match args.command {
        Commands::Check {
            path,
            json,
            fix,
            rule,
        } => {
            let config = config::Config::load(&path).unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load docgraph.toml: {}", e);
                config::Config::default()
            });
            let diagnostics = lint::check_workspace(&path, fix, rule, true, &config);

            if json {
                let json_out = serde_json::to_string_pretty(&diagnostics).unwrap();
                println!("{}", json_out);
            } else {
                print_diagnostics(&diagnostics);
            }

            if diagnostics
                .iter()
                .any(|d| matches!(d.severity, types::Severity::Error))
            {
                return ExitCode::FAILURE;
            }
        }
        Commands::Fmt { path, rule } => {
            let config = config::Config::load(&path).unwrap_or_default();
            let diagnostics = lint::check_workspace(&path, true, rule, false, &config);
            print_diagnostics(&diagnostics);

            // Fmt normally doesn't fail on lint errors unless critical,
            // but for consistency with check --fix, we might want to return failure if errors remain.
            // Let's return success for fmt usually, unless we want strict checking.
            // But if there are unfixable errors (MD051), maybe warning is better?
            // User didn't specify. I'll behave like 'cargo fmt' which usually succeeds.
            // But if check fails, maybe it should fail?
            // Let's keep it SUCCESS for now, as it's a formatter.
        }
        Commands::Rule { rule } => {
            let rumdl_config = rumdl_lib::config::Config::default();
            let mut all_rules = rumdl_lib::rules::all_rules(&rumdl_config);
            all_rules.push(Box::new(rules::dg001::DG001));
            all_rules.push(Box::new(rules::dg002::DG002));
            all_rules.push(Box::new(rules::dg003::DG003));
            all_rules.push(Box::new(rules::dg004::DG004));

            if let Some(rule_query) = rule {
                let rule_query = rule_query.to_ascii_uppercase();
                let found = all_rules.iter().find(|r| {
                    r.name().eq_ignore_ascii_case(&rule_query)
                        || r.name().replace("MD", "") == rule_query.replace("MD", "")
                });

                if let Some(rule) = found {
                    println!(
                        "{} - {}\n\nDescription:\n  {}",
                        rule.name(),
                        rule.description(),
                        rule.description()
                    );
                } else {
                    eprintln!("Rule '{}' not found.", rule_query);
                    return ExitCode::FAILURE;
                }
            } else {
                println!("Available rules:");
                for rule in &all_rules {
                    println!("  {} - {}", rule.name(), rule.description());
                }
            }
        }
        Commands::Graph { path } => {
            let (blocks, _refs) = collect::collect_workspace_all(&path);
            let json_out = serde_json::to_string_pretty(&blocks).unwrap();
            println!("{}", json_out);
        }
    }

    ExitCode::SUCCESS
}

fn print_diagnostics(diagnostics: &[types::Diagnostic]) {
    for d in diagnostics {
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
