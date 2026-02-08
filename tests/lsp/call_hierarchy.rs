use crate::support::lsp_client::LspClient;
use crate::support::server_bin;
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;
use tokio::time::Duration;

#[tokio::test]
async fn e2e_call_hierarchy() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let root_path = dir.path().to_path_buf();

    let config_path = root_path.join("docgraph.toml");
    fs::write(
        &config_path,
        r#"
[nodes.REQ]
desc = "Requirement"
"#,
    )?;

    // A -> B
    let file_a = root_path.join("a.md");
    let uri_a = format!("file://{}", file_a.to_str().unwrap());
    fs::write(
        &file_a,
        r#"
<a id="A"></a>
Links to: [B](b.md#B)
    "#,
    )?;

    let file_b = root_path.join("b.md");
    let uri_b = format!("file://{}", file_b.to_str().unwrap());
    fs::write(
        &file_b,
        r#"
<a id="B"></a>
Target
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

    c.send_notification("textDocument/didOpen", json!({
        "textDocument": { "uri": uri_a, "languageId": "markdown", "version": 1, "text": fs::read_to_string(&file_a)? }
    })).await?;
    c.send_notification("textDocument/didOpen", json!({
        "textDocument": { "uri": uri_b, "languageId": "markdown", "version": 1, "text": fs::read_to_string(&file_b)? }
    })).await?;

    let _ = c
        .wait_notification("textDocument/publishDiagnostics", Duration::from_secs(5))
        .await?;

    // Prepare Call Hierarchy on B (incoming from A)
    let prepare_res: Value = c
        .send_request(
            "textDocument/prepareCallHierarchy",
            json!({
                "textDocument": {"uri": uri_b},
                "position": {"line": 1, "character": 8}
            }),
        )
        .await?;

    let items = prepare_res
        .get("result")
        .and_then(|r| r.as_array())
        .expect("Prepare call hierarchy failed");
    let item = &items[0];

    // Incoming calls
    let incoming_res: Value = c
        .send_request(
            "callHierarchy/incomingCalls",
            json!({
                "item": item
            }),
        )
        .await?;

    let calls = incoming_res
        .get("result")
        .and_then(|r| r.as_array())
        .expect("Incoming calls failed");
    assert!(!calls.is_empty(), "Should have incoming calls");

    let from_a = calls.iter().any(|call| {
        call.get("from")
            .and_then(|f| f.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            == "A"
    });
    assert!(from_a, "Should see call from A");

    c.send_request("shutdown", json!({})).await?;
    c.send_notification("exit", json!({})).await?;
    Ok(())
}
