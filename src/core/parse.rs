use crate::core::types::{EdgeUse, RefUse, SpecBlock};
use pulldown_cmark::{Event, LinkType, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::path::Path;

/// Extract all definitions and references from content using pulldown-cmark
pub fn extract_all(content: &str, file_path: &Path) -> (Vec<SpecBlock>, Vec<RefUse>) {
    let mut blocks = Vec::new();
    let mut standalone_refs = Vec::new();

    // Context tracking
    let mut current_anchor_id: Option<String> = None;
    let mut current_anchor_line: usize = 0;
    let mut current_block_refs: Vec<EdgeUse> = Vec::new();
    let mut current_block_name: Option<String> = None;

    // We need to track the *byte offset* to *line/column* mapping manually or helper
    // pulldown-cmark gives byte offsets.
    let line_offsets: Vec<usize> = std::iter::once(0)
        .chain(content.match_indices('\n').map(|(i, _)| i + 1))
        .collect();

    let offset_to_line_col = |offset: usize| -> (usize, usize) {
        // Binary search for the line
        let line_idx = match line_offsets.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        let line_start = line_offsets[line_idx];
        let col = offset - line_start + 1; // 1-based
        (line_idx + 1, col)
    };

    let mut parser = Parser::new_ext(content, Options::all()).into_offset_iter();

    #[allow(clippy::while_let_on_iterator)]
    while let Some((event, range)) = parser.next() {
        match event {
            // Check for HTML anchor tag: <a id="XXX"></a>
            Event::Html(html) | Event::InlineHtml(html) => {
                // Determine if this is a block start anchor
                if let Some(id) = parse_anchor_tag(&html) {
                    // If we were already in a block, close it
                    if let Some(prev_id) = current_anchor_id.take() {
                        let (end_line, _) = offset_to_line_col(range.start);
                        blocks.push(SpecBlock {
                            id: prev_id,
                            name: current_block_name.take(),
                            edges: std::mem::take(&mut current_block_refs),
                            file_path: file_path.to_path_buf(),
                            line_start: current_anchor_line,
                            line_end: end_line, // Ends at start of new anchor
                        });
                    }

                    // Start new block
                    current_anchor_id = Some(id);
                    let (start_line, _) = offset_to_line_col(range.start);
                    current_anchor_line = start_line;
                }
            }

            // Check for Heading immediately following an anchor
            Event::Start(Tag::Heading {
                level: _,
                id: _,
                classes: _,
                attrs: _,
            }) => {
                if let Some(ref anchor_id) = current_anchor_id {
                    // Extract heading text
                    let mut heading_text = String::new();
                    // Consume events until heading end
                    while let Some((h_event, _)) = parser.next() {
                        match h_event {
                            Event::Text(text) => heading_text.push_str(&text),
                            Event::Code(text) => heading_text.push_str(&text),
                            Event::End(TagEnd::Heading(_)) => break,
                            _ => {}
                        }
                    }

                    // Strip ID prefix if present to clean up name
                    let clean_name = heading_text
                        .strip_prefix(anchor_id)
                        .map(|s| s.trim_start())
                        .unwrap_or(&heading_text)
                        .to_string();

                    if current_block_name.is_none() && !clean_name.is_empty() {
                        current_block_name = Some(clean_name);
                    }
                }
            }

            // Check for Links: [text](#ID)
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                ..
            }) => {
                // We only care about inline links with fragment
                // Note: pulldown-cmark might handle reference links differently
                if (link_type == LinkType::Inline
                    || link_type == LinkType::Reference
                    || link_type == LinkType::Shortcut)
                    && let Some(target_id) = parse_link_fragment(&dest_url)
                {
                    let (line, col) = offset_to_line_col(range.start);
                    let col_end = offset_to_line_col(range.end).1;

                    // We also need to extract the link text for "name" field if possible
                    // But getting the text inside the link requires consuming events or peeking?
                    // For simplicity, we can use the raw string from byte range if needed,
                    // but strictly speaking we just need the edge.
                    // Let's grab the text from the content slice for the display name.
                    let full_link_text = &content[range.clone()];
                    let display_name = parse_link_text(full_link_text);

                    let edge = EdgeUse {
                        id: target_id.clone(),
                        name: display_name,
                        line,
                        col_start: col,
                        col_end,
                    };

                    if current_anchor_id.is_some() {
                        current_block_refs.push(edge);
                    } else {
                        // Standalone ref
                        standalone_refs.push(RefUse {
                            target_id,
                            file_path: file_path.to_path_buf(),
                            line,
                            col_start: col,
                            col_end,
                        });
                    }
                }
            }
            // Ignore other events
            _ => {}
        }
    }

    // Close final block
    if let Some(prev_id) = current_anchor_id {
        let (end_line, _) = offset_to_line_col(content.len());
        blocks.push(SpecBlock {
            id: prev_id,
            name: current_block_name,
            edges: current_block_refs, // took ownership
            file_path: file_path.to_path_buf(),
            line_start: current_anchor_line,
            line_end: end_line,
        });
    }

    (blocks, standalone_refs)
}

/// Helper to parse <a id="XXX"></a>
fn parse_anchor_tag(html: &str) -> Option<String> {
    // Relaxed regex to match <a id="..."> (start tag only is enough) including inside InlineHtml
    let re = Regex::new(r#"<a\s+id=["']([^"']+)["']"#).ok()?;
    re.captures(html)
        .map(|c| c.get(1).unwrap().as_str().to_string())
}

/// Helper to extract ID from link destination: #ID or path#ID
fn parse_link_fragment(dest: &str) -> Option<String> {
    if let Some(idx) = dest.find('#') {
        let id_part = &dest[idx + 1..];
        if !id_part.is_empty() {
            return Some(id_part.to_string());
        }
    }
    None
}

/// Helper to exact display text from full link string [text](url)
fn parse_link_text(raw_link: &str) -> Option<String> {
    // This is a rough estimation.
    // Ideally we would inspect the nested events, but that complicates the main loop.
    // Regex is safe enough for standard markdown links here.
    let re = Regex::new(r"^\[([^\]]*)\]").ok()?;
    re.captures(raw_link)
        .map(|c| c.get(1).unwrap().as_str().to_string())
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
        let (blocks, _) = extract_all(content, &path);

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
        let (blocks, _) = extract_all(content, &path);

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
    fn test_extract_standalone_refs() {
        let content = r#"
This is a standalone ref: [REF-001](#REF-001)

```markdown
This should be ignored: [REF-002](#REF-002)
```

Another valid ref: [REF-003](#REF-003)
"#;
        let path = PathBuf::from("test.md");
        let (_, refs) = extract_all(content, &path);

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
    fn test_extract_anchor_multiple_headings() {
        let content = r#"
<a id="IF_CONFIG"></a>

## docgraph.toml Configuration

### Sub-heading

Some content.
"#;
        let path = PathBuf::from("test.md");
        let (blocks, _) = extract_all(content, &path);

        assert_eq!(blocks.len(), 1);
        let b = &blocks[0];
        assert_eq!(b.id, "IF_CONFIG");

        // Should capture the FIRST heading
        assert_eq!(b.name.as_deref(), Some("docgraph.toml Configuration"));
    }
}
