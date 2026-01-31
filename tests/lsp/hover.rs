use crate::support::lsp_client::LspClient;
use crate::support::server_bin;
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;
use tokio::time::Duration;

#[tokio::test]
async fn e2e_hover() -> anyhow::Result<()> {
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
# Known Block
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

    let hover_res: Value = c
        .send_request(
            "textDocument/hover",
            json!({
                "textDocument": {"uri": file_uri},
                "position": {"line": 1, "character": 8}
            }),
        )
        .await?;

    let hover_contents = hover_res
        .get("result")
        .and_then(|r| r.get("contents"))
        .unwrap();
    if let Some(val) = hover_contents.get("value") {
        let s: &str = val.as_str().unwrap();
        assert!(s.contains("REQ-001"));
    }

    c.send_request("shutdown", json!({})).await?;
    c.send_notification("exit", json!({})).await?;
    Ok(())
}
