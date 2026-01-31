use super::common::glob_to_regex;
use crate::core::{collect, config};
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_list(query: String, path: PathBuf) -> ExitCode {
    let config = config::Config::load(&path).unwrap_or_default();
    let (blocks, _refs) = collect::collect_workspace_all(&path, &config.graph.ignore);
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
    ExitCode::SUCCESS
}
