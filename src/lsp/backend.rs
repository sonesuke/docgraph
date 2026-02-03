use dashmap::DashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use super::handlers;
use crate::core::{collect, config, lint, types};

pub struct Backend {
    pub client: Client,
    pub workspace_root: Arc<Mutex<Option<PathBuf>>>,
    pub blocks: Arc<Mutex<Vec<types::SpecBlock>>>,
    pub standalone_refs: Arc<Mutex<Vec<types::RefUse>>>,
    pub documents: Arc<DashMap<Url, String>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            workspace_root: Arc::new(Mutex::new(None)),
            blocks: Arc::new(Mutex::new(Vec::new())),
            standalone_refs: Arc::new(Mutex::new(Vec::new())),
            documents: Arc::new(DashMap::new()),
        }
    }

    pub async fn run_lint(&self) {
        let root_opt = self.workspace_root.lock().await.clone();
        if let Some(root) = root_opt {
            let config = config::Config::load(&root).unwrap_or_else(|_| config::Config::default());

            // Create overrides map (convert DashMap<Url, String> to HashMap<PathBuf, String>)
            // This decouples core from lsp types
            let mut overrides = std::collections::HashMap::new();
            for entry in self.documents.iter() {
                if let Ok(path) = entry.key().to_file_path() {
                    // Try to canonicalize the path for consistent lookup
                    if let Ok(canon_path) = std::fs::canonicalize(&path) {
                        overrides.insert(canon_path, entry.value().clone());
                    } else {
                        overrides.insert(path, entry.value().clone());
                    }
                }
            }

            // Use documents map for overrides (pass reference to HashMap)
            let diagnostics =
                lint::check_workspace(&root, false, None, true, &config, Some(&overrides));

            // Update index
            let (blocks, refs) =
                collect::collect_workspace_all(&root, &config.graph.ignore, Some(&overrides));
            {
                let mut b = self.blocks.lock().await;
                *b = blocks;
                let mut r = self.standalone_refs.lock().await;
                *r = refs;
            }

            // Group diagnostics by file path
            let mut file_diagnostics: std::collections::HashMap<PathBuf, Vec<Diagnostic>> =
                std::collections::HashMap::new();

            // Initialize with all workspace files + open files to ensure we clear diagnostics for everything
            let workspace_files =
                crate::core::walk::find_markdown_files(&root, &config.graph.ignore);
            for path in workspace_files {
                file_diagnostics.entry(path).or_default();
            }

            // Also ensure open files are included (though they should be in workspace_files if on disk)
            // But if they are new untitled files or outside root? (LSP usually handles workspace files)
            for entry in self.documents.iter() {
                if let Ok(path) = entry.key().to_file_path() {
                    file_diagnostics.entry(path).or_default();
                }
            }

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
        } else if let Some(folders) = params.workspace_folders
            && let Some(folder) = folders.first()
            && let Ok(path) = folder.uri.to_file_path()
        {
            *root = Some(path);
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        "[".to_string(),
                        "#".to_string(),
                        "(".to_string(),
                    ]),
                    ..Default::default()
                }),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
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

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.documents
            .insert(params.text_document.uri.clone(), params.text_document.text);
        self.run_lint().await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // We assume full text sync (TextDocumentSyncKind::FULL)
        if let Some(change) = params.content_changes.first() {
            self.documents
                .insert(params.text_document.uri.clone(), change.text.clone());
        }
        self.run_lint().await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.run_lint().await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri);
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let blocks = self.blocks.lock().await;
        let refs = self.standalone_refs.lock().await;
        handlers::goto_definition(&blocks, &refs, params)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let blocks = self.blocks.lock().await;
        let refs = self.standalone_refs.lock().await;
        handlers::hover(&blocks, &refs, params)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let blocks = self.blocks.lock().await;
        handlers::completion(&blocks, params)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let blocks = self.blocks.lock().await;
        let refs = self.standalone_refs.lock().await;
        handlers::references(&blocks, &refs, params)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let blocks = self.blocks.lock().await;
        let refs = self.standalone_refs.lock().await;
        handlers::rename(&blocks, &refs, params)
    }

    async fn prepare_call_hierarchy(
        &self,
        params: CallHierarchyPrepareParams,
    ) -> Result<Option<Vec<CallHierarchyItem>>> {
        handlers::prepare_call_hierarchy(self, params).await
    }

    async fn incoming_calls(
        &self,
        params: CallHierarchyIncomingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        handlers::incoming_calls(self, params).await
    }

    async fn outgoing_calls(
        &self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        handlers::outgoing_calls(self, params).await
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let blocks = self.blocks.lock().await;
        handlers::document_symbol(&blocks, params)
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        let blocks = self.blocks.lock().await;
        handlers::workspace_symbol(&blocks, params)
    }
}
