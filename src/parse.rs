use crate::types::{EdgeUse, RefUse, SpecBlock};
use regex::Regex;
use std::path::{Path, PathBuf};

pub fn extract_blocks(content: &str, file_path: &Path) -> Vec<SpecBlock> {
    let mut blocks = Vec::new();
    // Regex for fenced code block with {doc} info string
    // Matches: ```{document} [Title]\n[Content]```
    
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.trim_start().starts_with("```{document}") {
            let start_line = i + 1;
            
            // Find end of block
            let mut j = i + 1;
            let mut block_content_lines = Vec::new();
            while j < lines.len() {
                if lines[j].trim_start().starts_with("```") {
                    break;
                }
                block_content_lines.push(lines[j]);
                j += 1;
            }
            let end_line = j + 1;
            
            if j < lines.len() {
                // We found a complete block
                let block_str = block_content_lines.join("\n");
                if let Some(block) = parse_block_content(&block_str, file_path.to_path_buf(), start_line, end_line) {
                    blocks.push(block);
                }
                i = j;
            }
        }
        i += 1;
    }

    blocks
}

fn parse_block_content(content: &str, file_path: PathBuf, block_start: usize, block_end: usize) -> Option<SpecBlock> {
    let mut id = None;
    let mut kind = None;
    let mut edges = Vec::new();
    let mut refs = Vec::new(); // In-body refs

    let lines: Vec<&str> = content.lines().collect();
    let mut body_start_idx = 0;

    // 1. Parse Options Headers
    let option_re = Regex::new(r"^:([a-z_]+):\s*(.*)$").unwrap();
    
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // If line doesn't start with :, we assume option block ended (or never started if empty lines skipped)
        if !trimmed.starts_with(":") {
            body_start_idx = idx;
            break;
        }

        if let Some(caps) = option_re.captures(trimmed) {
            let key = caps.get(1).unwrap().as_str();
            let value = caps.get(2).unwrap().as_str().trim();

            match key {
                "id" => id = Some(value.to_string()),
                "kind" => kind = Some(value.to_string()),
                // Edges
                "verifies" | "depends_on" | "derived_from" => {
                    let targets: Vec<&str> = value.split_whitespace().collect();
                    for target in targets {
                        edges.push(EdgeUse {
                            edge_type: key.to_string(),
                            target_id: target.to_string(),
                            line: block_start + idx + 1, // +1 because block_start is the fence line
                        });
                    }
                }
                _ => {} // Ignore unknown or removed options
            }
        } else {
             // Line starts with : but format doesn't match, treat as body? 
             // Strictly MyST options must come first. If we hit a non-option, we stop option parsing.
             body_start_idx = idx;
             break;
        }
        body_start_idx = idx + 1;
    }

    id.as_ref()?;

    let id_val = id.unwrap_or_default();

    // 2. Parse Body for {ref}
    let ref_re = Regex::new(r"\{ref\}`([^`]+)`").unwrap();
    
    for (i, line_content) in lines.iter().enumerate().skip(body_start_idx) {
        let current_line_num = block_start + 1 + i; // +1 for fence
        
        for cap in ref_re.captures_iter(line_content) {
            if let Some(m) = cap.get(1) {
                let target_id = m.as_str();
                let col = m.start() + 1; // 1-based col
                refs.push(RefUse {
                    target_id: target_id.to_string(),
                    line: current_line_num,
                    col,
                });
            }
        }
    }

    Some(SpecBlock {
        id: id_val,
        kind,
        edges,
        refs,
        file_path,
        line_start: block_start,
        line_end: block_end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_block() {
        let content = r#"
```{document} Title
:id: REQ-001
:kind: req

Body with {ref}`REQ-002`.
```
"#;
        let path = PathBuf::from("test.md");
        let blocks = extract_blocks(content, &path);
        assert_eq!(blocks.len(), 1);
        let b = &blocks[0];
        assert_eq!(b.id, "REQ-001");
        assert_eq!(b.kind.as_deref(), Some("req"));
        assert_eq!(b.refs.len(), 1);
        assert_eq!(b.refs[0].target_id, "REQ-002");
    }

     #[test]
    fn test_extract_edges() {
        let content = r#"
```{document}
:id: T-01
:verifies: R-01 R-02
:depends_on: D-01
```
"#;
        let path = PathBuf::from("test.md");
        let blocks = extract_blocks(content, &path);
        let b = &blocks[0];
        assert_eq!(b.edges.len(), 3);
        assert!(b.edges.iter().any(|e| e.edge_type == "verifies" && e.target_id == "R-01"));
        assert!(b.edges.iter().any(|e| e.edge_type == "verifies" && e.target_id == "R-02"));
        assert!(b.edges.iter().any(|e| e.edge_type == "depends_on" && e.target_id == "D-01"));
    }
}
