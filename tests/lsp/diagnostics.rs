use crate::support::lsp_client::LspClient;
use crate::support::server_bin;
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;
use tokio::time::Duration;

#[tokio::test]
async fn e2e_diagnostics_and_hover() -> anyhow::Result<()> {
    // 1. Setup workspace
    let dir = tempdir()?;
    let root_path = dir.path().to_path_buf();

    // Create config
    let config_path = root_path.join("docgraph.toml");
    fs::write(
        &config_path,
        r#"
[graph]
strict_node_types = true
[node_types.REQ]
desc = "Requirement"
    "#,
    )?;

    // Create a file with an error (unknown prefix UNK)
    let file_path = root_path.join("test.md");
    let file_uri = format!("file://{}", file_path.to_str().unwrap());
    fs::write(
        &file_path,
        r#"
<a id="UNK-001"></a>
# Unknown Block

<a id="REQ-001"></a>
# Known Block

This is a test.
    "#,
    )?;

    // 2. Spawn Client
    let mut c: LspClient = LspClient::spawn(&server_bin(), &["lsp"]).await?;

    // 3. Initialize
    let init_params = json!({
        "processId": null,
        "rootUri": format!("file://{}", root_path.to_str().unwrap()),
        "capabilities": {
            "textDocument": {
                "publishDiagnostics": {},
                "hover": {}
            }
        }
    });

    let init_res: Value = c.send_request("initialize", init_params).await?;
    assert!(init_res.get("result").is_some());

    c.send_notification("initialized", json!({})).await?;

    // 4. Open Document
    let text = fs::read_to_string(&file_path)?;
    c.send_notification(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": file_uri,
                "languageId": "markdown",
                "version": 1,
                "text": text
            }
        }),
    )
    .await?;

    // 5. Wait for Diagnostics (DG005 should fire for UNK-001)
    let diag_msg: Value = c
        .wait_notification("textDocument/publishDiagnostics", Duration::from_secs(5))
        .await?;
    let diags = diag_msg
        .get("params")
        .and_then(|p| p.get("diagnostics"))
        .and_then(|d| d.as_array())
        .ok_or_else(|| anyhow::anyhow!("No diagnostics found"))?;

    assert!(!diags.is_empty(), "Expected diagnostics, got empty");

    let has_unk_error = diags.iter().any(|d| {
        d.get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .contains("Unknown node type prefix")
    });

    assert!(has_unk_error, "Expected DG005 error for UNK-001");

    // 7. Shutdown
    c.send_request("shutdown", json!({})).await?;
    c.send_notification("exit", json!({})).await?;

    Ok(())
}
