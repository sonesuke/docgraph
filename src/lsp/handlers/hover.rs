use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub fn hover(
    blocks: &[crate::core::types::SpecBlock],
    refs: &[crate::core::types::RefUse],
    params: HoverParams,
) -> Result<Option<Hover>> {
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
        if target_id.is_none() {
            for block in blocks.iter() {
                if block.file_path == path && block.line_start == line {
                    target_id = Some(block.id.clone());
                    break;
                }
            }
        }

        if let Some(id) = target_id
            && let Some(target_block) = blocks.iter().find(|b| b.id == id)
        {
            let title = target_block.name.as_deref().unwrap_or(&id);
            let mut markdown = format!("**{}** ({})", title, id);

            let incoming = blocks
                .iter()
                .filter(|b| b.edges.iter().any(|e| e.id == id))
                .count();
            let outgoing = target_block.edges.len();

            markdown.push_str(&format!(
                "\n\nIncoming: {} | Outgoing: {}",
                incoming, outgoing
            ));

            return Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(markdown)),
                range: None,
            }));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::SpecBlock;

    #[test]
    fn test_hover_definition() {
        let path = std::env::current_dir().unwrap().join("test.md");
        let blocks = vec![SpecBlock {
            id: "FR-01".to_string(),
            name: Some("Test Req".to_string()),
            file_path: path.clone(),
            line_start: 1,
            line_end: 5,
            edges: vec![],
        }];
        let refs = vec![];

        let uri = Url::from_file_path(path).unwrap();
        let params = HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 0,
                    character: 0,
                },
            },
            work_done_progress_params: Default::default(),
        };

        let result = hover(&blocks, &refs, params).unwrap();
        if let Some(h) = result {
            match h.contents {
                HoverContents::Scalar(MarkedString::String(s)) => {
                    assert!(s.contains("**Test Req** (FR-01)"));
                }
                _ => panic!("Expected string content"),
            }
        } else {
            panic!("Expected hover response");
        }
    }
}
