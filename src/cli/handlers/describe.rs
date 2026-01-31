use crate::core::{collect, config};
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_describe(id: String, path: PathBuf) -> ExitCode {
    let config = config::Config::load(&path).unwrap_or_default();
    let (blocks, _refs) = collect::collect_workspace_all(&path, &config.graph.ignore);

    let target_block = blocks.iter().find(|b| b.id == id);

    match target_block {
        Some(block) => {
            println!(
                "{}: {}",
                id,
                block.name.as_deref().unwrap_or("No description")
            );
            println!("\n{} references to", id);
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
    ExitCode::SUCCESS
}
