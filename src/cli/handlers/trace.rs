use super::common::glob_to_regex;
use crate::core::{collect, config};
use anyhow::Context;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_trace(from: String, to: String, path: PathBuf, direction: String) -> ExitCode {
    match try_trace(from, to, path, direction) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn try_trace(
    from: String,
    to: String,
    path: PathBuf,
    direction: String,
) -> anyhow::Result<ExitCode> {
    let config = config::Config::load(&path).context("failed to load docgraph.toml")?;
    let (blocks, _refs) = collect::collect_workspace_all(&path, &config.graph.ignore);
    let target_regex_str = glob_to_regex(&to);
    let target_re = regex::Regex::new(&target_regex_str)
        .with_context(|| format!("Invalid target pattern: '{}'", to))?;

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
        anyhow::bail!("Invalid direction '{}'. Use 'down' or 'up'", direction);
    }

    if !blocks.iter().any(|b| b.id == from) && direction == "down" {
        anyhow::bail!("Start ID '{}' not found", from);
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
    Ok(ExitCode::SUCCESS)
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
