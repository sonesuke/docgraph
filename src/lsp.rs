use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::PathBuf;

use crate::{config, lint, types, collect};

pub struct Backend {
    client: Client,
    workspace_root: Arc<Mutex<Option<PathBuf>>>,
    blocks: Arc<Mutex<Vec<types::SpecBlock>>>,
    standalone_refs: Arc<Mutex<Vec<types::RefUse>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            workspace_root: Arc::new(Mutex::new(None)),
            blocks: Arc::new(Mutex::new(Vec::new())),
            standalone_refs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn run_lint(&self) {
        let root_opt = self.workspace_root.lock().await.clone();
        if let Some(root) = root_opt {
            let config = config::Config::load(&root).unwrap_or_else(|_| config::Config::default());
            let diagnostics = lint::check_workspace(&root, false, None, true, &config);

            // Update index
            let (blocks, refs) = collect::collect_workspace_all(&root);
            {
                let mut b = self.blocks.lock().await;
                *b = blocks;
                let mut r = self.standalone_refs.lock().await;
                *r = refs;
            }

            // Group diagnostics by file path
            let mut file_diagnostics: std::collections::HashMap<PathBuf, Vec<Diagnostic>> = std::collections::HashMap::new();
            
            for d in diagnostics {
                let diag = Diagnostic {
                    range: Range {
                        start: Position {
                            line: d.range.start_line as u32 - 1,
                            character: d.range.start_col as u32 - 1,
                        },
                        end: Position {
                            line: d.range.end_line as u32 - 1,
                            character: d.range.end_col as u32 - 1,
                        },
                    },
                    severity: Some(match d.severity {
                        types::Severity::Error => DiagnosticSeverity::ERROR,
                        types::Severity::Warning => DiagnosticSeverity::WARNING,
                    }),
                    code: Some(NumberOrString::String(d.code)),
                    source: Some("docgraph".to_string()),
                    message: d.message,
                    ..Default::default()
                };
                file_diagnostics.entry(d.path).or_default().push(diag);
            }

            // For simplicity, always publish current diagnostics. 
            // In a real server, we might want to clear old ones.
            for (path, diags) in file_diagnostics {
                if let Ok(uri) = Url::from_file_path(path) {
                    self.client.publish_diagnostics(uri, diags, None).await;
                }
            }
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let mut root = self.workspace_root.lock().await;
        if let Some(uri) = params.root_uri {
            if let Ok(path) = uri.to_file_path() {
                *root = Some(path);
            }
        } else if let Some(folders) = params.workspace_folders {
            if let Some(folder) = folders.first() {
                if let Ok(path) = folder.uri.to_file_path() {
                    *root = Some(path);
                }
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["[".to_string(), "#".to_string(), "(".to_string()]),
                    ..Default::default()
                }),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "docgraph language server initialized")
            .await;
        self.run_lint().await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, _: DidOpenTextDocumentParams) {
        self.run_lint().await;
    }

    async fn did_change(&self, _: DidChangeTextDocumentParams) {
        self.run_lint().await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.run_lint().await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client.publish_diagnostics(params.text_document.uri, vec![], None).await;
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let line = position.line as usize + 1;
        let col = position.character as usize + 1;

        if let Ok(path) = uri.to_file_path() {
            let blocks = self.blocks.lock().await;
            let refs = self.standalone_refs.lock().await;

            // Find if cursor is on a reference
            let mut target_id = None;

            // 1. Check scoped edges
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

            // 2. Check standalone refs
            if target_id.is_none() {
                for r in refs.iter() {
                    if r.file_path == path && r.line == line && col >= r.col_start && col <= r.col_end {
                        target_id = Some(r.target_id.clone());
                        break;
                    }
                }
            }

            if let Some(id) = target_id {
                // Find definition of target_id
                if let Some(target_block) = blocks.iter().find(|b| b.id == id) {
                    if let Ok(target_uri) = Url::from_file_path(&target_block.file_path) {
                        return Ok(Some(GotoDefinitionResponse::Scalar(Location {
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
                        })));
                    }
                }
            }
        }

        Ok(None)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let line = position.line as usize + 1;
        let col = position.character as usize + 1;

        if let Ok(path) = uri.to_file_path() {
            let blocks = self.blocks.lock().await;
            let refs = self.standalone_refs.lock().await;

            let mut target_id = None;

            // Find if on link
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

            // Find if on definition itself
            if target_id.is_none() {
                for block in blocks.iter() {
                    if block.file_path == path && block.line_start == line {
                        // Approximate ID check on anchor line
                        target_id = Some(block.id.clone());
                        break;
                    }
                }
            }

            if let Some(id) = target_id {
                if let Some(target_block) = blocks.iter().find(|b| b.id == id) {
                    let title = target_block.name.as_deref().unwrap_or(&id);
                    let mut markdown = format!("**{}** ({})", title, id);
                    
                    // Add references summary
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

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let blocks = self.blocks.lock().await;
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

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let line = position.line as usize + 1;
        let col = position.character as usize + 1;

        if let Ok(path) = uri.to_file_path() {
            let blocks = self.blocks.lock().await;
            let refs = self.standalone_refs.lock().await;

            let mut target_id = None;

            // Find what we are looking for
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
                
                // Collect all uses
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

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;
        let line = position.line as usize + 1;
        let _col = position.character as usize + 1;

        if let Ok(path) = uri.to_file_path() {
            let blocks = self.blocks.lock().await;
            let refs = self.standalone_refs.lock().await;

            let mut target_id = None;

            // Find target ID
            for block in blocks.iter() {
                if block.file_path == path && block.line_start == line {
                    target_id = Some(block.id.clone());
                    break;
                }
            }

            if let Some(id) = target_id {
                let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> = std::collections::HashMap::new();

                // 1. Update the definition (anchor)
                for block in blocks.iter() {
                    if block.id == id {
                        if let Ok(u) = Url::from_file_path(&block.file_path) {
                            // We need to find the exact ID in the anchor line.
                            // Currently we only have line_start.
                            // Assume <a id="ID"></a> format and find ID.
                            // For now, let's just do a simple replacement if we can read the file.
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

                // 2. Update all edge references
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

                // 3. Update all standalone references
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

                return Ok(Some(WorkspaceEdit {
                    changes: Some(changes),
                    ..Default::default()
                }));
            }
        }

        Ok(None)
    }
}

pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
