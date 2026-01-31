use crate::core::{collect, config};
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_graph(path: PathBuf) -> ExitCode {
    let config = config::Config::load(&path).unwrap_or_default();
    let (blocks, _refs) = collect::collect_workspace_all(&path, &config.graph.ignore);
    let json_out = serde_json::to_string_pretty(&blocks).unwrap();
    println!("{}", json_out);
    ExitCode::SUCCESS
}
