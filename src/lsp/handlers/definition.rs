use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub fn goto_definition(
    blocks: &[crate::core::types::SpecBlock],
    refs: &[crate::core::types::RefUse],
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let mut target_id = None;

        for block in blocks.iter() {
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

        if let Some(id) = target_id
            && let Some(target_block) = blocks.iter().find(|b| b.id == id)
            && let Ok(target_uri) = Url::from_file_path(&target_block.file_path)
        {
            return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri: target_uri,
                range: Range {
                    start: Position {
                        line: target_block.line_start as u32 - 1,
                        character: 0,
                    },
                    end: Position {
                        line: target_block.line_start as u32 - 1,
                        character: 0,
                    },
                },
            })));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{EdgeUse, SpecBlock};

    #[test]
    fn test_goto_definition() {
        let path = std::env::current_dir().unwrap().join("test.md");
        let target_pos = std::env::current_dir().unwrap().join("target.md");
        let blocks = vec![
            SpecBlock {
                id: "FR-01".to_string(),
                file_path: target_pos.clone(),
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
            },
        ];
        let refs = vec![];

        let uri = Url::from_file_path(path).unwrap();
        let params = GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 1,
                    character: 6,
                }, // Inside the edge range match (line 2 is index 1)
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };

        let result = goto_definition(&blocks, &refs, params).unwrap();
        if let Some(GotoDefinitionResponse::Scalar(loc)) = result {
            assert_eq!(loc.uri, Url::from_file_path(target_pos).unwrap());
            assert_eq!(loc.range.start.line, 9);
        } else {
            panic!("Expected definition response");
        }
    }
}
