use anyhow::Result;
use lsp_types::*;

pub fn document_symbol(
    blocks: &[crate::core::types::SpecBlock],
    params: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>> {
    let uri = params.text_document.uri;
    if let Ok(path) = uri.to_file_path() {
        let path = std::fs::canonicalize(&path).unwrap_or(path);
        let symbols: Vec<DocumentSymbol> = blocks
            .iter()
            .filter(|b| b.file_path == path)
            .map(|b| {
                let range = Range {
                    start: Position {
                        line: b.line_start as u32 - 1,
                        character: 0,
                    },
                    end: Position {
                        line: b.line_end as u32 - 1,
                        character: 1000, // Represent the whole line
                    },
                };

                let name = if let Some(n) = &b.name {
                    format!("{} ({})", b.id, n)
                } else {
                    b.id.clone()
                };
                let detail = b.name.clone();

                #[allow(deprecated)]
                DocumentSymbol {
                    name,
                    detail,
                    kind: SymbolKind::FIELD,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                }
            })
            .collect();

        if symbols.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DocumentSymbolResponse::Nested(symbols)))
        }
    } else {
        Ok(None)
    }
}

pub fn workspace_symbol(
    blocks: &[crate::core::types::SpecBlock],
    params: WorkspaceSymbolParams,
) -> Result<Option<Vec<SymbolInformation>>> {
    let query = params.query.to_lowercase();
    let search_term = query.trim_start_matches('#');

    let symbols: Vec<SymbolInformation> = blocks
        .iter()
        .filter(|b| {
            b.id.to_lowercase().contains(search_term)
                || b.name
                    .as_deref()
                    .map(|n| n.to_lowercase().contains(search_term))
                    .unwrap_or(false)
        })
        .filter_map(|b| {
            let uri = Url::from_file_path(&b.file_path).ok()?;
            let range = Range {
                start: Position {
                    line: b.line_start as u32 - 1,
                    character: 0,
                },
                end: Position {
                    line: b.line_end as u32 - 1,
                    character: 1000,
                },
            };

            let name = if let Some(n) = &b.name {
                format!("{} ({})", b.id, n)
            } else {
                b.id.clone()
            };

            #[allow(deprecated)]
            Some(SymbolInformation {
                name,
                kind: SymbolKind::CONSTANT,
                tags: None,
                deprecated: None,
                location: Location { uri, range },
                container_name: None,
            })
        })
        .collect();

    if symbols.is_empty() {
        Ok(None)
    } else {
        Ok(Some(symbols))
    }
}
