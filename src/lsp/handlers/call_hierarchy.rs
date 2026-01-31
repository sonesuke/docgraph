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

        // Delegate to Core
        if let Some(target_id) =
            crate::core::locate::locate_id_at_position(&blocks, &[], &path, line, col)
            && let Some(target_block) = blocks.iter().find(|b| b.id == target_id)
            && let Ok(target_uri) = Url::from_file_path(&target_block.file_path)
        {
            return Ok(Some(vec![CallHierarchyItem {
                name: target_block
                    .name
                    .clone()
                    .unwrap_or_else(|| target_id.clone()),
                kind: SymbolKind::INTERFACE,
                tags: None,
                detail: Some(format!("Ref count: {}", target_block.edges.len())),
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
                data: Some(serde_json::to_value(target_id).unwrap()),
            }]));
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

    // Delegate to Core
    // Incoming calls = References to this ID
    // We reuse find_references_msg but filter for "Edges" (callers from other blocks)
    // Actually find_references_msg returns locations. We need to map back to Blocks.
    // Ideally Core should provide "find_incoming_edges(target_id) -> Vec<(SourceBlock, Range)>"
    // For now, let's reuse find_references_msg and map manually (Thin Handler Logic).

    let locations = crate::core::locate::find_references_msg(&blocks, &[], &target_id);
    let mut calls = Vec::new();

    for loc in locations {
        // Find which block contains this location
        // This is "reverse lookup" - identifying the caller.
        // Core doesn't give us the "Source Block" in LocateResult directly yet (just file path).
        // We need to find the block at that location.
        // Or we can just iterate blocks like before.
        // Let's implement it "thinly" by iterating blocks again? No, that's duplicative.
        // Ideally 'find_references_msg' or a new 'find_incoming_relations' returns the relationship.

        // Let's stick to the existing logic pattern but using core helpers if possible.
        // Attempting to match `loc` back to a block:
        if let Some(source_block) = blocks.iter().find(|b| {
            b.file_path == loc.file_path
                && loc.range_start_line >= b.line_start
                && loc.range_end_line <= b.line_end
        }) {
            // Exclude self-references if necessary? Usually Call Hierarchy includes them.
            // We need to make sure this location corresponds to an EDGE (outgoing call from source), not a definition or ref.
            // `find_references_msg` usage:
            // 1. Definition (loc matches source_block definition) -> Skip (it's the item itself)
            // 2. Edge (loc is inside source_block) -> Include
            // 3. Ref -> Skip (Docgraph doesn't treat Refs as "calls" in hierarchy usually, or maybe it should?)

            // Check if it's an edge
            let is_edge = source_block
                .edges
                .iter()
                .any(|e| e.line == loc.range_start_line && e.col_start == loc.range_start_col);

            if is_edge && let Ok(u) = Url::from_file_path(&source_block.file_path) {
                calls.push(CallHierarchyIncomingCall {
                    from: CallHierarchyItem {
                        name: source_block
                            .name
                            .clone()
                            .unwrap_or_else(|| source_block.id.clone()),
                        kind: SymbolKind::INTERFACE,
                        tags: None,
                        detail: None,
                        uri: u,
                        range: Range {
                            start: Position {
                                line: source_block.line_start as u32 - 1,
                                character: 0,
                            },
                            end: Position {
                                line: source_block.line_start as u32 - 1,
                                character: 0,
                            },
                        },
                        selection_range: Range {
                            start: Position {
                                line: source_block.line_start as u32 - 1,
                                character: 0,
                            },
                            end: Position {
                                line: source_block.line_start as u32 - 1,
                                character: 0,
                            },
                        },
                        data: Some(serde_json::to_value(&source_block.id).unwrap()),
                    },
                    from_ranges: vec![Range {
                        start: Position {
                            line: loc.range_start_line as u32 - 1,
                            character: loc.range_start_col as u32 - 1,
                        },
                        end: Position {
                            line: loc.range_end_line as u32 - 1,
                            character: loc.range_end_col as u32 - 1,
                        },
                    }],
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

    // Delegate to Core
    let _outgoing_edges = crate::core::locate::find_outgoing_edges(&blocks, &source_id);
    let mut calls = Vec::new();

    // We need to group ranges by Target ID.
    // But `find_outgoing_edges` currently just returns locations in the Source file.
    // It doesn't tell us WHICH ID is targeted (LocateResult loses that info).
    // I need to update `find_outgoing_edges` or iterate blocks here.
    // Since `find_outgoing_edges` implementation I wrote earlier just iterates edges,
    // I should probably have returned `(String, LocateResult)` or similar.

    // Re-implementing correctly using Core's data:
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
            if let Some(target_block) = blocks.iter().find(|b| b.id == target_id)
                && let Ok(u) = Url::from_file_path(&target_block.file_path)
            {
                calls.push(CallHierarchyOutgoingCall {
                    to: CallHierarchyItem {
                        name: target_block
                            .name
                            .clone()
                            .unwrap_or_else(|| target_id.clone()),
                        kind: SymbolKind::INTERFACE,
                        tags: None,
                        detail: None,
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

    Ok(Some(calls))
}
