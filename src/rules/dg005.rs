use crate::config::Config;
use crate::types::{Diagnostic, Range, Severity, SpecBlock};

/// DG005: Strict Node Type Enforcement
/// Validates that all documented IDs start with a registered prefix from docgraph.toml
pub fn check_strict_node_types(
    blocks: &[SpecBlock],
    config: &Config,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if !config.graph.strict_node_types {
        return diagnostics;
    }

    for block in blocks {
        let s: &str = block.id.as_str();
        let parts: Vec<&str> = s.split('-').collect();
        let prefix = parts.first().copied().unwrap_or(s);
        
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
