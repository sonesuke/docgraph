use crate::core::types::{Diagnostic, Range, RuleMetadata, Severity, SpecBlock};

use std::path::PathBuf;

pub fn metadata() -> RuleMetadata {
    RuleMetadata {
        code: "DG001",
        summary: "Anchor must be followed by a heading",
        description: "Each HTML anchor <a id=\"...\"></a> used for node identification must be immediately followed by a Markdown heading on the next line to provide a human-readable title for the node.",
    }
}

pub fn check_anchor_headings(_files: &[PathBuf], blocks: &[SpecBlock]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for block in blocks {
        if block.name.is_none() {
            diagnostics.push(Diagnostic {
                code: "DG001".to_string(),
                message: format!("Anchor '{}' is not followed by a heading", block.id),
                path: block.file_path.clone(),
                range: Range {
                    start_line: block.line_start,
                    start_col: 1,
                    end_line: block.line_start,
                    end_col: 1,
                },
                severity: Severity::Error,
            });
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parse::extract_all;

    #[test]
    fn test_dg001_missing_heading() {
        // Anchor without heading
        let content = r#"<a id="REQ-001"></a>
Some text but no heading."#;
        let path = PathBuf::from("test.md");
        let (blocks, _) = extract_all(content, &path);

        assert_eq!(blocks.len(), 1);
        let diags = check_anchor_headings(std::slice::from_ref(&path), &blocks);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "DG001");
    }

    #[test]
    fn test_dg001_valid() {
        // Anchor with heading
        let content = r#"<a id="REQ-001"></a>
# Heading"#;
        let path = PathBuf::from("test.md");
        let (blocks, _) = extract_all(content, &path);

        assert_eq!(blocks.len(), 1);
        let diags = check_anchor_headings(std::slice::from_ref(&path), &blocks);
        assert_eq!(diags.len(), 0);
    }
}
