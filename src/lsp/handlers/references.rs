use crate::lsp::backend::Backend;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub async fn references(
    backend: &Backend,
    params: ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let blocks = backend.blocks.lock().await;
        let refs = backend.standalone_refs.lock().await;

        let mut target_id = None;

        for block in blocks.iter() {
            if block.file_path == path && block.line_start == line {
                target_id = Some(block.id.clone());
                break;
            }
            if block.file_path == path && line >= block.line_start && line <= block.line_end {
                for edge in &block.edges {
                    if edge.line == line && col >= edge.col_start && col <= edge.col_end {
                        target_id = Some(edge.id.clone());
                        break;
                    }
                }
            }
        }
        if target_id.is_none() {
            for r in refs.iter() {
                if r.file_path == path && r.line == line && col >= r.col_start && col <= r.col_end {
                    target_id = Some(r.target_id.clone());
                    break;
                }
            }
        }

        if let Some(id) = target_id {
            let mut locations = Vec::new();
            for block in blocks.iter() {
                for edge in &block.edges {
                    if edge.id == id {
                        if let Ok(u) = Url::from_file_path(&block.file_path) {
                            locations.push(Location {
                                uri: u,
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
                            });
                        }
                    }
                }
            }
            for r in refs.iter() {
                if r.target_id == id {
                    if let Ok(u) = Url::from_file_path(&r.file_path) {
                        locations.push(Location {
                            uri: u,
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
                        });
                    }
                }
            }
            return Ok(Some(locations));
        }
    }
    Ok(None)
}
