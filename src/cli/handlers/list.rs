use super::common::glob_to_regex;
use crate::core::{collect, config};
use anyhow::Context;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_list(query: String, path: PathBuf) -> ExitCode {
    match try_list(query, path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn try_list(query: String, path: PathBuf) -> anyhow::Result<ExitCode> {
    let config = config::Config::load(&path).context("failed to load docgraph.toml")?;
    let (blocks, _refs) = collect::collect_workspace_all(&path, &config.graph.ignore, None);
    let regex_str = glob_to_regex(&query);
    let re = regex::Regex::new(&regex_str)
        .with_context(|| format!("Invalid query pattern: '{}'", query))?;

    let mut matched_blocks: Vec<_> = blocks
        .into_iter()
        .filter(|block| re.is_match(&block.id))
        .collect();

    matched_blocks.sort_by(|a, b| a.id.cmp(&b.id));

    for block in matched_blocks {
        println!(
            "{} : {} ({})",
            block.id,
            block.name.as_deref().unwrap_or("No description"),
            block.file_path.display()
        );
    }
    Ok(ExitCode::SUCCESS)
}
