pub mod lsp_client;

pub fn server_bin() -> String {
    env!("CARGO_BIN_EXE_docgraph").to_string()
}
