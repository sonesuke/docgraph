use regex::Regex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub fn rename(
    blocks: &[crate::core::types::SpecBlock],
    refs: &[crate::core::types::RefUse],
    params: RenameParams,
) -> Result<Option<WorkspaceEdit>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let mut target_id = None;

        // 1. Check if cursor is on a Definition (Block)
        // Parse the line to see if cursor is within the ID attribute
        let anchor_re = Regex::new(r#"<a\s+id=["']([^"']+)["']\s*>\s*</a>"#).unwrap();

        for block in blocks.iter() {
            if block.file_path == path && block.line_start == line {
                // Read file line to check exact column match
                if let Ok(content) = std::fs::read_to_string(&block.file_path) {
                    if let Some(line_content) = content.lines().nth(line - 1) {
                        if let Some(caps) = anchor_re.captures(line_content) {
                            if let Some(id_match) = caps.get(1) {
                                // Check if cursor col is within the ID match range
                                let start = id_match.start() + 1;
                                let end = id_match.end() + 1;
                                // Strict check: cursor must be within the ID string
                                if block.id == id_match.as_str() && col >= start && col <= end {
                                    target_id = Some(block.id.clone());
                                }
                            }
                        }
                    }
                }
                // Fallback: if regex fail but line matches, assume block id
                if target_id.is_none() {
                    target_id = Some(block.id.clone());
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

        if let Some(id) = target_id {
            let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> =
                std::collections::HashMap::new();

            // Helper regex for replacement identifying
            let id_pattern = regex::escape(&id);
            // Matches id="ID" or id='ID'
            let def_re = Regex::new(&format!(r#"id=["']({})["']"#, id_pattern)).unwrap();

            // 1. Update Definitions
            for block in blocks.iter() {
                if block.id == id
                    && let Ok(u) = Url::from_file_path(&block.file_path)
                    && let Ok(content) = std::fs::read_to_string(&block.file_path)
                {
                    let lines: Vec<&str> = content.lines().collect();
                    if block.line_start <= lines.len() {
                        let line_content = lines[block.line_start - 1];
                        // Find position of the ID within the anchor tag
                        if let Some(caps) = def_re.captures(line_content) {
                            if let Some(m) = caps.get(1) {
                                changes.entry(u).or_default().push(TextEdit {
                                    range: Range {
                                        start: Position {
                                            line: block.line_start as u32 - 1,
                                            character: m.start() as u32,
                                        },
                                        end: Position {
                                            line: block.line_start as u32 - 1,
                                            character: m.end() as u32,
                                        },
                                    },
                                    new_text: params.new_name.clone(),
                                });
                            }
                        } else if let Some(pos) = line_content.find(&id) {
                            // Fallback to simple find if regex fails (e.g. malformed or simple text)
                            changes.entry(u).or_default().push(TextEdit {
                                range: Range {
                                    start: Position {
                                        line: block.line_start as u32 - 1,
                                        character: pos as u32,
                                    },
                                    end: Position {
                                        line: block.line_start as u32 - 1,
                                        character: (pos + id.len()) as u32,
                                    },
                                },
                                new_text: params.new_name.clone(),
                            });
                        }
                    }
                }
            }

            // 2. Update Edges (Reference in blocks)
            for block in blocks.iter() {
                for edge in &block.edges {
                    if edge.id == id
                        && let Ok(u) = Url::from_file_path(&block.file_path)
                    {
                        changes.entry(u).or_default().push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: edge.line as u32 - 1,
                                    character: edge.col_start as u32 - 1,
                                },
                                end: Position {
                                    line: edge.line as u32 - 1,
                                    character: edge.col_end as u32 - 1,
                                },
                            },
                            new_text: params.new_name.clone(),
                        });
                    }
                }
            }

            // 3. Update Refs (Standalone references)
            for r in refs.iter() {
                if r.target_id == id
                    && let Ok(u) = Url::from_file_path(&r.file_path)
                {
                    changes.entry(u).or_default().push(TextEdit {
                        range: Range {
                            start: Position {
                                line: r.line as u32 - 1,
                                character: r.col_start as u32 - 1,
                            },
                            end: Position {
                                line: r.line as u32 - 1,
                                character: r.col_end as u32 - 1,
                            },
                        },
                        new_text: params.new_name.clone(),
                    });
                }
            }
            return Ok(Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{RefUse, SpecBlock};
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_rename_definition() {
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

        let uri = Url::from_file_path(&path).unwrap();
        // Trigger rename on the ID part "FR-01" (col 8 to 13)
        let params = RenameParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 0,
                    character: 9,
                },
            },
            new_name: "FR-NEW".to_string(),
            work_done_progress_params: Default::default(),
        };

        let result = rename(&blocks, &refs, params).unwrap();
        let changes = result.expect("Edit").changes.unwrap();
        let edits = changes.values().next().unwrap();

        assert_eq!(edits[0].new_text, "FR-NEW");
        // <a id="FR-01"></a>  -> 'FR-01' starts at index 7 (0: <, 1: a, 2:  , 3: i, 4: d, 5: =, 6: ", 7: F)
        assert_eq!(edits[0].range.start.character, 7);
    }

    #[test]
    fn test_rename_reference() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // content doesn't matter much as long as fs read works, but we mock the logic via edges/refs data
        writeln!(temp_file, "Ref to [FR-01](#FR-01)").unwrap();
        let path = temp_file.path().to_path_buf();

        let blocks = vec![SpecBlock {
            id: "FR-01".to_string(),
            // Definition elsewhere
            file_path: std::env::current_dir().unwrap().join("other.md"),
            line_start: 10,
            line_end: 10,
            ..Default::default()
        }]; // But we need the definition file to enable replacement there too (if we tested that)

        // Simulating a Ref at line 1, col 20 (the #FR-01 part)
        // [FR-01] is col 8-15 (text)
        // (#FR-01) is col 16-23 (url part, id starts at 18)

        let refs = vec![RefUse {
            target_id: "FR-01".to_string(),
            file_path: path.clone(),
            line: 1,
            col_start: 18,
            col_end: 23,
        }];

        let uri = Url::from_file_path(&path).unwrap();
        let params = RenameParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 0,
                    character: 19,
                }, // Inside "FR-01"
            },
            new_name: "FR-NEW".to_string(),
            work_done_progress_params: Default::default(),
        };

        let result = rename(&blocks, &refs, params).unwrap();
        let changes = result.expect("Edit").changes.unwrap();
        let edits = changes.values().next().unwrap();

        assert_eq!(edits[0].new_text, "FR-NEW");
        assert_eq!(edits[0].range.start.character, 17); // 0-based index for col 18
    }
}
