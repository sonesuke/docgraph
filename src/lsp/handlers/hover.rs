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
        // Delegate to Core Logic
        if let Some(target_id) = crate::core::locate::locate_id_at_position(blocks, refs, &path, line, col)
             && let Some(target_block) = blocks.iter().find(|b| b.id == target_id)
        {
                let title = target_block.name.as_deref().unwrap_or(&target_id);
                let mut markdown = format!("**{}** ({})", title, target_id);

                let incoming = blocks
                    .iter()
                    .filter(|b| b.edges.iter().any(|e| e.id == target_id))
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


