use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub fn references(
    blocks: &[crate::core::types::SpecBlock],
    refs: &[crate::core::types::RefUse],
    params: ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Ok(path) = uri.to_file_path() {
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
                    if edge.id == id
                        && let Ok(u) = Url::from_file_path(&block.file_path)
                    {
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
            for r in refs.iter() {
                if r.target_id == id
                    && let Ok(u) = Url::from_file_path(&r.file_path)
                {
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
            return Ok(Some(locations));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{SpecBlock, EdgeUse};

    #[test]
    fn test_references() {
        let path = std::env::current_dir().unwrap().join("test.md");
        let blocks = vec![
             SpecBlock {
                id: "FR-01".to_string(),
                file_path: path.clone(),
                line_start: 10,
                line_end: 12,
                ..Default::default()
             },
             SpecBlock {
                id: "UC-01".to_string(),
                file_path: path.clone(),
                line_start: 1,
                line_end: 5,
                edges: vec![EdgeUse {
                    id: "FR-01".to_string(),
                    line: 2,
                    col_start: 5,
                    col_end: 10,
                    ..Default::default()
                }],
                ..Default::default()
             }
        ];
        let refs = vec![];

        let uri = Url::from_file_path(&path).unwrap();
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line: 9, character: 0 }, // At definition of FR-01
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: ReferenceContext { include_declaration: true },
        };

        let result = references(&blocks, &refs, params).unwrap();
        if let Some(locs) = result {
             assert_eq!(locs.len(), 1);
             assert_eq!(locs[0].range.start.line, 1); // Edge at line 2 (index 1)
        } else {
            panic!("Expected references locations");
        }
    }
}
