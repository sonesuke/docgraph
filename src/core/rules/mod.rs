pub mod dg001;
pub mod dg002;
pub mod dg003;
pub mod dg004;
pub mod dg005;
pub mod dg006;
pub mod dg007;

use crate::core::types::RuleMetadata;

pub fn get_all_rules() -> Vec<RuleMetadata> {
    vec![
        dg001::metadata(),
        dg002::metadata(),
        dg003::metadata(),
        dg004::metadata(),
        dg005::metadata(),
        dg006::metadata(),
        dg007::metadata(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn all_implemented_rules_have_metadata_registered() {
        let rules_dir = Path::new("src/core/rules");
        let entries = fs::read_dir(rules_dir).expect("Failed to read rules directory");

        let registered_rules = get_all_rules();

        for entry in entries {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();

            if path.is_file()
                && let Some(file_name) = path.file_name().and_then(|n| n.to_str())
                && file_name.starts_with("dg")
                && file_name.ends_with(".rs")
            {
                let rule_id = file_name.trim_end_matches(".rs").to_ascii_uppercase();

                // Check if this rule is in our central registry
                assert!(
                    registered_rules.iter().any(|r| r.code == rule_id),
                    "Rule {} is implemented in {} but not registered in get_all_rules()",
                    rule_id,
                    file_name
                );
            }
        }
    }
}
