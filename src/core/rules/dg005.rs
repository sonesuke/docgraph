use crate::core::config::Config;
use crate::core::types::{Diagnostic, Range, Severity, SpecBlock};

/// DG005: Strict Node Type Enforcement
/// Validates that all documented IDs start with a registered prefix from docgraph.toml
pub fn check_strict_node_types(blocks: &[SpecBlock], config: &Config) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if !config.graph.strict_node_types {
        return diagnostics;
    }

    for block in blocks {
        let s: &str = block.id.as_str();
        let prefix = s.split(|c| c == '-' || c == '_').next().unwrap_or(s);

        if !config.node_types.contains_key(prefix) {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "DG005".to_string(),
                message: format!(
                    "Unknown node type prefix '{}' in ID '{}'. Must be one of: {:?}.",
                    prefix,
                    block.id,
                    config.node_types.keys().collect::<Vec<_>>()
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

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use crate::core::types::SpecBlock;
    use std::path::PathBuf;

    fn create_block(id: &str) -> SpecBlock {
        SpecBlock {
            id: id.to_string(),
            name: Some("Test Block".to_string()),
            file_path: PathBuf::from("test.md"),
            line_start: 1,
            line_end: 1,
            edges: vec![],
        }
    }

    #[test]
    fn test_dg005_unknown_prefix() {
        let mut config = Config::default();
        config.graph.strict_node_types = true;
        config
            .node_types
            .insert("REQ".to_string(), Default::default());

        let blocks = vec![create_block("UNK-01")];
        let diags = check_strict_node_types(&blocks, &config);

        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("Unknown node type prefix"));
    }

    #[test]
    fn test_dg005_known_prefix() {
        let mut config = Config::default();
        config.graph.strict_node_types = true;
        config
            .node_types
            .insert("REQ".to_string(), Default::default());

        let blocks = vec![create_block("REQ-01")];
        let diags = check_strict_node_types(&blocks, &config);

        assert!(diags.is_empty());
    }

    #[test]
    fn test_dg005_disabled() {
        let mut config = Config::default();
        config.graph.strict_node_types = false; // default

        let blocks = vec![create_block("UNK-01")];
        let diags = check_strict_node_types(&blocks, &config);

        assert!(diags.is_empty());
    }
}
