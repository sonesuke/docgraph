use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub fn completion(
    blocks: &[crate::core::types::SpecBlock],
    _: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    let items = blocks
        .iter()
        .map(|block| CompletionItem {
            label: block.id.clone(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: block.name.clone(),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    "**{}**\n\nDefined in `{}`",
                    block.id,
                    block.file_path.display()
                ),
            })),
            ..Default::default()
        })
        .collect();
    Ok(Some(CompletionResponse::Array(items)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::SpecBlock;
    use std::path::PathBuf;

    #[test]
    fn test_completion() {
        let blocks = vec![SpecBlock {
            id: "FR-01".to_string(),
            name: Some("Test Req".to_string()),
            file_path: PathBuf::from("test.md"),
            ..Default::default()
        }];

        let params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("file:///test.md").unwrap(),
                },
                position: Position::new(0, 0),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        };

        let result = completion(&blocks, params).unwrap();
        if let Some(CompletionResponse::Array(items)) = result {
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].label, "FR-01");
            assert_eq!(items[0].detail.as_deref(), Some("Test Req"));
        } else {
            panic!("Expected array response");
        }
    }
}
