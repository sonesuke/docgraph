use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use crate::lsp::backend::Backend;

pub async fn rename(backend: &Backend, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let new_name = params.new_name;
    let line = position.line as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let blocks = backend.blocks.lock().await;
        let refs = backend.standalone_refs.lock().await;

        let mut target_id = None;
        for block in blocks.iter() {
            if block.file_path == path && block.line_start == line {
                target_id = Some(block.id.clone());
                break;
            }
        }

        if let Some(id) = target_id {
            let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> = std::collections::HashMap::new();
            for block in blocks.iter() {
                if block.id == id {
                    if let Ok(u) = Url::from_file_path(&block.file_path) {
                        if let Ok(content) = std::fs::read_to_string(&block.file_path) {
                            let lines: Vec<&str> = content.lines().collect();
                            if block.line_start <= lines.len() {
                                let line_content = lines[block.line_start - 1];
                                if let Some(start_idx) = line_content.find(&id) {
                                    changes.entry(u).or_default().push(TextEdit {
                                        range: Range {
                                            start: Position { line: block.line_start as u32 - 1, character: start_idx as u32 },
                                            end: Position { line: block.line_start as u32 - 1, character: (start_idx + id.len()) as u32 },
                                        },
                                        new_text: new_name.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            for block in blocks.iter() {
                for edge in &block.edges {
                    if edge.id == id {
                        if let Ok(u) = Url::from_file_path(&block.file_path) {
                            changes.entry(u).or_default().push(TextEdit {
                                range: Range {
                                    start: Position { line: edge.line as u32 - 1, character: edge.col_start as u32 - 1 },
                                    end: Position { line: edge.line as u32 - 1, character: edge.col_end as u32 - 1 },
                                },
                                new_text: new_name.clone(),
                            });
                        }
                    }
                }
            }
            for r in refs.iter() {
                if r.target_id == id {
                    if let Ok(u) = Url::from_file_path(&r.file_path) {
                        changes.entry(u).or_default().push(TextEdit {
                            range: Range {
                                start: Position { line: r.line as u32 - 1, character: r.col_start as u32 - 1 },
                                end: Position { line: r.line as u32 - 1, character: r.col_end as u32 - 1 },
                            },
                            new_text: new_name.clone(),
                        });
                    }
                }
            }
            return Ok(Some(WorkspaceEdit { changes: Some(changes), ..Default::default() }));
        }
    }
    Ok(None)
}
