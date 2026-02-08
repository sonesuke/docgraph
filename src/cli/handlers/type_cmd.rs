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
    if config.nodes.is_empty() {
        println!("No node types defined.");
        return ExitCode::SUCCESS;
    }

    println!("Node Types:");
    println!();

    let mut types: Vec<_> = config.nodes.iter().collect();
    types.sort_by_key(|(k, _)| k.as_str());

    for (id, node_config) in types {
        println!("  {} - {}", id, node_config.desc);
    }

    ExitCode::SUCCESS
}

fn show_type_details(config: &Config, id: &str) -> ExitCode {
    let id_upper = id.to_uppercase();

    let node_config = match config.nodes.get(&id_upper) {
        Some(t) => t,
        None => {
            eprintln!("Unknown type: {}", id);
            return ExitCode::FAILURE;
        }
    };

    println!("Type: {}", id_upper);
    println!("Description: {}", node_config.desc);
    if let Some(template) = &node_config.template {
        println!("Template: {}", template.display());
        if let Ok(content) = std::fs::read_to_string(template) {
            println!("\n---\n{}\n---", content.trim());
        }
    }
    println!();

    if !node_config.rules.is_empty() {
        println!("Rules:");
        for rule in &node_config.rules {
            let targets = rule.targets.join(", ");
            let min = rule
                .min
                .map(|m: usize| m.to_string())
                .unwrap_or("-".to_string());

            let max_str = rule
                .max
                .map(|m: usize| format!(" max={}", m))
                .unwrap_or_default();

            let desc = match &rule.context {
                Some(ctx) => format!("(Context: {}) {}", ctx, rule.desc.as_deref().unwrap_or("")),
                None => rule.desc.as_deref().unwrap_or("").to_string(),
            };

            println!(
                "  {} [{}] min={}{}: {}",
                rule.dir, targets, min, max_str, desc
            );
        }
    } else {
        println!("No rules defined for this type.");
    }

    ExitCode::SUCCESS
}
