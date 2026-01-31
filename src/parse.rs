use crate::types::{EdgeUse, RefUse, SpecBlock};
use regex::Regex;
use std::path::Path;

/// Extract anchor headings: `<a id="XXX"></a>` followed by a Markdown heading
/// Scope extends from anchor to next anchor (or EOF), extracting refs as edges
pub fn extract_anchor_headings(content: &str, file_path: &Path) -> Vec<SpecBlock> {
    let lines: Vec<&str> = content.lines().collect();

    // Regex for <a id="XXX"></a> or <a id='XXX'></a>
    let anchor_re = Regex::new(r#"<a\s+id=["']([^"']+)["']\s*>\s*</a>"#).unwrap();
    // Regex for Markdown heading
    let heading_re = Regex::new(r"^(#{1,6})\s+(.+)$").unwrap();
    // Regex for Markdown links with fragment: [text](#ID) or [text](path#ID)
    let link_re = Regex::new(r"\[([^\]]*)\]\(([^)]*#([^)]+))\)").unwrap();

    // First pass: find all anchor positions and track code blocks
    let mut anchor_positions: Vec<(usize, String, Option<String>, usize)> = Vec::new(); // (line_idx, id, name, heading_line_idx)
    let mut is_code_line = vec![false; lines.len()];

    let mut i = 0;
    let mut in_code_fence = false;
    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Toggle code fence state
        if trimmed.starts_with("```") {
            in_code_fence = !in_code_fence;
            is_code_line[i] = true;
            i += 1;
            continue;
        }

        // Track if inside a code fence
        if in_code_fence {
            is_code_line[i] = true;
            i += 1;
            continue;
        }

        if let Some(caps) = anchor_re.captures(trimmed) {
            let id = caps.get(1).unwrap().as_str().to_string();

            // Look for heading in next non-empty lines
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }

            let (name, heading_idx) = if j < lines.len() {
                if let Some(h_caps) = heading_re.captures(lines[j].trim()) {
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

    // Second pass: build blocks with scoped refs
    let mut blocks = Vec::new();

    for (idx, (anchor_idx, id, name, heading_idx)) in anchor_positions.iter().enumerate() {
        let anchor_line = anchor_idx + 1; // 1-based

        // Scope: from heading to next anchor (or EOF)
        let scope_start = *heading_idx + 1;
        let scope_end = if idx + 1 < anchor_positions.len() {
            anchor_positions[idx + 1].0
        } else {
            lines.len()
        };

        // Extract refs within scope
        let mut edges = Vec::new();
        for line_idx in scope_start..scope_end {
            if line_idx < lines.len() && !is_code_line[line_idx] {
                for cap in link_re.captures_iter(lines[line_idx]) {
                    if let Some(id_match) = cap.get(3) {
                        let target_id = id_match.as_str().to_string();
                        let display_name = cap.get(1).map(|m| m.as_str().to_string());
                        edges.push(EdgeUse {
                            id: target_id,
                            name: display_name,
                            line: line_idx + 1, // 1-based
                        });
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

    let mut in_code_fence = false;
    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Toggle code fence state
        if trimmed.starts_with("```") {
            in_code_fence = !in_code_fence;
            continue;
        }

        // Skip processing if inside a code fence
        if in_code_fence {
            continue;
        }

        let line_num = line_idx + 1; // 1-based

        for cap in link_re.captures_iter(line) {
            if let Some(id_match) = cap.get(3) {
                let target_id = id_match.as_str().to_string();
                let col = id_match.start() + 1; // 1-based

                refs.push(RefUse {
                    target_id,
                    file_path: file_path.to_path_buf(),
                    line: line_num,
                    col,
                });
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
