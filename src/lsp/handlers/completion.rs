use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub fn completion(
    blocks: &[crate::core::types::SpecBlock],
    _: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    // Delegate candidate selection to Core
    let candidates = crate::core::locate::completion_candidates(blocks);

    let items = candidates
        .into_iter()
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
