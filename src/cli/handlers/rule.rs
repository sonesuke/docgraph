use crate::core::rules;
use std::process::ExitCode;

pub fn handle_rule(rule: Option<String>) -> ExitCode {
    let rumdl_config = rumdl_lib::config::Config::default();
    let mut all_rules = rumdl_lib::rules::all_rules(&rumdl_config);
    all_rules.push(Box::new(rules::dg001::DG001));
    all_rules.push(Box::new(rules::dg002::DG002));
    all_rules.push(Box::new(rules::dg003::DG003));
    all_rules.push(Box::new(rules::dg004::DG004));
    all_rules.push(Box::new(rules::dg004::DG004));

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
            eprintln!("Rule '{}' not found.", rule_query);
            return ExitCode::FAILURE;
        }
    } else {
        println!("Available rules:");
        for rule in &all_rules {
            println!("  {} - {}", rule.name(), rule.description());
        }
    }
    ExitCode::SUCCESS
}
