use crate::lsp::uri_ext::{uri_from_file_path, UriExt};
use anyhow::Result;
use lsp_types::*;

pub fn goto_definition(
    blocks: &[crate::core::types::SpecBlock],
    refs: &[crate::core::types::RefUse],
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Some(path) = uri.to_file_path() {
        let path = std::fs::canonicalize(&path).unwrap_or(path);

        if let Some(target_id) =
            crate::core::locate::locate_id_at_position(blocks, refs, &path, line, col)
            && let Some(loc) = crate::core::locate::find_definition(blocks, &target_id)
            && let Some(target_uri) = uri_from_file_path(&loc.file_path)
        {
            return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri: target_uri,
                range: Range {
                    start: Position {
                        line: loc.range_start_line as u32 - 1,
                        character: 0,
                    },
                    end: Position {
                        line: loc.range_end_line as u32 - 1,
                        character: 0,
                    },
                },
            })));
        }
    }

    Ok(None)
}
