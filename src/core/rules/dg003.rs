use crate::core::types::{Diagnostic, Range, RefUse, Severity, SpecBlock};
use std::collections::HashSet;
use std::path::PathBuf;

pub fn check_broken_links(
    _files: &[PathBuf],
    blocks: &[SpecBlock],
    standalone_refs: &[RefUse],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let known_ids: HashSet<String> = blocks.iter().map(|b| b.id.clone()).collect();

    // Check refs inside blocks
    for block in blocks {
        for edge in &block.edges {
            if !known_ids.contains(&edge.id) {
                diagnostics.push(Diagnostic {
                    code: "DG003".to_string(),
                    message: format!("Link to unknown ID '{}'", edge.id),
                    path: block.file_path.clone(),
                    range: Range {
                        start_line: edge.line,
                        start_col: edge.col_start,
                        end_line: edge.line,
                        end_col: edge.col_end,
                    },
                    severity: Severity::Error,
                });
            }
        }
    }

    // Check standalone refs
    for rf in standalone_refs {
        if !known_ids.contains(&rf.target_id) {
            diagnostics.push(Diagnostic {
                code: "DG003".to_string(),
                message: format!("Link to unknown ID '{}'", rf.target_id),
                path: rf.file_path.clone(),
                range: Range {
                    start_line: rf.line,
                    start_col: rf.col_start,
                    end_line: rf.line,
                    end_col: rf.col_end,
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
    fn test_dg003_broken_links() {
        let content = r#"<a id="ID-1"></a>
# Heading
[Good](#ID-1)
[Bad](#UNKNOWN)
"#;
        let path = PathBuf::from("test.md");
        let (blocks, _) = extract_all(content, &path);

        assert_eq!(blocks.len(), 1);
        let diags = check_broken_links(std::slice::from_ref(&path), &blocks, &[]);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("UNKNOWN"));
    }

    #[test]
    fn test_dg003_standalone_broken() {
        let content = r#"
[Bad](#UNKNOWN)
"#;
        let path = PathBuf::from("test.md");
        let (blocks, refs) = extract_all(content, &path);

        assert_eq!(blocks.len(), 0);
        let diags = check_broken_links(std::slice::from_ref(&path), &blocks, &refs);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("UNKNOWN"));
    }
}
