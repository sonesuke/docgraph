use crate::lsp::backend::Backend;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub async fn hover(backend: &Backend, params: HoverParams) -> Result<Option<Hover>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let blocks = backend.blocks.lock().await;
        let refs = backend.standalone_refs.lock().await;

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

        if let Some(id) = target_id {
            if let Some(target_block) = blocks.iter().find(|b| b.id == id) {
                let title = target_block.name.as_deref().unwrap_or(&id);
                let mut markdown = format!("**{}** ({})", title, id);
                let ref_count = blocks
                    .iter()
                    .flat_map(|b| b.edges.iter())
                    .filter(|e| e.id == id)
                    .count()
                    + refs.iter().filter(|r| r.target_id == id).count();
                markdown.push_str(&format!(
                    "\n\nReferenced {} times in the workspace.",
                    ref_count
                ));

                return Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: markdown,
                    }),
                    range: None,
                }));
            }
        }
    }
    Ok(None)
}
