use crate::core::rules;
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
    let rumdl_config = rumdl_lib::config::Config::default();
    let mut all_rules = rumdl_lib::rules::all_rules(&rumdl_config);
    all_rules.push(Box::new(rules::dg001::DG001));
    all_rules.push(Box::new(rules::dg002::DG002));
    all_rules.push(Box::new(rules::dg003::DG003));
    // TODO: Add DG005, DG006, DG004 when they implement the Rule trait or support custom listing

    if let Some(rule_query) = rule {
        let rule_query = rule_query.to_ascii_uppercase();
        let found = all_rules.iter().find(|r| {
            r.name().eq_ignore_ascii_case(&rule_query)
                || r.name().replace("MD", "") == rule_query.replace("MD", "")
        });

        if let Some(rule) = found {
            println!(
                "{} - {}\n\nDescription:\n  {}",
                rule.name(),
                rule.description(),
                rule.description()
            );
        } else {
            anyhow::bail!("Rule '{}' not found", rule_query);
        }
    } else {
        println!("Available rules:");
        for rule in &all_rules {
            println!("  {} - {}", rule.name(), rule.description());
        }
    }
    Ok(ExitCode::SUCCESS)
}
