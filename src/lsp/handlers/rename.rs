use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub fn rename(
    blocks: &[crate::core::types::SpecBlock],
    refs: &[crate::core::types::RefUse],
    params: RenameParams
) -> Result<Option<WorkspaceEdit>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let line = position.line as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let mut target_id = None;
        for block in blocks.iter() {
            if block.file_path == path && block.line_start == line {
                target_id = Some(block.id.clone());
                break;
            }
        }

        if let Some(id) = target_id {
            let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> =
                std::collections::HashMap::new();
            for block in blocks.iter() {
                if block.id == id
                    && let Ok(u) = Url::from_file_path(&block.file_path)
                    && let Ok(content) = std::fs::read_to_string(&block.file_path)
                {
                    let lines: Vec<&str> = content.lines().collect();
                    if block.line_start <= lines.len() {
                        let line = lines[block.line_start - 1];
                        if let Some(pos) = line.find(&id) {
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
    use crate::core::types::{SpecBlock};
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_rename() {
        // Create a temp file for the rename target to satisfy read_to_string
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "## FR-01").unwrap();
        let path = temp_file.path().to_path_buf();

        let blocks = vec![
             SpecBlock {
                id: "FR-01".to_string(),
                file_path: path.clone(),
                line_start: 1, // matches content
                line_end: 1,
                ..Default::default()
             }
        ];
        let refs = vec![];

        let uri = Url::from_file_path(&path).unwrap();
        let params = RenameParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line: 0, character: 0 },
            },
            new_name: "FR-NEW".to_string(),
            work_done_progress_params: Default::default(),
        };

        let result = rename(&blocks, &refs, params).unwrap();
        if let Some(edit) = result {
             let changes = edit.changes.unwrap();
             assert!(changes.len() > 0);
             let edits = changes.values().next().unwrap();
             assert_eq!(edits[0].new_text, "FR-NEW");
             // "## FR-01" -> "FR-01" is at index 3
             assert_eq!(edits[0].range.start.character, 3);
        } else {
            panic!("Expected rename edits");
        }
    }
}
