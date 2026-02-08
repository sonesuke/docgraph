use crossbeam_channel::Sender;
use dashmap::DashMap;
use lsp_server::{Connection, Message, Notification, RequestId, Response};
use lsp_types::{
    Diagnostic, InitializeParams, Position, PublishDiagnosticsParams, Range, Uri, WorkspaceFolder,
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument,
        Initialized, Notification as _, PublishDiagnostics,
    },
    request::{
        CallHierarchyIncomingCalls, CallHierarchyOutgoingCalls, CallHierarchyPrepare, Completion,
        DocumentSymbolRequest, GotoDefinition, HoverRequest, References, Rename,
        WorkspaceSymbolRequest,
    },
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use url::Url;

use super::handlers;
use crate::core::{collect, config, lint, types};

pub struct Backend {
    pub sender: Sender<Message>,
    pub workspace_root: Arc<Mutex<Option<PathBuf>>>,
    pub blocks: Arc<Mutex<Vec<types::SpecBlock>>>,
    pub standalone_refs: Arc<Mutex<Vec<types::RefUse>>>,
    pub documents: Arc<DashMap<String, String>>,
}

impl Backend {
    pub fn new(
        sender: Sender<Message>,
        root_uri: Option<Uri>,
        workspace_folders: Option<Vec<WorkspaceFolder>>,
    ) -> Self {
        let mut root = None;
        if let Some(uri) = root_uri {
            if let Ok(url) = Url::parse(uri.as_str()) 
                && let Ok(path) = url.to_file_path() {
                    root = Some(std::fs::canonicalize(&path).unwrap_or(path));
            }
        } else if let Some(folders) = workspace_folders
            && let Some(folder) = folders.first()
            && let Ok(url) = Url::parse(folder.uri.as_str())
            && let Ok(path) = url.to_file_path()
        {
            root = Some(std::fs::canonicalize(&path).unwrap_or(path));
        }

        Self {
            sender,
            workspace_root: Arc::new(Mutex::new(root)),
            blocks: Arc::new(Mutex::new(Vec::new())),
            standalone_refs: Arc::new(Mutex::new(Vec::new())),
            documents: Arc::new(DashMap::new()),
        }
    }

    pub fn run_main_loop(
        &self,
        connection: Connection,
        _params: InitializeParams,
    ) -> anyhow::Result<()> {
        for msg in &connection.receiver {
            match msg {
                Message::Request(req) => {
                    if connection.handle_shutdown(&req)? {
                        return Ok(());
                    }
                    self.dispatch_request(req)?;
                }
                Message::Notification(not) => {
                    self.dispatch_notification(not)?;
                }
                Message::Response(_) => {}
            }
        }
        Ok(())
    }

    fn dispatch_request(&self, req: lsp_server::Request) -> anyhow::Result<()> {
        match req.method.as_str() {
            "textDocument/definition" => {
                let (id, params) = cast_req::<GotoDefinition>(req)?;
                let blocks = self.blocks.lock().unwrap();
                let refs = self.standalone_refs.lock().unwrap();
                let result = handlers::goto_definition(&blocks, &refs, params)?;
                self.send_response(id, result)?;
            }
            "textDocument/hover" => {
                let (id, params) = cast_req::<HoverRequest>(req)?;
                let blocks = self.blocks.lock().unwrap();
                let refs = self.standalone_refs.lock().unwrap();
                let result = handlers::hover(&blocks, &refs, params)?;
                self.send_response(id, result)?;
            }
            "textDocument/completion" => {
                let (id, params) = cast_req::<Completion>(req)?;
                let blocks = self.blocks.lock().unwrap();
                let result = handlers::completion(&blocks, params)?;
                self.send_response(id, result)?;
            }
            "textDocument/references" => {
                let (id, params) = cast_req::<References>(req)?;
                let blocks = self.blocks.lock().unwrap();
                let refs = self.standalone_refs.lock().unwrap();
                let result = handlers::references(&blocks, &refs, params)?;
                self.send_response(id, result)?;
            }
            "textDocument/rename" => {
                let (id, params) = cast_req::<Rename>(req)?;
                let blocks = self.blocks.lock().unwrap();
                let refs = self.standalone_refs.lock().unwrap();
                let result = handlers::rename(&blocks, &refs, params)?;
                self.send_response(id, result)?;
            }
            "textDocument/prepareCallHierarchy" => {
                let (id, params) = cast_req::<CallHierarchyPrepare>(req)?;
                let result = handlers::prepare_call_hierarchy(self, params)?;
                self.send_response(id, result)?;
            }
            "callHierarchy/incomingCalls" => {
                let (id, params) = cast_req::<CallHierarchyIncomingCalls>(req)?;
                let result = handlers::incoming_calls(self, params)?;
                self.send_response(id, result)?;
            }
            "callHierarchy/outgoingCalls" => {
                let (id, params) = cast_req::<CallHierarchyOutgoingCalls>(req)?;
                let result = handlers::outgoing_calls(self, params)?;
                self.send_response(id, result)?;
            }
            "textDocument/documentSymbol" => {
                let (id, params) = cast_req::<DocumentSymbolRequest>(req)?;
                let blocks = self.blocks.lock().unwrap();
                let result = handlers::document_symbol(&blocks, params)?;
                self.send_response(id, result)?;
            }
            "workspace/symbol" => {
                let (id, params) = cast_req::<WorkspaceSymbolRequest>(req)?;
                let blocks = self.blocks.lock().unwrap();
                let result = handlers::workspace_symbol(&blocks, params)?;
                self.send_response(id, result)?;
            }
            _ => {
                eprintln!("Method not found: {}", req.method);
            }
        }
        Ok(())
    }

    fn dispatch_notification(&self, not: Notification) -> anyhow::Result<()> {
        match not.method.as_str() {
            "initialized" => {
                let _ = cast_not::<Initialized>(not)?;
                eprintln!("docgraph language server initialized");
                self.run_lint();
            }
            "textDocument/didOpen" => {
                let params = cast_not::<DidOpenTextDocument>(not)?;
                self.documents
                    .insert(params.text_document.uri.to_string(), params.text_document.text);
                self.run_lint();
            }
            "textDocument/didChange" => {
                let params = cast_not::<DidChangeTextDocument>(not)?;
                // We assume full text sync
                if let Some(change) = params.content_changes.first() {
                    self.documents
                        .insert(params.text_document.uri.to_string(), change.text.clone());
                }
                self.run_lint();
            }
            "textDocument/didSave" => {
                let _ = cast_not::<DidSaveTextDocument>(not)?;
                self.run_lint();
            }
            "textDocument/didClose" => {
                let params = cast_not::<DidCloseTextDocument>(not)?;
                self.documents.remove(params.text_document.uri.as_str());
                self.publish_diagnostics(params.text_document.uri, vec![])?;
            }
            _ => {}
        }
        Ok(())
    }

    fn send_response<T: serde::Serialize>(&self, id: RequestId, result: T) -> anyhow::Result<()> {
        let result = serde_json::to_value(&result).unwrap();
        let resp = Response {
            id,
            result: Some(result),
            error: None,
        };
        self.sender.send(Message::Response(resp))?;
        Ok(())
    }

    fn publish_diagnostics(&self, uri: Uri, diagnostics: Vec<Diagnostic>) -> anyhow::Result<()> {
        let params = PublishDiagnosticsParams {
            uri,
            diagnostics,
            version: None,
        };
        let not = Notification::new(PublishDiagnostics::METHOD.to_string(), params);
        self.sender.send(Message::Notification(not))?;
        Ok(())
    }

    pub fn run_lint(&self) {
        let root_opt = self.workspace_root.lock().unwrap().clone();
        if let Some(root) = root_opt {
            let config = config::Config::load(&root).unwrap_or_else(|_| config::Config::default());

            // Create overrides map (convert DashMap<String, String> to HashMap<PathBuf, String>)
            let mut overrides = std::collections::HashMap::new();
            for entry in self.documents.iter() {
                if let Ok(url) = Url::parse(entry.key()) 
                    && let Ok(path) = url.to_file_path() {
                        // Try to canonicalize the path for consistent lookup
                        if let Ok(canon_path) = std::fs::canonicalize(&path) {
                            overrides.insert(canon_path, entry.value().clone());
                        } else {
                            overrides.insert(path, entry.value().clone());
                        }
                }
            }

            // Lint
            let diagnostics =
                lint::check_workspace(&root, false, None, true, &config, Some(&overrides));

            // Update index
            let (blocks, refs) =
                collect::collect_workspace_all(&root, &config.graph.ignore, Some(&overrides));
            {
                let mut b = self.blocks.lock().unwrap();
                *b = blocks;
                let mut r = self.standalone_refs.lock().unwrap();
                *r = refs;
            }

            // Group diagnostics by file path
            let mut file_diagnostics: std::collections::HashMap<PathBuf, Vec<Diagnostic>> =
                std::collections::HashMap::new();

            // Initialize with all workspace files
            let workspace_files =
                crate::core::walk::find_markdown_files(&root, &config.graph.ignore);
            for path in workspace_files {
                file_diagnostics.entry(path).or_default();
            }
            // Also open files
            for entry in self.documents.iter() {
                if let Ok(url) = Url::parse(entry.key()) 
                    && let Ok(path) = url.to_file_path() {
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
                        types::Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
                        types::Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
                    }),
                    code: Some(lsp_types::NumberOrString::String(d.code)),
                    source: Some("docgraph".to_string()),
                    message: d.message,
                    ..Default::default()
                };
                file_diagnostics.entry(d.path).or_default().push(diag);
            }

            for (path, diags) in file_diagnostics {
                if let Ok(url) = Url::from_file_path(path) 
                    && let Ok(uri) = url.as_str().parse::<Uri>() {
                        let _ = self.publish_diagnostics(uri, diags);
                }
            }
        }
    }
}

fn cast_req<R>(req: lsp_server::Request) -> anyhow::Result<(RequestId, R::Params)>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
        .map_err(|e| anyhow::anyhow!("Invalid request: {:?}", e))
}

fn cast_not<N>(not: Notification) -> anyhow::Result<N::Params>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    not.extract(N::METHOD)
        .map_err(|e| anyhow::anyhow!("Invalid notification: {:?}", e))
}
