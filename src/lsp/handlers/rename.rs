use anyhow::Result;
use lsp_types::*;
use url::Url;

pub fn rename(
    blocks: &[crate::core::types::SpecBlock],
    refs: &[crate::core::types::RefUse],
    params: RenameParams,
) -> Result<Option<WorkspaceEdit>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Ok(url) = Url::parse(uri.as_str())
        && let Ok(path) = url.to_file_path()
    {
        let path = path.canonicalize().unwrap_or(path);

        // Delegate to Core Logic
        if let Some(target_id) =
            crate::core::locate::locate_id_at_position(blocks, refs, &path, line, col)
        {
            let locations = crate::core::locate::find_references_msg(blocks, refs, &target_id);

            #[allow(clippy::mutable_key_type)]
            let mut changes: std::collections::HashMap<Uri, Vec<TextEdit>> =
                std::collections::HashMap::new();

            for loc in locations {
                if let Ok(u) = Url::from_file_path(&loc.file_path)
                    && let Ok(uri) = u.as_str().parse::<Uri>()
                {
                    let edit = TextEdit {
                        range: Range {
                            start: Position {
                                line: loc.range_start_line as u32 - 1,
                                character: loc.range_start_col as u32 - 1,
                            },
                            end: Position {
                                line: loc.range_end_line as u32 - 1,
                                character: loc.range_end_col as u32 - 1,
                            },
                        },
                        new_text: params.new_name.clone(),
                    };

                    let edits = changes.entry(uri).or_default();

                    // Simple deduplication: allow if exact range doesn't exist
                    // Note: ideally we should check for overlap, but exact duplicate is the likely cause here.
                    let is_duplicate = edits.iter().any(|e| e.range == edit.range);

                    if !is_duplicate {
                        edits.push(edit);
                    }
                }
            }

            return Ok(Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }));
        }
    }
    Ok(None)
}
