use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::PathBuf;

use crate::{config, lint, types};

pub struct Backend {
    client: Client,
    workspace_root: Arc<Mutex<Option<PathBuf>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            workspace_root: Arc::new(Mutex::new(None)),
        }
    }

    async fn run_lint(&self) {
        let root_opt = self.workspace_root.lock().await.clone();
        if let Some(root) = root_opt {
            let config = config::Config::load(&root).unwrap_or_else(|_| config::Config::default());
            let diagnostics = lint::check_workspace(&root, false, None, true, &config);

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

            // TODO: Clear diagnostics for files that are no longer reporting errors?
            // For now, just publish what we have.
            for (path, diags) in file_diagnostics {
                let uri = Url::from_file_path(path).unwrap();
                self.client.publish_diagnostics(uri, diags, None).await;
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
        // Clear diagnostics on close?
        self.client.publish_diagnostics(params.text_document.uri, vec![], None).await;
    }
}

pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
