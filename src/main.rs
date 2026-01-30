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
        Commands::List { query, path } => {
            let (blocks, _refs) = collect::collect_workspace_all(&path);
            let regex_str = glob_to_regex(&query);
            let re = match regex::Regex::new(&regex_str) {
                Ok(re) => re,
                Err(e) => {
                    eprintln!("Error: Invalid query '{}': {}", query, e);
                    return ExitCode::FAILURE;
                }
            };

            for block in blocks {
                if re.is_match(&block.id) {
                    println!(
                        "{} : {}",
                        block.id,
                        block.name.as_deref().unwrap_or("No description")
                    );
                }
            }
        }
        Commands::Trace {
            from,
            to,
            path,
            direction,
        } => {
            let (blocks, _refs) = collect::collect_workspace_all(&path);
            let target_regex_str = glob_to_regex(&to);
            let target_re = regex::Regex::new(&target_regex_str).unwrap();

            let mut adjacency: std::collections::HashMap<String, Vec<String>> =
                std::collections::HashMap::new();

            if direction == "down" {
                for block in &blocks {
                    let targets: Vec<String> = block.edges.iter().map(|e| e.id.clone()).collect();
                    adjacency.insert(block.id.clone(), targets);
                }
            } else if direction == "up" {
                for block in &blocks {
                    for edge in &block.edges {
                        adjacency
                            .entry(edge.id.clone())
                            .or_default()
                            .push(block.id.clone());
                    }
                }
            } else {
                eprintln!(
                    "Error: Invalid direction '{}'. Use 'down' or 'up'.",
                    direction
                );
                return ExitCode::FAILURE;
            }

            if !blocks.iter().any(|b| b.id == from) && direction == "down" {
                eprintln!("Error: Start ID '{}' not found.", from);
            }

            let mut paths = Vec::new();
            let mut current_path = vec![from.clone()];
            let mut visited = std::collections::HashSet::new();
            visited.insert(from.clone());

            find_paths(
                &from,
                &target_re,
                &adjacency,
                &mut visited,
                &mut current_path,
                &mut paths,
            );

            if paths.is_empty() {
                println!("No paths found from '{}' to '{}'.", from, to);
            } else {
                for path in paths {
                    let sep = if direction == "down" { " -> " } else { " <- " };
                    println!("{}", path.join(sep));
                }
            }
        }
        Commands::Describe { id, path } => {
            let (blocks, _refs) = collect::collect_workspace_all(&path);

            // Find the target block
            let target_block = blocks.iter().find(|b| b.id == id);

            match target_block {
                Some(block) => {
                    println!("{} references to", id);
                    for edge in &block.edges {
                        let name = blocks
                            .iter()
                            .find(|b| b.id == edge.id)
                            .and_then(|b| b.name.as_deref())
                            .unwrap_or("No description");
                        println!("{}: {}", edge.id, name);
                    }

                    println!("\nThe following IDs are depends on {}", id);
                    let mut found_depend = false;
                    for other in &blocks {
                        if other.edges.iter().any(|e| e.id == id) {
                            let name = other.name.as_deref().unwrap_or("No description");
                            println!("{}: {}", other.id, name);
                            found_depend = true;
                        }
                    }
                    if !found_depend {
                        println!("(None)");
                    }
                }
                None => {
                    eprintln!("Error: ID '{}' not found.", id);
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    ExitCode::SUCCESS
}

fn find_paths(
    current: &str,
    target_re: &regex::Regex,
    adjacency: &std::collections::HashMap<String, Vec<String>>,
    visited: &mut std::collections::HashSet<String>,
    current_path: &mut Vec<String>,
    paths: &mut Vec<Vec<String>>,
) {
    if target_re.is_match(current) && current_path.len() > 1 {
        paths.push(current_path.clone());
        // Continue searching? In many trace cases, we want all paths,
        // but maybe stop at first match in this branch to avoid redundant sub-paths.
        // Let's continue to find potentially deeper matches if they exist,
        // but usually, once we hit a "FR-*" we might stop.
        // For simplicity, let's continue.
    }

    if let Some(neighbors) = adjacency.get(current) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                visited.insert(neighbor.clone());
                current_path.push(neighbor.clone());
                find_paths(neighbor, target_re, adjacency, visited, current_path, paths);
                current_path.pop();
                visited.remove(neighbor);
            }
        }
    }
}

fn glob_to_regex(glob: &str) -> String {
    let mut regex = String::from("^");
    let has_wildcard = glob.contains('*') || glob.contains('?');

    for c in glob.chars() {
        match c {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            '.' | '+' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '\\' => {
                regex.push('\\');
                regex.push(c);
            }
            _ => regex.push(c),
        }
    }

    if has_wildcard {
        regex.push('$');
    }
    regex
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
