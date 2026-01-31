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

    if let Ok(path) = uri.to_file_path()
        && let Some(target_id) = crate::core::locate::locate_id_at_position(blocks, refs, &path, line, col)
    {
            if let Some(loc) = crate::core::locate::find_definition(blocks, &target_id)
                && let Ok(target_uri) = Url::from_file_path(&loc.file_path)
            {
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: target_uri,
                        range: Range {
                            start: Position {
                                line: loc.range_start_line as u32 - 1,
                                character: 0, // Goto definition usually jumps to start of line or block. 0 is fine.
                                // If we want precise cursor on the ID:
                                // character: loc.range_start_col as u32 - 1,
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


