use std::process::ExitCode;

pub fn handle_rule(rule: Option<String>) -> ExitCode {
    match try_rule(rule) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn try_rule(rule: Option<String>) -> anyhow::Result<ExitCode> {
    // Get list of active rules from core
    let active_rules = crate::core::rules::get_all_rules();

    if let Some(rule_query) = rule {
        let rule_query = rule_query.to_ascii_uppercase();
        let found = active_rules
            .iter()
            .find(|r| r.code.eq_ignore_ascii_case(&rule_query));

        if let Some(meta) = found {
            println!(
                "{} - {}\n\nDescription:\n  {}",
                meta.code, meta.summary, meta.description
            );
        } else {
            anyhow::bail!("Rule '{}' not found", rule_query);
        }
    } else {
        println!("Available rules:");
        for meta in &active_rules {
            println!("  {} - {}", meta.code, meta.summary);
        }
    }
    Ok(ExitCode::SUCCESS)
}
