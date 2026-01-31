use super::common::print_diagnostics;
use crate::core::{config, lint, types};
use anyhow::Context;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_check(path: PathBuf, json: bool, fix: bool, rule: Option<Vec<String>>) -> ExitCode {
    match try_check(path, json, fix, rule) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn try_check(
    path: PathBuf,
    json: bool,
    fix: bool,
    rule: Option<Vec<String>>,
) -> anyhow::Result<ExitCode> {
    let config = config::Config::load(&path).context("failed to load docgraph.toml")?;

    let diagnostics = lint::check_workspace(&path, fix, rule, true, &config);

    if json {
        let json_out = serde_json::to_string_pretty(&diagnostics)
            .context("failed to serialize diagnostics to JSON")?;
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
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

pub fn handle_fmt(path: PathBuf, rule: Option<Vec<String>>) -> ExitCode {
    match try_fmt(path, rule) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn try_fmt(path: PathBuf, rule: Option<Vec<String>>) -> anyhow::Result<ExitCode> {
    let config = config::Config::load(&path).context("failed to load docgraph.toml")?;
    let diagnostics = lint::check_workspace(&path, true, rule, false, &config);
    print_diagnostics(&diagnostics);
    Ok(ExitCode::SUCCESS)
}
