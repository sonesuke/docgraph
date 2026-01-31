use super::common::print_diagnostics;
use crate::core::{config, lint, types};
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_check(path: PathBuf, json: bool, fix: bool, rule: Option<Vec<String>>) -> ExitCode {
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

        let error_count = diagnostics
            .iter()
            .filter(|d| matches!(d.severity, types::Severity::Error))
            .count();
        let warning_count = diagnostics
            .iter()
            .filter(|d| matches!(d.severity, types::Severity::Warning))
            .count();

        if error_count == 0 && warning_count == 0 {
            println!("No errors or warnings found.");
        } else {
            let mut summary = Vec::new();
            if error_count > 0 {
                summary.push(format!("{} error(s)", error_count));
            }
            if warning_count > 0 {
                summary.push(format!("{} warning(s)", warning_count));
            }

            let summary_str = summary.join(" and ");

            if !fix {
                println!(
                    "\nFound {}. Run with --fix to automatically fix some issues.",
                    summary_str
                );
            } else {
                println!(
                    "\nFound {} that could not be fixed automatically.",
                    summary_str
                );
            }
        }
    }

    if diagnostics
        .iter()
        .any(|d| matches!(d.severity, types::Severity::Error))
    {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

pub fn handle_fmt(path: PathBuf, rule: Option<Vec<String>>) -> ExitCode {
    let config = config::Config::load(&path).unwrap_or_default();
    let diagnostics = lint::check_workspace(&path, true, rule, false, &config);
    print_diagnostics(&diagnostics);
    ExitCode::SUCCESS
}
