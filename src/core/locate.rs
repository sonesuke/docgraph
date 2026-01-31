use crate::core::types::{RefUse, SpecBlock};
use regex::Regex;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub enum LocateTarget {
    Definition(SpecBlock),
    Edge(String), // Edge ID
    Reference(RefUse),
}

impl LocateTarget {
    pub fn id(&self) -> String {
        match self {
            LocateTarget::Definition(b) => b.id.clone(),
            LocateTarget::Edge(id) => id.clone(),
            LocateTarget::Reference(r) => r.target_id.clone(),
        }
    }
}

pub struct LocateResult {
    pub file_path: PathBuf,
    pub range_start_line: usize, // 1-based
    pub range_start_col: usize,  // 1-based
    pub range_end_line: usize,   // 1-based
    pub range_end_col: usize,    // 1-based
}

/// Locate the target ID at the given position.
/// Returns the target ID if found.
pub fn locate_id_at_position(
    blocks: &[SpecBlock],
    refs: &[RefUse],
    path: &Path,
    line: usize,
    col: usize,
) -> Option<String> {
    let mut target_id = None;

    // 1. Check if cursor is on a Definition (Block)
    let anchor_re = Regex::new(r#"<a\s+id=["']([^"']+)["']\s*>\s*</a>"#).unwrap();

    for block in blocks.iter() {
        if block.file_path == path && block.line_start == line {
            // Read file line to check exact column match
            if let Ok(content) = std::fs::read_to_string(&block.file_path)
                && let Some(line_content) = content.lines().nth(line - 1)
                && let Some(caps) = anchor_re.captures(line_content)
                && let Some(id_match) = caps.get(1)
            {
                let start = id_match.start() + 1;
                let end = id_match.end() + 1;
                // Strict check: cursor must be within the ID string
                if block.id == id_match.as_str() && col >= start && col <= end {
                    target_id = Some(block.id.clone());
                }
            }
            break;
        }
    }

    // 2. Check if cursor is on an Edge (Link text or URL)
    if target_id.is_none() {
        for block in blocks.iter() {
            if block.file_path == path {
                for edge in &block.edges {
                    if edge.line == line && col >= edge.col_start && col <= edge.col_end {
                        target_id = Some(edge.id.clone());
                        break;
                    }
                }
            }
            if target_id.is_some() {
                break;
            }
        }
    }

    // 3. Check if cursor is on a Ref (Standalone Link)
    if target_id.is_none() {
        for r in refs.iter() {
            if r.file_path == path && r.line == line && col >= r.col_start && col <= r.col_end {
                target_id = Some(r.target_id.clone());
                break;
            }
        }
    }

    target_id
}

/// Find all references to the target ID (definition, usage in edges, usage in refs).
pub fn find_references_msg(
    blocks: &[SpecBlock],
    refs: &[RefUse],
    target_id: &str,
) -> Vec<LocateResult> {
    let mut results = Vec::new();
    let id_pattern = regex::escape(target_id);
    // Matches id="ID" or id='ID'
    let def_re = Regex::new(&format!(r#"id=["']({})["']"#, id_pattern)).unwrap();

    // 1. Definition
    for block in blocks.iter() {
        if block.id == target_id
            && let Ok(content) = std::fs::read_to_string(&block.file_path)
        {
            let lines: Vec<&str> = content.lines().collect();
            if block.line_start <= lines.len() {
                let line_content = lines[block.line_start - 1];
                // Find position of the ID within the anchor tag
                if let Some(caps) = def_re.captures(line_content)
                    && let Some(m) = caps.get(1)
                {
                    results.push(LocateResult {
                        file_path: block.file_path.clone(),
                        range_start_line: block.line_start,
                        range_start_col: m.start() + 1,
                        range_end_line: block.line_start,
                        range_end_col: m.end() + 1,
                    });
                } else if let Some(pos) = line_content.find(target_id) {
                    // Fallback
                    results.push(LocateResult {
                        file_path: block.file_path.clone(),
                        range_start_line: block.line_start,
                        range_start_col: pos + 1,
                        range_end_line: block.line_start,
                        range_end_col: pos + 1 + target_id.len(),
                    });
                }
            }
        }
    }

    // 2. Edges
    for block in blocks.iter() {
        for edge in &block.edges {
            if edge.id == target_id {
                results.push(LocateResult {
                    file_path: block.file_path.clone(),
                    range_start_line: edge.line,
                    range_start_col: edge.col_start,
                    range_end_line: edge.line,
                    range_end_col: edge.col_end,
                });
            }
        }
    }

    // 3. Refs
    for r in refs.iter() {
        if r.target_id == target_id {
            results.push(LocateResult {
                file_path: r.file_path.clone(),
                range_start_line: r.line,
                range_start_col: r.col_start,
                range_end_line: r.line,
                range_end_col: r.col_end,
            });
        }
    }

    results
}

/// Find the definition location of the target ID.
pub fn find_definition(blocks: &[SpecBlock], target_id: &str) -> Option<LocateResult> {
    let id_pattern = regex::escape(target_id);
    let def_re = Regex::new(&format!(r#"id=["']({})["']"#, id_pattern)).unwrap();

    for block in blocks.iter() {
        if block.id == target_id {
            if let Ok(content) = std::fs::read_to_string(&block.file_path) {
                let lines: Vec<&str> = content.lines().collect();
                if block.line_start <= lines.len() {
                    let line_content = lines[block.line_start - 1];
                    // Find position of the ID within the anchor tag
                    if let Some(caps) = def_re.captures(line_content) {
                        if let Some(m) = caps.get(1) {
                            return Some(LocateResult {
                                file_path: block.file_path.clone(),
                                range_start_line: block.line_start,
                                range_start_col: m.start() + 1,
                                range_end_line: block.line_start,
                                range_end_col: m.end() + 1,
                            });
                        }
                    } else if let Some(pos) = line_content.find(target_id) {
                        // Fallback
                        return Some(LocateResult {
                            file_path: block.file_path.clone(),
                            range_start_line: block.line_start,
                            range_start_col: pos + 1,
                            range_end_line: block.line_start,
                            range_end_col: pos + 1 + target_id.len(),
                        });
                    }
                }
            }
            // Fallback if file read fails or line not found (shouldn't happen for valid blocks)
            return Some(LocateResult {
                file_path: block.file_path.clone(),
                range_start_line: block.line_start,
                range_start_col: 1,
                range_end_line: block.line_start,
                range_end_col: 1,
            });
        }
    }
    None
}

/// Get all candidates for completion (currently just all blocks).
pub fn completion_candidates(blocks: &[SpecBlock]) -> Vec<&SpecBlock> {
    blocks.iter().collect()
}

/// Find all outgoing edges from the target ID.
pub fn find_outgoing_edges(blocks: &[SpecBlock], target_id: &str) -> Vec<LocateResult> {
    let mut results = Vec::new();
    if let Some(block) = blocks.iter().find(|b| b.id == target_id) {
        for edge in &block.edges {
            // We want to point to the edge usage in the source file, which is what we have in edge.line/col
            results.push(LocateResult {
                file_path: block.file_path.clone(),
                range_start_line: edge.line,
                range_start_col: edge.col_start,
                range_end_line: edge.line,
                range_end_col: edge.col_end,
            });
            // Note: Call Hierarchy Outgoing Calls expects "to" item (the target block) and "from_ranges" (the edge range).
            // But we can resolve the "to" item using the edge.id later in the handler.
            // This function returns the LOCATION of the edge.
            // Wait, LocateResult doesn't carry the "target_id" of the edge.
            // We might need a richer return type or just return the edges?
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::SpecBlock;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_locate_definition_strict() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "<a id=\"FR-01\"></a>").unwrap();
        let path = temp_file.path().to_path_buf();

        let blocks = vec![SpecBlock {
            id: "FR-01".to_string(),
            file_path: path.clone(),
            line_start: 1,
            line_end: 1,
            ..Default::default()
        }];
        let refs = vec![];

        // Cursor at column 8 (start of "FR-01") <a id="...
        let id_found = locate_id_at_position(&blocks, &refs, &path, 1, 8);
        assert_eq!(id_found, Some("FR-01".to_string()));

        // Cursor at column 7 (quote) - should fail strict check if outside ID?
        // <a id="FR-01
        // 01234567
        // regex capture group is just FR-01. matched start is 7 (0-based) = 8 (1-based)?
        // regex match indices are byte offsets.
        // <a id="FR-01...
        // 01234567
        // m.start() for FR-01 is 7. +1 = 8.
        // col 8 is 'F'. Correct.

        let id_found_outside = locate_id_at_position(&blocks, &refs, &path, 1, 7);
        // " fallback: if regex fail but line matches, assume block id " logic exists but only if loop continues or check fails.
        // The implementation checks "if block.id == ... && col >= start ...". If strict check fails, it goes to fallback?
        // The cleanup: "if block.id == ... { target_id = Some... }" is inside the loop.
        // Fallback is AFTER the loop? No, inside "for block in blocks".

        assert_eq!(id_found_outside, None); // Should be None if strict
    }

    #[test]
    fn test_locate_cc_reproduction() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "<a id=\"CC_LAYERED_ARCH\"></a>").unwrap();
        let path = temp_file.path().to_path_buf();

        let blocks = vec![SpecBlock {
            id: "CC_LAYERED_ARCH".to_string(),
            file_path: path.clone(),
            line_start: 1,
            line_end: 1,
            ..Default::default()
        }];
        let refs = vec![];

        // <a id="CC_LAYERED_ARCH
        // 01234567
        // m.start() = 7. +1 = 8.
        // col 8 is 'C'.

        let id = locate_id_at_position(&blocks, &refs, &path, 1, 8);
        assert_eq!(id, Some("CC_LAYERED_ARCH".to_string()));
    }

    #[test]
    fn test_find_outgoing_edges() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "<a id=\"BLOCK_A\"></a>").unwrap();
        let path = temp_file.path().to_path_buf();

        let edge = crate::core::types::EdgeUse {
            id: "BLOCK_B".to_string(),
            name: Some("Block B".to_string()),
            line: 5,
            col_start: 10,
            col_end: 20,
        };

        let blocks = vec![SpecBlock {
            id: "BLOCK_A".to_string(),
            file_path: path.clone(),
            line_start: 1,
            line_end: 10,
            edges: vec![edge.clone()],
            ..Default::default()
        }];

        let results = find_outgoing_edges(&blocks, "BLOCK_A");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].range_start_line, 5);
        assert_eq!(results[0].range_start_col, 10);
        assert_eq!(results[0].range_end_col, 20);
    }

    #[test]
    fn test_find_definition_details() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "<a id=\"DEF_1\"></a>").unwrap();
        let path = temp_file.path().to_path_buf();

        let blocks = vec![SpecBlock {
            id: "DEF_1".to_string(),
            file_path: path.clone(),
            line_start: 1,
            line_end: 1,
            ..Default::default()
        }];

        // <a id="DEF_1"></a>
        // 01234567890123
        // id match is "DEF_1". start=7 (0-based) -> 8 (1-based). end=12 -> 13.

        let result = find_definition(&blocks, "DEF_1");
        assert!(result.is_some());
        let res = result.unwrap();
        assert_eq!(res.range_start_line, 1);
        assert_eq!(res.range_start_col, 8);
        assert_eq!(res.range_end_col, 13);
    }

    #[test]
    fn test_completion_candidates() {
        let blocks = vec![
            SpecBlock {
                id: "A".to_string(),
                ..Default::default()
            },
            SpecBlock {
                id: "B".to_string(),
                ..Default::default()
            },
        ];
        let candidates = completion_candidates(&blocks);
        assert_eq!(candidates.len(), 2);
    }

    #[test]
    fn test_locate_target_id_method() {
        let block = SpecBlock {
            id: "BLK".to_string(),
            ..Default::default()
        };
        let t1 = LocateTarget::Definition(block);
        assert_eq!(t1.id(), "BLK");

        let t2 = LocateTarget::Edge("EDGE".to_string());
        assert_eq!(t2.id(), "EDGE");

        let r = RefUse {
            target_id: "REF".to_string(),
            ..Default::default()
        };
        let t3 = LocateTarget::Reference(r);
        assert_eq!(t3.id(), "REF");
    }
}
