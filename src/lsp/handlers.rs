use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use super::backend::Backend;

pub async fn goto_definition(
    backend: &Backend,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
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

        if let Some(id) = target_id {
            if let Some(target_block) = blocks.iter().find(|b| b.id == id) {
                if let Ok(target_uri) = Url::from_file_path(&target_block.file_path) {
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: target_uri,
                        range: Range {
                            start: Position { line: target_block.line_start as u32 - 1, character: 0 },
                            end: Position { line: target_block.line_start as u32 - 1, character: 0 },
                        },
                    })));
                }
            }
        }
    }
    Ok(None)
}

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
                let ref_count = blocks.iter().flat_map(|b| b.edges.iter()).filter(|e| e.id == id).count() 
                               + refs.iter().filter(|r| r.target_id == id).count();
                markdown.push_str(&format!("\n\nReferenced {} times in the workspace.", ref_count));

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

pub async fn references(backend: &Backend, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let blocks = backend.blocks.lock().await;
        let refs = backend.standalone_refs.lock().await;

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
        if target_id.is_none() {
            for r in refs.iter() {
                if r.file_path == path && r.line == line && col >= r.col_start && col <= r.col_end {
                    target_id = Some(r.target_id.clone());
                    break;
                }
            }
        }

        if let Some(id) = target_id {
            let mut locations = Vec::new();
            for block in blocks.iter() {
                for edge in &block.edges {
                    if edge.id == id {
                        if let Ok(u) = Url::from_file_path(&block.file_path) {
                            locations.push(Location {
                                uri: u,
                                range: Range {
                                    start: Position { line: edge.line as u32 - 1, character: edge.col_start as u32 - 1 },
                                    end: Position { line: edge.line as u32 - 1, character: edge.col_end as u32 - 1 },
                                },
                            });
                        }
                    }
                }
            }
            for r in refs.iter() {
                if r.target_id == id {
                    if let Ok(u) = Url::from_file_path(&r.file_path) {
                        locations.push(Location {
                            uri: u,
                            range: Range {
                                start: Position { line: r.line as u32 - 1, character: r.col_start as u32 - 1 },
                                end: Position { line: r.line as u32 - 1, character: r.col_end as u32 - 1 },
                            },
                        });
                    }
                }
            }
            return Ok(Some(locations));
        }
    }
    Ok(None)
}

pub async fn rename(backend: &Backend, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let new_name = params.new_name;
    let line = position.line as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let blocks = backend.blocks.lock().await;
        let refs = backend.standalone_refs.lock().await;

        let mut target_id = None;
        for block in blocks.iter() {
            if block.file_path == path && block.line_start == line {
                target_id = Some(block.id.clone());
                break;
            }
        }

        if let Some(id) = target_id {
            let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> = std::collections::HashMap::new();
            for block in blocks.iter() {
                if block.id == id {
                    if let Ok(u) = Url::from_file_path(&block.file_path) {
                        if let Ok(content) = std::fs::read_to_string(&block.file_path) {
                            let lines: Vec<&str> = content.lines().collect();
                            if block.line_start <= lines.len() {
                                let line_content = lines[block.line_start - 1];
                                if let Some(start_idx) = line_content.find(&id) {
                                    changes.entry(u).or_default().push(TextEdit {
                                        range: Range {
                                            start: Position { line: block.line_start as u32 - 1, character: start_idx as u32 },
                                            end: Position { line: block.line_start as u32 - 1, character: (start_idx + id.len()) as u32 },
                                        },
                                        new_text: new_name.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            for block in blocks.iter() {
                for edge in &block.edges {
                    if edge.id == id {
                        if let Ok(u) = Url::from_file_path(&block.file_path) {
                            changes.entry(u).or_default().push(TextEdit {
                                range: Range {
                                    start: Position { line: edge.line as u32 - 1, character: edge.col_start as u32 - 1 },
                                    end: Position { line: edge.line as u32 - 1, character: edge.col_end as u32 - 1 },
                                },
                                new_text: new_name.clone(),
                            });
                        }
                    }
                }
            }
            for r in refs.iter() {
                if r.target_id == id {
                    if let Ok(u) = Url::from_file_path(&r.file_path) {
                        changes.entry(u).or_default().push(TextEdit {
                            range: Range {
                                start: Position { line: r.line as u32 - 1, character: r.col_start as u32 - 1 },
                                end: Position { line: r.line as u32 - 1, character: r.col_end as u32 - 1 },
                            },
                            new_text: new_name.clone(),
                        });
                    }
                }
            }
            return Ok(Some(WorkspaceEdit { changes: Some(changes), ..Default::default() }));
        }
    }
    Ok(None)
}

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
                            start: Position { line: target_block.line_start as u32 - 1, character: 0 },
                            end: Position { line: target_block.line_start as u32 - 1, character: 0 },
                        },
                        selection_range: Range {
                            start: Position { line: target_block.line_start as u32 - 1, character: 0 },
                            end: Position { line: target_block.line_start as u32 - 1, character: 0 },
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
    if target_id.is_empty() { return Ok(None); }

    let blocks = backend.blocks.lock().await;
    let mut calls = Vec::new();

    for block in blocks.iter() {
        let mut from_ranges = Vec::new();
        for edge in &block.edges {
            if edge.id == target_id {
                from_ranges.push(Range {
                    start: Position { line: edge.line as u32 - 1, character: edge.col_start as u32 - 1 },
                    end: Position { line: edge.line as u32 - 1, character: edge.col_end as u32 - 1 },
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
                            start: Position { line: block.line_start as u32 - 1, character: 0 },
                            end: Position { line: block.line_start as u32 - 1, character: 0 },
                        },
                        selection_range: Range {
                            start: Position { line: block.line_start as u32 - 1, character: 0 },
                            end: Position { line: block.line_start as u32 - 1, character: 0 },
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
    if source_id.is_empty() { return Ok(None); }

    let blocks = backend.blocks.lock().await;
    let mut calls = Vec::new();

    if let Some(source_block) = blocks.iter().find(|b| b.id == source_id) {
        let mut targets: std::collections::HashMap<String, Vec<Range>> = std::collections::HashMap::new();
        for edge in &source_block.edges {
            targets.entry(edge.id.clone()).or_default().push(Range {
                start: Position { line: edge.line as u32 - 1, character: edge.col_start as u32 - 1 },
                end: Position { line: edge.line as u32 - 1, character: edge.col_end as u32 - 1 },
            });
        }
        for (target_id, from_ranges) in targets {
            if let Some(target_block) = blocks.iter().find(|b| b.id == target_id) {
                if let Ok(u) = Url::from_file_path(&target_block.file_path) {
                    calls.push(CallHierarchyOutgoingCall {
                        to: CallHierarchyItem {
                            name: target_block.name.clone().unwrap_or_else(|| target_id.clone()),
                            kind: SymbolKind::INTERFACE,
                            tags: None,
                            detail: Some(target_id.clone()),
                            uri: u,
                            range: Range {
                                start: Position { line: target_block.line_start as u32 - 1, character: 0 },
                                end: Position { line: target_block.line_start as u32 - 1, character: 0 },
                            },
                            selection_range: Range {
                                start: Position { line: target_block.line_start as u32 - 1, character: 0 },
                                end: Position { line: target_block.line_start as u32 - 1, character: 0 },
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
