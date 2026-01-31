use crate::core::config::Config;
use std::process::ExitCode;

pub fn handle_type(type_id: Option<String>) -> ExitCode {
    let config = match Config::load(std::path::Path::new(".")) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return ExitCode::FAILURE;
        }
    };

    match type_id {
        None => list_all_types(&config),
        Some(id) => show_type_details(&config, &id),
    }
}

fn list_all_types(config: &Config) -> ExitCode {
    if config.node_types.is_empty() {
        println!("No node types defined.");
        return ExitCode::SUCCESS;
    }

    println!("Node Types:");
    println!();

    let mut types: Vec<_> = config.node_types.iter().collect();
    types.sort_by_key(|(k, _)| k.as_str());

    for (id, node_type) in types {
        println!("  {} - {}", id, node_type.desc);
    }

    ExitCode::SUCCESS
}

fn show_type_details(config: &Config, id: &str) -> ExitCode {
    let id_upper = id.to_uppercase();

    let node_type = match config.node_types.get(&id_upper) {
        Some(t) => t,
        None => {
            eprintln!("Unknown type: {}", id);
            return ExitCode::FAILURE;
        }
    };

    println!("Type: {}", id_upper);
    println!("Description: {}", node_type.desc);
    println!();

    if let Some(ref_config) = config.references.get(&id_upper) {
        if !ref_config.rules.is_empty() {
            println!("Rules:");
            for rule in &ref_config.rules {
                let targets = rule.targets.join(", ");
                let min = rule.min.map(|m| m.to_string()).unwrap_or("-".to_string());
                let max = rule.max.map(|m| m.to_string()).unwrap_or("-".to_string());
                let desc = rule.desc.as_deref().unwrap_or("");

                println!(
                    "  {} [{}] min={} max={}: {}",
                    rule.dir, targets, min, max, desc
                );
            }
        }
    } else {
        println!("No rules defined for this type.");
    }

    ExitCode::SUCCESS
}
