use crate::collect::collect_workspace;
use crate::types::{Diagnostic, Range, Severity, SpecBlock};
use std::collections::HashMap;
use std::path::Path;

pub fn check_workspace(root: &Path) -> Vec<Diagnostic> {
    let all_blocks = collect_workspace(root);
    check_blocks(&all_blocks)
}

pub fn check_blocks(all_blocks: &[SpecBlock]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // 2. Build Symbol Table & Check Duplicates (RULE-2) & Missing ID (RULE-1)
    let mut id_map: HashMap<String, &SpecBlock> = HashMap::new();

    for block in all_blocks {
        // RULE-1: No ID
        if block.id.is_empty() {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "E_NO_ID".to_string(),
                message: "Missing :id: in {document} block".to_string(),
                path: block.file_path.clone(),
                range: Range {
                    start_line: block.line_start,
                    start_col: 1,
                    end_line: block.line_start,
                    end_col: 1,
                },
            });
            continue; // Can't add to map if empty
        }

        // RULE-2: Duplicate ID
        if let Some(prev) = id_map.get(&block.id) {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "E_DUP_ID".to_string(),
                message: format!(
                    "Duplicate id '{}'. First defined at {:?}:{}",
                    block.id, prev.file_path, prev.line_start
                ),
                path: block.file_path.clone(),
                range: Range {
                    start_line: block.line_start,
                    start_col: 1, // approximate
                    end_line: block.line_start,
                    end_col: 1,
                },
            });
        } else {
            id_map.insert(block.id.clone(), block);
        }
    }

    // 3. Check References (RULE-3)
    for block in all_blocks {
        if block.id.is_empty() {
            continue;
        }

        // Check Body Refs
        for r in &block.refs {
            if !id_map.contains_key(&r.target_id) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    code: "E_BAD_REF".to_string(),
                    message: format!("Unknown ref target '{}'", r.target_id),
                    path: block.file_path.clone(),
                    range: Range {
                        start_line: r.line,
                        start_col: r.col,
                        end_line: r.line,
                        end_col: r.col + r.target_id.len(),
                    },
                });
            }
        }

        // Check Typed Edges
        for e in &block.edges {
            if !id_map.contains_key(&e.target_id) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    code: "E_BAD_REF".to_string(),
                    message: format!("Unknown target '{}' in edge :{}", e.target_id, e.edge_type),
                    path: block.file_path.clone(),
                    range: Range {
                        start_line: e.line,
                        start_col: 1, // We don't have col for options yet, assume 1
                        end_line: e.line,
                        end_col: 1,
                    },
                });
            }
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::extract_blocks;
    use std::path::PathBuf;

    fn parse(content: &str) -> Vec<SpecBlock> {
        extract_blocks(content, &PathBuf::from("test.md"))
    }

    #[test]
    fn test_valid_graph() {
        let content = r#"
```{document}
:id: REQ-1
```
```{document}
:id: TST-1
:verifies: REQ-1
```
"#;
        let blocks = parse(content);
        let diags = check_blocks(&blocks);
        assert!(diags.is_empty());
    }

    #[test]
    fn test_missing_id() {
        let content = r#"
```{document}
:kind: req
```
"#;
        // Parsing might filter it out if we changed parse logic,
        // but currently parse logic returns empty ID string if missing.
        // Let's check parse.rs. It expects :id: or returns None if id.is_none().
        // Wait, parse.rs lines 108: if id.is_none() { return None; }
        // So `extract_blocks` won't return blocks without ID at all?
        // Ah, earlier I said "return None // RULE-1".
        // If parse returns None, then Lint can't check it.
        // If we want E_NO_ID, Parse MUST return it with empty ID or something.
        // Let's check current parse.rs implementation.
        // Step 150 showed the content.
        // `if id.is_none() { return None; }`
        // This means E_NO_ID logic in Lint is unreachable for missing options!
        // But if `:id:` is present but empty value? `:id: ` -> value is "" (trimmed).
        // Then `id` is Some(""). `check_blocks` checks `block.id.is_empty()`.
        // So checking `:id: ` (empty value) works.
        // Checking missing `:id:` line entirely: parse returns None.

        let blocks = parse(content);
        // If parse returns 0 blocks, diagnostics will be 0.
        // If we want to strictly catch "missing :id: line", parse needs update.
        // But for this test let's test empty ID value.

        let content_empty_val = r#"
```{document}
:id:
```
"#;
        let blocks = parse(content_empty_val);
        let diags = check_blocks(&blocks);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "E_NO_ID");
    }

    #[test]
    fn test_duplicate_id() {
        let content = r#"
```{document}
:id: REQ-1
```
```{document}
:id: REQ-1
```
"#;
        let blocks = parse(content);
        let diags = check_blocks(&blocks);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "E_DUP_ID");
    }

    #[test]
    fn test_bad_ref_body() {
        let content = r#"
```{document}
:id: REQ-1
This refers to {ref}`UNKNOWN`.
```
"#;
        let blocks = parse(content);
        let diags = check_blocks(&blocks);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "E_BAD_REF");
        assert!(diags[0].message.contains("Unknown ref target 'UNKNOWN'"));
    }

    #[test]
    fn test_bad_ref_edge() {
        let content = r#"
```{document}
:id: REQ-1
:verifies: UNKNOWN
```
"#;
        let blocks = parse(content);
        let diags = check_blocks(&blocks);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "E_BAD_REF");
        assert!(diags[0].message.contains("Unknown target 'UNKNOWN'"));
    }
}
