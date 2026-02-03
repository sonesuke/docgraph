use crate::core::{collect, config};
use anyhow::Context;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_graph(path: PathBuf) -> ExitCode {
    match try_graph(path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn try_graph(path: PathBuf) -> anyhow::Result<ExitCode> {
    let config = config::Config::load(&path).context("failed to load docgraph.toml")?;
    let (blocks, _refs) = collect::collect_workspace_all(&path, &config.graph.ignore, None);
    let json_out =
        serde_json::to_string_pretty(&blocks).context("failed to serialize graph to JSON")?;
    println!("{}", json_out);
    Ok(ExitCode::SUCCESS)
}
