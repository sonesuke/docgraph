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
    // List of active custom rules
    let active_rules = vec![
        ("DG001", "Anchor must be followed by a heading"),
        ("DG002", "No duplicate anchor IDs allowed"),
        ("DG003", "Links must point to valid anchor IDs"),
        (
            "DG004",
            "Link text must match target title (or expected format)",
        ),
        (
            "DG005",
            "Enforce strict node types defined in docgraph.toml",
        ),
        (
            "DG006",
            "Enforce strict relationships (allowed_dependencies/derived_from)",
        ),
        ("DG007", "Enforce template adherence for node types"),
    ];

    if let Some(rule_query) = rule {
        let rule_query = rule_query.to_ascii_uppercase();
        let found = active_rules
            .iter()
            .find(|(name, _desc)| name.eq_ignore_ascii_case(&rule_query));

        if let Some((name, desc)) = found {
            println!("{} - {}\n\nDescription:\n  {}", name, desc, desc);
        } else {
            anyhow::bail!("Rule '{}' not found", rule_query);
        }
    } else {
        println!("Available rules:");
        for (name, desc) in &active_rules {
            println!("  {} - {}", name, desc);
        }
    }
    Ok(ExitCode::SUCCESS)
}
