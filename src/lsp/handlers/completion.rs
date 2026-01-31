use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use super::super::backend::Backend;

pub async fn completion(backend: &Backend, _: CompletionParams) -> Result<Option<CompletionResponse>> {
    let blocks = backend.blocks.lock().await;
    let items = blocks
        .iter()
        .map(|block| CompletionItem {
            label: block.id.clone(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: block.name.clone(),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!("**{}**\n\nDefined in `{}`", block.id, block.file_path.display()),
            })),
            ..Default::default()
        })
        .collect();
    Ok(Some(CompletionResponse::Array(items)))
}
