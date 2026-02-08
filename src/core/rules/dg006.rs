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
            .split(['-', '_'])
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
        let prefix = block.id.split(['-', '_']).next().unwrap_or(&block.id);

        if let Some(node_config) = config.nodes.get(prefix) {
            let mut allowed_outgoing_types = std::collections::HashSet::new();

            for rule in &node_config.rules {
                match rule.dir.as_str() {
                    "from" => {
                        let sources = incoming_types.get(&block.id).cloned().unwrap_or_default();
                        // Check min count
                        if let Some(min) = rule.min {
                            let count = sources
                                .iter()
                                .filter(|t| {
                                    rule.targets.contains(t)
                                        || rule.targets.contains(&"*".to_string())
                                })
                                .count();
                            if count < min {
                                let label = rule.context.as_deref().unwrap_or("be referenced by");
                                let mut message = format!(
                                    "REQUIRED: Node '{}' (type {}) must {} at least {} {}. (Found {})",
                                    block.id,
                                    prefix,
                                    label,
                                    min,
                                    if min > 1 { "nodes" } else { "node" },
                                    count
                                );
                                if let Some(desc) = &rule.desc {
                                    message.push_str(&format!("\nReason: {}", desc));
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

                        // Check max count
                        if let Some(max) = rule.max {
                            let count = sources
                                .iter()
                                .filter(|t| {
                                    rule.targets.contains(t)
                                        || rule.targets.contains(&"*".to_string())
                                })
                                .count();
                            if count > max {
                                let label = rule.context.as_deref().unwrap_or("be referenced by");
                                let mut message = format!(
                                    "LIMIT EXCEEDED: Node '{}' (type {}) can {} at most {} {}. (Found {})",
                                    block.id,
                                    prefix,
                                    label,
                                    max,
                                    if max > 1 { "nodes" } else { "node" },
                                    count
                                );
                                if let Some(desc) = &rule.desc {
                                    message.push_str(&format!("\nReason: {}", desc));
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

                        let count = block
                            .edges
                            .iter()
                            .filter(|e| {
                                let target_type = e.id.split(['-', '_']).next().unwrap_or(&e.id);
                                // Count if it matches one of the allowed types or wildcard
                                rule.targets.contains(&target_type.to_string())
                                    || rule.targets.contains(&"*".to_string())
                            })
                            .count();

                        // Check min count
                        if let Some(min) = rule.min
                            && count < min
                        {
                            let label = rule.context.as_deref().unwrap_or("have relation to");
                            let mut message = format!(
                                "REQUIRED: Node '{}' (type {}) must {} at least {} {}. (Found {})",
                                block.id,
                                prefix,
                                label,
                                min,
                                if min > 1 { "nodes" } else { "node" },
                                count
                            );
                            if let Some(desc) = &rule.desc {
                                message.push_str(&format!("\nReason: {}", desc));
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

                        // Check max count
                        if let Some(max) = rule.max
                            && count > max
                        {
                            let label = rule.context.as_deref().unwrap_or("have relation to");
                            let mut message = format!(
                                "LIMIT EXCEEDED: Node '{}' (type {}) can {} at most {} {}. (Found {})",
                                block.id,
                                prefix,
                                label,
                                max,
                                if max > 1 { "nodes" } else { "node" },
                                count
                            );
                            if let Some(desc) = &rule.desc {
                                message.push_str(&format!("\nReason: {}", desc));
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
                    _ => {}
                }
            }

            // All outgoing edges must be in allowed_outgoing_types
            for edge in &block.edges {
                let target_type = edge.id.split(['-', '_']).next().unwrap_or(&edge.id);

                let source_allows_all = allowed_outgoing_types.contains("*");
                let target_accepts_all = config.nodes.get(target_type).is_some_and(|tc| {
                    tc.rules
                        .iter()
                        .any(|r| r.dir == "from" && r.targets.contains(&"*".to_string()))
                });

                if !allowed_outgoing_types.contains(target_type)
                    && !source_allows_all
                    && !target_accepts_all
                {
                    let mut message = format!(
                        "INVALID RELATION: Node '{}' (type {}) is not allowed to reference '{}' (type {}).",
                        block.id, prefix, edge.id, target_type
                    );

                    // Provide guidance based on allowed rules
                    let mut guidance = Vec::new();
                    for rule in &node_config.rules {
                        if rule.dir == "to" {
                            let label = rule.context.as_deref().unwrap_or("reference");
                            guidance.push(format!("\"{}\" {}", label, rule.targets.join("/")));
                        }
                    }
                    if !guidance.is_empty() {
                        message.push_str(&format!(
                            "\n{} nodes typically: {}.",
                            prefix,
                            guidance.join(", ")
                        ));
                    }

                    // Check for reverse relationship hint
                    if let Some(target_config) = config.nodes.get(target_type) {
                        let mut reverse_allowed = false;
                        let mut reverse_label = "reference";

                        // Check if Target -> Source is allowed via direct rules
                        for target_rule in &target_config.rules {
                            if target_rule.dir == "to"
                                && (target_rule.targets.contains(&prefix.to_string())
                                    || target_rule.targets.contains(&"*".to_string()))
                            {
                                reverse_allowed = true;
                                if let Some(ctx) = &target_rule.context {
                                    reverse_label = ctx;
                                }
                            }
                        }

                        // Check if Source -> Target is allowed via Source's "from" rules (less likely but possible)
                        // Or if Target -> Source is allowed via Source's "from" rules
                        for source_rule in &node_config.rules {
                            if source_rule.dir == "from"
                                && (source_rule.targets.contains(&target_type.to_string())
                                    || source_rule.targets.contains(&"*".to_string()))
                            {
                                reverse_allowed = true;
                                if let Some(ctx) = &source_rule.context {
                                    reverse_label = ctx;
                                }
                            }
                        }

                        if reverse_allowed {
                            message.push_str(&format!(
                                "\nHINT: The reverse relationship '{} {} {}' is permitted. Did you mean to link FROM the {} to this {}?",
                                target_type, reverse_label, prefix, target_type, prefix
                            ));
                        }
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
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{Config, NodeConfig, RuleConfig};
    use crate::core::types::EdgeUse;
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
            content: String::new(),
            node_type: id.split(['-', '_']).next().unwrap_or(id).to_string(),
        }
    }

    #[test]
    fn test_dg006_strict_relations_invalid() {
        let mut config = Config::default();

        let mut node_config = NodeConfig::default();
        node_config.rules.push(RuleConfig {
            dir: "to".to_string(),
            targets: vec!["SYS".to_string()],
            min: None,
            max: None,
            desc: None,
            context: None,
        });

        config.nodes.insert("REQ".to_string(), node_config);

        // REQ -> UNK (Not allowed)
        let blocks = vec![create_block("REQ-01", vec!["UNK-01"])];
        let diags = check_strict_relations(&blocks, &config);

        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("INVALID RELATION"));
        assert!(diags[0].message.contains("is not allowed to reference"));
    }

    #[test]
    fn test_dg006_strict_relations_valid() {
        let mut config = Config::default();

        let mut node_config = NodeConfig::default();
        node_config.rules.push(RuleConfig {
            dir: "to".to_string(),
            targets: vec!["SYS".to_string()],
            min: None,
            max: None,
            desc: None,
            context: None,
        });

        config.nodes.insert("REQ".to_string(), node_config);

        // REQ -> SYS (Allowed)
        let blocks = vec![create_block("REQ-01", vec!["SYS-01"])];
        let diags = check_strict_relations(&blocks, &config);

        assert!(diags.is_empty());
    }

    #[test]
    fn test_dg006_min_incoming() {
        let mut config = Config::default();

        let mut node_config = NodeConfig::default();
        node_config.rules.push(RuleConfig {
            dir: "from".to_string(),
            targets: vec!["REQ".to_string()],
            min: Some(1),
            max: None,
            desc: None,
            context: None,
        });

        // SYS nodes must have at least 1 incoming from REQ
        config.nodes.insert("SYS".to_string(), node_config);

        // Case 1: SYS has 0 incoming (Error)
        let blocks_fail = vec![create_block("SYS-01", vec![])]; // No REQ points to it
        let diags_fail = check_strict_relations(&blocks_fail, &config);
        assert_eq!(diags_fail.len(), 1);
        assert!(diags_fail[0].message.contains(
            "REQUIRED: Node 'SYS-01' (type SYS) must be referenced by at least 1 node. (Found 0)"
        ));

        // Case 2: SYS has 1 incoming (Ok)
        let blocks_ok = vec![
            create_block("SYS-01", vec![]),
            create_block("REQ-01", vec!["SYS-01"]),
        ];
        let diags_ok = check_strict_relations(&blocks_ok, &config);
        assert!(diags_ok.is_empty());
    }

    #[test]
    fn test_dg006_max_incoming() {
        let mut config = Config::default();

        let mut node_config = NodeConfig::default();
        node_config.rules.push(RuleConfig {
            dir: "from".to_string(),
            targets: vec!["REQ".to_string()],
            min: None,
            max: Some(1),
            desc: None,
            context: None,
        });

        config.nodes.insert("SYS".to_string(), node_config);

        // Case 1: SYS has 2 incoming from REQ (Error)
        // We need 2 separate blocks pointing to SYS
        let blocks_fail = vec![
            create_block("SYS-01", vec![]),
            create_block("REQ-01", vec!["SYS-01"]),
            create_block("REQ-02", vec!["SYS-01"]),
        ];
        let diags_fail = check_strict_relations(&blocks_fail, &config);
        assert_eq!(diags_fail.len(), 1);
        assert!(
            diags_fail[0]
                .message
                .contains("LIMIT EXCEEDED: Node 'SYS-01' (type SYS) can be referenced by at most 1 node. (Found 2)")
        );
    }

    #[test]
    fn test_dg006_invalid_direction_and_desc() {
        let mut config = Config::default();
        let mut node_config = NodeConfig::default();

        // Rule with invalid direction and a description
        node_config.rules.push(RuleConfig {
            dir: "invalid".to_string(),
            targets: vec!["SYS".to_string()],
            min: Some(1),
            max: None,
            desc: Some("Must link to system".to_string()),
            context: None,
        });

        // Rule with valid direction and description, testing description in output
        node_config.rules.push(RuleConfig {
            dir: "to".to_string(),
            targets: vec!["SYS".to_string()],
            min: Some(1),
            max: None,
            desc: Some("Important Business Rule".to_string()),
            context: None,
        });

        config.nodes.insert("REQ".to_string(), node_config);

        // REQ -> [] (Fail 'to' rule)
        let blocks = vec![create_block("REQ-01", vec![])];
        let diags = check_strict_relations(&blocks, &config);

        // Check "invalid" direction is ignored (no panic, no extra error)
        // Check "to" rule failed and included description
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains(
            "REQUIRED: Node 'REQ-01' (type REQ) must have relation to at least 1 node. (Found 0)"
        ));
        assert!(diags[0].message.contains("Reason: Important Business Rule"));
    }
}
