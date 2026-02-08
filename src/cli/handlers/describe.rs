use crate::core::{collect, config};
use anyhow::Context;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_describe(id: String, path: PathBuf) -> ExitCode {
    match try_describe(id, path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn try_describe(id: String, path: PathBuf) -> anyhow::Result<ExitCode> {
    let config = config::Config::load(&path).context("failed to load docgraph.toml")?;
    let (blocks, _refs) = collect::collect_workspace_all(&path, &config.graph.ignore, None);

    let target_block = blocks
        .iter()
        .find(|b| b.id == id)
        .ok_or_else(|| anyhow::anyhow!("ID '{}' not found", id))?;

    println!(
        "{}: {}",
        id,
        target_block.name.as_deref().unwrap_or("No description")
    );
    println!("\n---\n{}\n---", target_block.content.trim());
    println!("\n{} references to", id);
    for edge in &target_block.edges {
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

    Ok(ExitCode::SUCCESS)
}
