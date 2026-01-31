use crate::lsp::backend::Backend;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

pub async fn prepare_call_hierarchy(
    backend: &Backend,
    params: CallHierarchyPrepareParams,
) -> Result<Option<Vec<CallHierarchyItem>>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let blocks = backend.blocks.lock().await;
        let mut target_id = None;

        for block in blocks.iter() {
            if block.file_path == path && block.line_start == line {
                target_id = Some(block.id.clone());
                break;
            }
            if block.file_path == path && line >= block.line_start && line <= block.line_end {
                for edge in &block.edges {
                    if edge.line == line && col >= edge.col_start && col <= edge.col_end {
                        target_id = Some(edge.id.clone());
                        break;
                    }
                }
            }
        }

        if let Some(id) = target_id {
            if let Some(target_block) = blocks.iter().find(|b| b.id == id) {
                if let Ok(target_uri) = Url::from_file_path(&target_block.file_path) {
                    return Ok(Some(vec![CallHierarchyItem {
                        name: target_block.name.clone().unwrap_or_else(|| id.clone()),
                        kind: SymbolKind::INTERFACE,
                        tags: None,
                        detail: Some(id.clone()),
                        uri: target_uri,
                        range: Range {
                            start: Position {
                                line: target_block.line_start as u32 - 1,
                                character: 0,
                            },
                            end: Position {
                                line: target_block.line_start as u32 - 1,
                                character: 0,
                            },
                        },
                        selection_range: Range {
                            start: Position {
                                line: target_block.line_start as u32 - 1,
                                character: 0,
                            },
                            end: Position {
                                line: target_block.line_start as u32 - 1,
                                character: 0,
                            },
                        },
                        data: Some(serde_json::to_value(id).unwrap()),
                    }]));
                }
            }
        }
    }
    Ok(None)
}

pub async fn incoming_calls(
    backend: &Backend,
    params: CallHierarchyIncomingCallsParams,
) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
    let id_val = params.item.data.clone().unwrap_or_default();
    let target_id: String = serde_json::from_value(id_val).unwrap_or_default();
    if target_id.is_empty() {
        return Ok(None);
    }

    let blocks = backend.blocks.lock().await;
    let mut calls = Vec::new();

    for block in blocks.iter() {
        let mut from_ranges = Vec::new();
        for edge in &block.edges {
            if edge.id == target_id {
                from_ranges.push(Range {
                    start: Position {
                        line: edge.line as u32 - 1,
                        character: edge.col_start as u32 - 1,
                    },
                    end: Position {
                        line: edge.line as u32 - 1,
                        character: edge.col_end as u32 - 1,
                    },
                });
            }
        }
        if !from_ranges.is_empty() {
            if let Ok(u) = Url::from_file_path(&block.file_path) {
                calls.push(CallHierarchyIncomingCall {
                    from: CallHierarchyItem {
                        name: block.name.clone().unwrap_or_else(|| block.id.clone()),
                        kind: SymbolKind::INTERFACE,
                        tags: None,
                        detail: Some(block.id.clone()),
                        uri: u,
                        range: Range {
                            start: Position {
                                line: block.line_start as u32 - 1,
                                character: 0,
                            },
                            end: Position {
                                line: block.line_start as u32 - 1,
                                character: 0,
                            },
                        },
                        selection_range: Range {
                            start: Position {
                                line: block.line_start as u32 - 1,
                                character: 0,
                            },
                            end: Position {
                                line: block.line_start as u32 - 1,
                                character: 0,
                            },
                        },
                        data: Some(serde_json::to_value(block.id.clone()).unwrap()),
                    },
                    from_ranges,
                });
            }
        }
    }
    Ok(Some(calls))
}

pub async fn outgoing_calls(
    backend: &Backend,
    params: CallHierarchyOutgoingCallsParams,
) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
    let id_val = params.item.data.clone().unwrap_or_default();
    let source_id: String = serde_json::from_value(id_val).unwrap_or_default();
    if source_id.is_empty() {
        return Ok(None);
    }

    let blocks = backend.blocks.lock().await;
    let mut calls = Vec::new();

    if let Some(source_block) = blocks.iter().find(|b| b.id == source_id) {
        let mut targets: std::collections::HashMap<String, Vec<Range>> =
            std::collections::HashMap::new();
        for edge in &source_block.edges {
            targets.entry(edge.id.clone()).or_default().push(Range {
                start: Position {
                    line: edge.line as u32 - 1,
                    character: edge.col_start as u32 - 1,
                },
                end: Position {
                    line: edge.line as u32 - 1,
                    character: edge.col_end as u32 - 1,
                },
            });
        }
        for (target_id, from_ranges) in targets {
            if let Some(target_block) = blocks.iter().find(|b| b.id == target_id) {
                if let Ok(u) = Url::from_file_path(&target_block.file_path) {
                    calls.push(CallHierarchyOutgoingCall {
                        to: CallHierarchyItem {
                            name: target_block
                                .name
                                .clone()
                                .unwrap_or_else(|| target_id.clone()),
                            kind: SymbolKind::INTERFACE,
                            tags: None,
                            detail: Some(target_id.clone()),
                            uri: u,
                            range: Range {
                                start: Position {
                                    line: target_block.line_start as u32 - 1,
                                    character: 0,
                                },
                                end: Position {
                                    line: target_block.line_start as u32 - 1,
                                    character: 0,
                                },
                            },
                            selection_range: Range {
                                start: Position {
                                    line: target_block.line_start as u32 - 1,
                                    character: 0,
                                },
                                end: Position {
                                    line: target_block.line_start as u32 - 1,
                                    character: 0,
                                },
                            },
                            data: Some(serde_json::to_value(target_id).unwrap()),
                        },
                        from_ranges,
                    });
                }
            }
        }
    }
    Ok(Some(calls))
}
