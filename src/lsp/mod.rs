mod backend;
mod handlers;
mod uri_ext;

pub use backend::Backend;
pub use uri_ext::{uri_from_file_path, UriExt};

use lsp_server::Connection;
use lsp_types::{
    InitializeParams, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};

pub fn run_server() {
    eprintln!("Starting docgraph LSP server...");

    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger shutdown request).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        completion_provider: Some(lsp_types::CompletionOptions {
            trigger_characters: Some(vec!["[".to_string(), "#".to_string(), "(".to_string()]),
            ..Default::default()
        }),
        references_provider: Some(lsp_types::OneOf::Left(true)),
        rename_provider: Some(lsp_types::OneOf::Left(true)),
        call_hierarchy_provider: Some(lsp_types::CallHierarchyServerCapability::Simple(true)),
        document_symbol_provider: Some(lsp_types::OneOf::Left(true)),
        workspace_symbol_provider: Some(lsp_types::OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join().unwrap();
            }
            return;
        }
    };

    let params: InitializeParams = serde_json::from_value(initialization_params).unwrap();

    let backend = Backend::new(
        connection.sender.clone(),
        params.root_uri.clone(),
        params.workspace_folders.clone(),
    );

    backend.run_main_loop(connection, params).unwrap();

    io_threads.join().unwrap();
    eprintln!("Shutting down server");
}
