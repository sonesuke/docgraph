use crate::core::config::Config;
use crate::core::types::{Diagnostic, Range, Severity, SpecBlock};
use std::collections::HashMap;

/// DG006: Strict Relation Enforcement
/// Validates incoming and outgoing edge constraints based on docgraph.toml
pub fn check_strict_relations(blocks: &[SpecBlock], config: &Config) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Map of target_id -> list of source node types that refer to it
    let mut incoming_types: HashMap<String, Vec<String>> = HashMap::new();

    for block in blocks {
        let source_type = block
            .id
            .split(|c| c == '-' || c == '_')
            .next()
            .unwrap_or(&block.id)
            .to_string();
        for edge in &block.edges {
            incoming_types
                .entry(edge.id.clone())
                .or_default()
                .push(source_type.clone());
        }
    }

    for block in blocks {
        let prefix = block
            .id
            .split(|c| c == '-' || c == '_')
            .next()
            .unwrap_or(&block.id);

        if let Some(rel_config) = config.references.get(prefix) {
            let mut allowed_outgoing_types = std::collections::HashSet::new();

            for rule in &rel_config.rules {
                match rule.dir.as_str() {
                    "from" => {
                        let sources = incoming_types.get(&block.id).cloned().unwrap_or_default();
                        // Check min count
                        if let Some(min) = rule.min {
                            let count = sources.iter().filter(|t| rule.targets.contains(t)).count();
                            if count < min {
                                let mut message = format!(
                                    "Node '{}' (type {}) requires at least {} incoming relation(s) from {:?}, but found {}.",
                                    block.id, prefix, min, rule.targets, count
                                );
                                if let Some(desc) = &rule.desc {
                                    message.push_str(&format!(" (Description: {})", desc));
                                }
                                diagnostics.push(Diagnostic {
                                    severity: Severity::Error,
                                    code: "DG006".to_string(),
                                    message,
                                    path: block.file_path.clone(),
                                    range: Range {
                                        start_line: block.line_start,
                                        start_col: 1,
                                        end_line: block.line_start,
                                        end_col: 1,
                                    },
                                });
                            }
                        }
                    }
                    "to" => {
                        allowed_outgoing_types.extend(rule.targets.iter().cloned());

                        // Check min count
                        if let Some(min) = rule.min {
                            let count = block
                                .edges
                                .iter()
                                .filter(|e| {
                                    let target_type =
                                        e.id.split(|c| c == '-' || c == '_')
                                            .next()
                                            .unwrap_or(&e.id);
                                    // Count if it matches one of the allowed types
                                    rule.targets.contains(&target_type.to_string())
                                })
                                .count();

                            if count < min {
                                let mut message = format!(
                                    "Node '{}' (type {}) requires at least {} outgoing relation(s) to {:?}, but found {}.",
                                    block.id, prefix, min, rule.targets, count
                                );
                                if let Some(desc) = &rule.desc {
                                    message.push_str(&format!(" (Description: {})", desc));
                                }
                                diagnostics.push(Diagnostic {
                                    severity: Severity::Error,
                                    code: "DG006".to_string(),
                                    message,
                                    path: block.file_path.clone(),
                                    range: Range {
                                        start_line: block.line_start,
                                        start_col: 1,
                                        end_line: block.line_start,
                                        end_col: 1,
                                    },
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }

            // if strict_relations is true, all outgoing edges must be in allowed_outgoing_types
            if config.graph.strict_relations {
                for edge in &block.edges {
                    let target_type = edge
                        .id
                        .split(|c| c == '-' || c == '_')
                        .next()
                        .unwrap_or(&edge.id);

                    // Always allow documentation types
                    if config.graph.doc_types.contains(&target_type.to_string()) {
                        continue;
                    }

                    if !allowed_outgoing_types.contains(target_type) {
                        diagnostics.push(Diagnostic {
                            severity: Severity::Error,
                            code: "DG006".to_string(),
                            message: format!(
                                "Node '{}' (type {}) is not allowed to reference '{}' (type {}). Allowed target types: {:?}.",
                                block.id, prefix, edge.id, target_type, allowed_outgoing_types
                            ),
                            path: block.file_path.clone(),
                            range: Range {
                                start_line: block.line_start,
                                start_col: 1,
                                end_line: block.line_start,
                                end_col: 1,
                            },
                        });
                    }
                }
            }
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{Config, ReferenceConfig, RuleConfig};
    use crate::core::types::{EdgeUse, SpecBlock};
    use std::path::PathBuf;

    fn create_block(id: &str, edges: Vec<&str>) -> SpecBlock {
        SpecBlock {
            id: id.to_string(),
            name: Some("Test".to_string()),
            file_path: PathBuf::from("test.md"),
            line_start: 1,
            line_end: 1,
            edges: edges
                .into_iter()
                .map(|e| EdgeUse {
                    id: e.to_string(),
                    name: None,
                    line: 1,
                    col_start: 1,
                    col_end: 1,
                })
                .collect(),
        }
    }

    #[test]
    fn test_dg006_strict_relations_invalid() {
        let mut config = Config::default();
        config.graph.strict_relations = true;

        let mut ref_config = ReferenceConfig::default();
        ref_config.rules.push(RuleConfig {
            dir: "to".to_string(),
            targets: vec!["SYS".to_string()],
            min: None,
            max: None,
            desc: None,
        });

        config.references.insert("REQ".to_string(), ref_config);

        // REQ -> UNK (Not allowed)
        let blocks = vec![create_block("REQ-01", vec!["UNK-01"])];
        let diags = check_strict_relations(&blocks, &config);

        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("not allowed to reference"));
    }

    #[test]
    fn test_dg006_strict_relations_valid() {
        let mut config = Config::default();
        config.graph.strict_relations = true;

        let mut ref_config = ReferenceConfig::default();
        ref_config.rules.push(RuleConfig {
            dir: "to".to_string(),
            targets: vec!["SYS".to_string()],
            min: None,
            max: None,
            desc: None,
        });

        config.references.insert("REQ".to_string(), ref_config);

        // REQ -> SYS (Allowed)
        let blocks = vec![create_block("REQ-01", vec!["SYS-01"])];
        let diags = check_strict_relations(&blocks, &config);

        assert!(diags.is_empty());
    }

    #[test]
    fn test_dg006_min_incoming() {
        let mut config = Config::default();
        config.graph.strict_relations = false;

        let mut ref_config = ReferenceConfig::default();
        ref_config.rules.push(RuleConfig {
            dir: "from".to_string(),
            targets: vec!["REQ".to_string()],
            min: Some(1),
            max: None,
            desc: None,
        });

        // SYS nodes must have at least 1 incoming from REQ
        config.references.insert("SYS".to_string(), ref_config);

        // Case 1: SYS has 0 incoming (Error)
        let blocks_fail = vec![create_block("SYS-01", vec![])]; // No REQ points to it
        let diags_fail = check_strict_relations(&blocks_fail, &config);
        assert_eq!(diags_fail.len(), 1);
        assert!(
            diags_fail[0]
                .message
                .contains("requires at least 1 incoming relation")
        );

        // Case 2: SYS has 1 incoming (Ok)
        let blocks_ok = vec![
            create_block("SYS-01", vec![]),
            create_block("REQ-01", vec!["SYS-01"]),
        ];
        let diags_ok = check_strict_relations(&blocks_ok, &config);
        assert!(diags_ok.is_empty());
    }

    #[test]
    fn test_dg006_invalid_direction_and_desc() {
        let mut config = Config::default();
        let mut ref_config = ReferenceConfig::default();

        // Rule with invalid direction and a description
        ref_config.rules.push(RuleConfig {
            dir: "invalid".to_string(),
            targets: vec!["SYS".to_string()],
            min: Some(1),
            max: None,
            desc: Some("Must link to system".to_string()),
        });

        // Rule with valid direction and description, testing description in output
        ref_config.rules.push(RuleConfig {
            dir: "to".to_string(),
            targets: vec!["SYS".to_string()],
            min: Some(1),
            max: None,
            desc: Some("Important Business Rule".to_string()),
        });

        config.references.insert("REQ".to_string(), ref_config);

        // REQ -> [] (Fail 'to' rule)
        let blocks = vec![create_block("REQ-01", vec![])];
        let diags = check_strict_relations(&blocks, &config);

        // Check "invalid" direction is ignored (no panic, no extra error)
        // Check "to" rule failed and included description
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("Important Business Rule"));
    }
}
