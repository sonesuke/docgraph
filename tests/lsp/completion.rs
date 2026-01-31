use crate::support::lsp_client::LspClient;
use crate::support::server_bin;
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;
use tokio::time::Duration;

#[tokio::test]
async fn e2e_completion() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let root_path = dir.path().to_path_buf();

    let config_path = root_path.join("docgraph.toml");
    fs::write(&config_path, r#"[graph]"#)?;

    let file_path = root_path.join("test.md");
    let file_uri = format!("file://{}", file_path.to_str().unwrap());
    fs::write(
        &file_path,
        r#"
<a id="REQ-001"></a>
# Target

[REQ
    "#,
    )?;

    let mut c = LspClient::spawn(&server_bin(), &["lsp"]).await?;

    c.send_request(
        "initialize",
        json!({
            "processId": null,
            "rootUri": format!("file://{}", root_path.to_str().unwrap()),
            "capabilities": {}
        }),
    )
    .await?;
    c.send_notification("initialized", json!({})).await?;

    c.send_notification(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": file_uri,
                "languageId": "markdown",
                "version": 1,
                "text": fs::read_to_string(&file_path)?
            }
        }),
    )
    .await?;

    let _ = c
        .wait_notification("textDocument/publishDiagnostics", Duration::from_secs(5))
        .await?;

    // Trigger completion after `[REQ` (line 4, col 4)
    let comp_res: Value = c
        .send_request(
            "textDocument/completion",
            json!({
                "textDocument": {"uri": file_uri},
                "position": {"line": 4, "character": 4}
            }),
        )
        .await?;

    let items = comp_res
        .get("result")
        .and_then(|r| r.get("items"))
        .or_else(|| comp_res.get("result"))
        .and_then(|r| r.as_array())
        .expect("Completion items not found");

    let found = items
        .iter()
        .any(|item| item.get("label").and_then(|l| l.as_str()).unwrap_or("") == "REQ-001");
    assert!(found, "Should find REQ-001 in completion candidates");

    c.send_request("shutdown", json!({})).await?;
    c.send_notification("exit", json!({})).await?;
    Ok(())
}
