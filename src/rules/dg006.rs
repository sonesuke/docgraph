use crate::config::Config;
use crate::types::{Diagnostic, Range, Severity, SpecBlock};
use std::collections::HashMap;

/// DG006: Strict Relation Enforcement
/// Validates incoming and outgoing edge constraints based on docgraph.toml
pub fn check_strict_relations(blocks: &[SpecBlock], config: &Config) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Map of target_id -> list of source node types that refer to it
    let mut incoming_types: HashMap<String, Vec<String>> = HashMap::new();

    for block in blocks {
        let source_type = block.id.split('-').next().unwrap_or(&block.id).to_string();
        for edge in &block.edges {
            incoming_types
                .entry(edge.id.clone())
                .or_default()
                .push(source_type.clone());
        }
    }

    for block in blocks {
        let prefix = block.id.split('-').next().unwrap_or(&block.id);

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
                                diagnostics.push(Diagnostic {
                                    severity: Severity::Error,
                                    code: "DG006".to_string(),
                                    message: format!(
                                        "Node '{}' (type {}) requires at least {} incoming relation(s) from {:?}, but found {}.",
                                        block.id, prefix, min, rule.targets, count
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
                    "to" => {
                        allowed_outgoing_types.extend(rule.targets.iter().cloned());

                        // Check min count
                        if let Some(min) = rule.min {
                            let count = block
                                .edges
                                .iter()
                                .filter(|e| {
                                    let target_type = e.id.split('-').next().unwrap_or(&e.id);
                                    // Count if it matches one of the allowed types
                                    rule.targets.contains(&target_type.to_string())
                                })
                                .count();

                            if count < min {
                                diagnostics.push(Diagnostic {
                                    severity: Severity::Error,
                                    code: "DG006".to_string(),
                                    message: format!(
                                        "Node '{}' (type {}) requires at least {} outgoing relation(s) to {:?}, but found {}.",
                                        block.id, prefix, min, rule.targets, count
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
                    _ => {}
                }
            }

            // if strict_relations is true, all outgoing edges must be in allowed_outgoing_types
            if config.graph.strict_relations {
                for edge in &block.edges {
                    let target_type = edge.id.split('-').next().unwrap_or(&edge.id);

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
