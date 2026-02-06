use crate::core::types::{Diagnostic, Range, Severity, SpecBlock};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn check_duplicate_ids(_files: &[PathBuf], blocks: &[SpecBlock]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut id_map: HashMap<String, Vec<&SpecBlock>> = HashMap::new();

    // Group blocks by ID
    for block in blocks {
        id_map.entry(block.id.clone()).or_default().push(block);
    }

    // Check for duplicates
    for (id, occurrences) in id_map {
        if occurrences.len() > 1 {
            for block in occurrences {
                diagnostics.push(Diagnostic {
                    code: "DG002".to_string(),
                    message: format!("Duplicate anchor ID '{}' found", id),
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
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parse::extract_all;

    #[test]
    fn test_dg002_duplicates() {
        let content = r#"<a id="DUP"></a>
<a id="DUP"></a>"#;
        let path = PathBuf::from("test.md");
        let (blocks, _) = extract_all(content, &path);

        assert_eq!(blocks.len(), 2);
        let diags = check_duplicate_ids(std::slice::from_ref(&path), &blocks);
        assert_eq!(diags.len(), 2); // Both occurrences reported
        assert_eq!(diags[0].code, "DG002");
    }

    #[test]
    fn test_dg002_unique() {
        let content = r#"<a id="ID-1"></a>
<a id="ID-2"></a>"#;
        let path = PathBuf::from("test.md");
        let (blocks, _) = extract_all(content, &path);

        assert_eq!(blocks.len(), 2);
        let diags = check_duplicate_ids(std::slice::from_ref(&path), &blocks);
        assert_eq!(diags.len(), 0);
    }
}
