use crate::core::types::Diagnostic;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn check_workspace(
    path: &Path,
    _fix: bool, // Fix mode not yet implemented for custom parser
    rule_filter: Option<Vec<String>>,
    _use_docgraph_filter: bool,
    config: &crate::core::config::Config,
    overrides: Option<&HashMap<PathBuf, String>>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let files = crate::core::walk::find_markdown_files(path, &config.graph.ignore);

    // Collect all spec blocks and refs using our custom parser
    let (spec_blocks, refs) =
        crate::core::collect::collect_workspace_all(path, &config.graph.ignore, overrides);

    // Filter rules if provided
    let should_run = |rule_name: &str| -> bool {
        if let Some(ref names) = rule_filter {
            names.iter().any(|n| n == rule_name)
        } else {
            true
        }
    };

    // DG005: Strict Node Types
    if should_run("DG005") {
        let dg005_diags = crate::core::rules::dg005::check_strict_node_types(&spec_blocks, config);
        diagnostics.extend(dg005_diags);
    }

    // DG006: Strict Relations
    if should_run("DG006") {
        let dg006_diags = crate::core::rules::dg006::check_strict_relations(&spec_blocks, config);
        diagnostics.extend(dg006_diags);
    }

    // DG004: Strict Link Text
    if should_run("DG004") {
        let dg004_diags = crate::core::rules::dg004::check_link_text(&files, &spec_blocks);
        diagnostics.extend(dg004_diags);
    }

    // DG001: Anchor followed by Heading
    if should_run("DG001") {
        let dg001_diags = crate::core::rules::dg001::check_anchor_headings(&files, &spec_blocks);
        diagnostics.extend(dg001_diags);
    }

    // DG002: Duplicate IDs
    if should_run("DG002") {
        let dg002_diags = crate::core::rules::dg002::check_duplicate_ids(&files, &spec_blocks);
        diagnostics.extend(dg002_diags);
    }

    // DG003: Broken Links
    if should_run("DG003") {
        let dg003_diags =
            crate::core::rules::dg003::check_broken_links(&files, &spec_blocks, &refs);
        diagnostics.extend(dg003_diags);
    }

    // DG007: Template Validation
    if should_run("DG007") {
        let dg007_diags = crate::core::rules::dg007::check_templates(path, &spec_blocks, config);
        diagnostics.extend(dg007_diags);
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use tempfile::tempdir;

    #[test]
    fn test_check_workspace_dg005() {
        let dir = tempdir().unwrap();
        let f1 = dir.path().join("test.md");
        // Unknown Node Type "FOO"
        let content = r#"<a id="FOO-001"></a>
# Heading
"#;
        std::fs::write(&f1, content).unwrap();

        let config = Config::default();
        let diagnostics = check_workspace(dir.path(), false, None, true, &config, None);

        // Expect DG005 error for unknown node type
        assert!(
            diagnostics
                .iter()
                .any(|d| d.code == "DG005" && d.severity == crate::core::types::Severity::Error)
        );
    }
}
