use crate::core::config::Config;
use crate::core::types::{Diagnostic, Range, RuleMetadata, Severity, SpecBlock};

/// DG005: Strict Node Type Enforcement
/// Validates that all documented IDs start with a registered prefix from docgraph.toml
pub fn metadata() -> RuleMetadata {
    RuleMetadata {
        code: "DG005",
        summary: "Enforce strict node types defined in docgraph.toml",
    }
}

pub fn check_node_types(config: &Config, blocks: &[SpecBlock]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for block in blocks {
        let s: &str = block.id.as_str();
        let prefix = s.split(['-', '_']).next().unwrap_or(s);

        if !config.nodes.contains_key(prefix) {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "DG005".to_string(),
                message: format!(
                    "Unknown node type prefix '{}' in ID '{}'. Must be one of: {:?}.",
                    prefix,
                    block.id,
                    config.nodes.keys().collect::<Vec<_>>()
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
    use std::path::PathBuf;

    fn create_block(id: &str) -> SpecBlock {
        SpecBlock {
            id: id.to_string(),
            name: Some("Test Block".to_string()),
            file_path: PathBuf::from("test.md"),
            line_start: 1,
            line_end: 1,
            edges: vec![],
            content: String::new(),
            node_type: id.split(['-', '_']).next().unwrap_or(id).to_string(),
        }
    }

    #[test]
    fn test_dg005_unknown_prefix() {
        let mut config = Config::default();
        config.nodes.insert("REQ".to_string(), Default::default());

        let blocks = vec![create_block("UNK-01")];
        let diags = check_node_types(&config, &blocks);

        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("Unknown node type prefix"));
    }

    #[test]
    fn test_dg005_known_prefix() {
        let mut config = Config::default();
        config.nodes.insert("REQ".to_string(), Default::default());

        let blocks = vec![create_block("REQ-01")];
        let diags = check_node_types(&config, &blocks);

        assert!(diags.is_empty());
    }
}
