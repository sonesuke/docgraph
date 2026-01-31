use crate::core::types::{EdgeUse, RefUse, SpecBlock};
use regex::Regex;
use std::path::Path;

/// Extract anchor headings: `<a id="XXX"></a>` followed by a Markdown heading
/// Scope extends from anchor to next anchor (or EOF), extracting refs as edges
pub fn extract_anchor_headings(content: &str, file_path: &Path) -> Vec<SpecBlock> {
    let raw_lines: Vec<&str> = content.lines().collect();
    let clean_content = crate::core::utils::strip_markdown_code(content);
    let clean_lines: Vec<&str> = clean_content.lines().collect();

    // Regex for <a id="XXX"></a> or <a id='XXX'></a> (must be the entire line)
    let anchor_re = Regex::new(r#"^<a\s+id=["']([^"']+)["']\s*>\s*</a>$"#).unwrap();
    // Regex for Markdown heading
    let heading_re = Regex::new(r"^(#{1,6})\s+(.+)$").unwrap();
    // Regex for Markdown links with fragment: [text](#ID) or [text](path#ID)
    let link_re = Regex::new(r"\[([^\]]*)\]\(([^)]*#([^)]+))\)").unwrap();

    // First pass: find all anchor positions using RAW lines
    // (IDs and Titles should be extracted as they appear in the file)
    let mut anchor_positions: Vec<(usize, String, Option<String>, usize)> = Vec::new(); // (line_idx, id, name, heading_line_idx)

    let mut i = 0;
    while i < raw_lines.len() {
        let trimmed = raw_lines[i].trim();

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        if let Some(caps) = anchor_re.captures(trimmed) {
            let id = caps.get(1).unwrap().as_str().to_string();

            // Look for heading in next non-empty lines
            let mut j = i + 1;
            while j < raw_lines.len() && raw_lines[j].trim().is_empty() {
                j += 1;
            }

            let (name, heading_idx) = if j < raw_lines.len() {
                if let Some(h_caps) = heading_re.captures(raw_lines[j].trim()) {
                    let raw_name = h_caps.get(2).unwrap().as_str();
                    // Strip ID prefix from heading if present
                    let clean_name = raw_name
                        .strip_prefix(&id)
                        .map(|s| s.trim_start())
                        .unwrap_or(raw_name)
                        .to_string();
                    (Some(clean_name), j)
                } else {
                    (None, i)
                }
            } else {
                (None, i)
            };

            anchor_positions.push((i, id, name, heading_idx));
            i = heading_idx;
        }
        i += 1;
    }

    // Second pass: build blocks with scoped refs using CLEAN lines
    // (Avoid extracting links from code blocks/fences)
    let mut blocks = Vec::new();

    for (idx, (anchor_idx, id, name, heading_idx)) in anchor_positions.iter().enumerate() {
        let anchor_line = anchor_idx + 1; // 1-based

        // Scope: from heading to next anchor (or EOF)
        let scope_start = *heading_idx + 1;
        let scope_end = if idx + 1 < anchor_positions.len() {
            anchor_positions[idx + 1].0
        } else {
            raw_lines.len()
        };

        // Extract refs within scope
        let mut edges = Vec::new();
        for line_idx in scope_start..scope_end {
            if line_idx < clean_lines.len() {
                for cap in link_re.captures_iter(clean_lines[line_idx]) {
                    if let Some(id_match) = cap.get(3) {
                        let target_id = id_match.as_str().to_string();
                        let display_name = cap.get(1).map(|m| m.as_str().to_string());

                        // 1. Edge for the ID in the URL (#ID)
                        edges.push(EdgeUse {
                            id: target_id.clone(),
                            name: display_name.clone(),
                            line: line_idx + 1, // 1-based
                            col_start: id_match.start() + 1,
                            col_end: id_match.end() + 1,
                        });

                        // 2. Edge for the ID in the Text ([ID])
                        if let Some(text_match) = cap.get(1) {
                            if let Some(idx) = text_match.as_str().find(&target_id) {
                                let start = text_match.start() + idx;
                                edges.push(EdgeUse {
                                    id: target_id,
                                    name: display_name,
                                    line: line_idx + 1,
                                    col_start: start + 1,
                                    col_end: start + 1 + id_match.as_str().len(),
                                });
                            }
                        }
                    }
                }
            }
        }

        blocks.push(SpecBlock {
            id: id.clone(),
            name: name.clone(),
            edges,
            file_path: file_path.to_path_buf(),
            line_start: anchor_line,
            line_end: scope_end,
        });
    }

    blocks
}

/// Extract Markdown link references: `[text](#ID)` or `[text](path#ID)`
pub fn extract_markdown_refs(content: &str, file_path: &Path) -> Vec<RefUse> {
    let mut refs = Vec::new();

    // Regex for Markdown links with fragment: [text](path#ID) or [text](#ID)
    let link_re = Regex::new(r"\[([^\]]*)\]\(([^)]*#([^)]+))\)").unwrap();

    let clean_content = crate::core::utils::strip_markdown_code(content);
    for (line_idx, line) in clean_content.lines().enumerate() {
        let line_num = line_idx + 1; // 1-based

        for cap in link_re.captures_iter(line) {
            if let Some(id_match) = cap.get(3) {
                let target_id = id_match.as_str().to_string();

                // 1. Ref for the ID in the URL (#ID)
                refs.push(RefUse {
                    target_id: target_id.clone(),
                    file_path: file_path.to_path_buf(),
                    line: line_num,
                    col_start: id_match.start() + 1,
                    col_end: id_match.end() + 1,
                });

                // 2. Ref for the ID in the Text ([ID])
                if let Some(text_match) = cap.get(1) {
                    if let Some(idx) = text_match.as_str().find(&target_id) {
                        let start = text_match.start() + idx;
                        refs.push(RefUse {
                            target_id,
                            file_path: file_path.to_path_buf(),
                            line: line_num,
                            col_start: start + 1,
                            col_end: start + 1 + id_match.as_str().len(),
                        });
                    }
                }
            }
        }
    }

    refs
}

/// Extract all definitions and references from content
pub fn extract_all(content: &str, file_path: &Path) -> (Vec<SpecBlock>, Vec<RefUse>) {
    let blocks = extract_anchor_headings(content, file_path);
    let standalone_refs = extract_markdown_refs(content, file_path);

    (blocks, standalone_refs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_extract_anchor_headings_with_scoped_refs() {
        let content = r#"
<a id="DAT-SSO-CONFIG"></a>

## SSO Configuration

Stores the Identity Provider details for a [Tenant](#DAT-TENANT).

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Unique identifier. |
| tenant_id | UUID | Foreign Key to [Tenants](#DAT-TENANT). |

<a id="DAT-USER"></a>

## User

A user in the system.

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Unique identifier. |
| sso_config_id | UUID | Foreign Key to [SSO Config](#DAT-SSO-CONFIG). |
"#;
        let path = PathBuf::from("test.md");
        let blocks = extract_anchor_headings(content, &path);

        assert_eq!(blocks.len(), 2);

        // First block: DAT-SSO-CONFIG
        let b1 = &blocks[0];
        assert_eq!(b1.id, "DAT-SSO-CONFIG");
        assert_eq!(b1.name.as_deref(), Some("SSO Configuration"));
        assert_eq!(b1.edges.len(), 2); // Two refs to DAT-TENANT
        assert!(b1.edges.iter().all(|e| e.id == "DAT-TENANT"));

        // Second block: DAT-USER
        let b2 = &blocks[1];
        assert_eq!(b2.id, "DAT-USER");
        assert_eq!(b2.name.as_deref(), Some("User"));
        assert_eq!(b2.edges.len(), 1);
        assert_eq!(b2.edges[0].id, "DAT-SSO-CONFIG");
    }

    #[test]
    fn test_extract_anchor_headings_skips_code_blocks() {
        let content = r#"
<a id="REQ-001"></a>
# REQ-001

This requirement references [REQ-002](#REQ-002).

```markdown
This link should be ignored: [REQ-003](#REQ-003)
```

The following is not in a code block: [REQ-004](#REQ-004)
"#;
        let path = PathBuf::from("test.md");
        let blocks = extract_anchor_headings(content, &path);

        assert_eq!(blocks.len(), 1);
        let b = &blocks[0];
        assert_eq!(b.id, "REQ-001");

        // Should find REQ-002 and REQ-004, but NOT REQ-003
        let target_ids: Vec<String> = b.edges.iter().map(|e| e.id.clone()).collect();
        assert!(target_ids.contains(&"REQ-002".to_string()));
        assert!(target_ids.contains(&"REQ-004".to_string()));
        assert!(
            !target_ids.contains(&"REQ-003".to_string()),
            "Should not extract refs from code blocks"
        );
    }

    #[test]
    fn test_extract_markdown_refs_skips_code_blocks() {
        let content = r#"
This is a standalone ref: [REF-001](#REF-001)

```markdown
This should be ignored: [REF-002](#REF-002)
```

Another valid ref: [REF-003](#REF-003)
"#;
        let path = PathBuf::from("test.md");
        let refs = extract_markdown_refs(content, &path);

        // Should find REF-001 and REF-003, but NOT REF-002
        let target_ids: Vec<String> = refs.iter().map(|r| r.target_id.clone()).collect();
        assert!(target_ids.contains(&"REF-001".to_string()));
        assert!(target_ids.contains(&"REF-003".to_string()));
        assert!(
            !target_ids.contains(&"REF-002".to_string()),
            "Should not extract standalone refs from code blocks"
        );
    }
    #[test]
    fn test_extract_refs_from_link_text() {
        let content = r#"
Reference to [REQ-001](#REQ-001) in text.
Mismatch text [Some Request](#REQ-002) here.
"#;
        let path = PathBuf::from("test.md");
        let refs = extract_markdown_refs(content, &path);

        // REQ-001: Should match twice (text and URL)
        // [REQ-001] starts at col 14. Text "REQ-001" starts at 15. URL "REQ-001" starts at 24.
        let req1_refs: Vec<&RefUse> = refs.iter().filter(|r| r.target_id == "REQ-001").collect();
        assert_eq!(req1_refs.len(), 2);

        // REQ-002: Should match once (URL only)
        let req2_refs: Vec<&RefUse> = refs.iter().filter(|r| r.target_id == "REQ-002").collect();
        assert_eq!(req2_refs.len(), 1);
    }
}
