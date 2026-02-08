use crate::support::lsp_client::LspClient;
use crate::support::server_bin;
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;
use tokio::time::Duration;

#[tokio::test]
async fn e2e_rename() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let root_path = dir.path().canonicalize()?;

    let config_path = root_path.join("docgraph.toml");
    fs::write(
        &config_path,
        r#"[nodes.REQ]
desc = "Requirement"
"#,
    )?;

    // A defines REQ-001
    let file_a = root_path.join("a.md");
    let uri_a = format!("file://{}", file_a.to_str().unwrap());
    fs::write(&file_a, r#"<a id="REQ-001"></a>"#)?;

    // B references REQ-001
    let file_b = root_path.join("b.md");
    let uri_b = format!("file://{}", file_b.to_str().unwrap());
    fs::write(&file_b, r#"[REQ-001](a.md#REQ-001)"#)?;

    let mut c = LspClient::spawn(&server_bin(), &["lsp"]).await?;

    c.send_request(
        "initialize",
        json!({
            "processId": null,
            "rootUri": format!("file://{}", root_path.to_str().unwrap()),
            "capabilities": {
                "textDocument": { "rename": { "prepareSupport": true } }
            }
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

    // Rename REQ-001 to REQ-999 in file A
    let rename_res: Value = c
        .send_request(
            "textDocument/rename",
            json!({
                "textDocument": {"uri": uri_a},
                "position": {"line": 0, "character": 8},
                "newName": "REQ-999"
            }),
        )
        .await?;

    let edits = rename_res
        .get("result")
        .and_then(|r| r.get("changes"))
        .expect("Rename should return WorkspaceEdit changes");

    // Check the edits
    let edits_obj = edits.as_object().unwrap();
    let edits_a = edits_obj.get(&uri_a).unwrap().as_array().unwrap();
    let edits_b = edits_obj.get(&uri_b).unwrap().as_array().unwrap();

    // A: <a id="REQ-001"></a> -> <a id="REQ-999"></a>
    // We expect one edit replacing "REQ-001"
    assert_eq!(edits_a.len(), 1);
    assert_eq!(edits_a[0].get("newText").unwrap(), "REQ-999");

    // B: [REQ-001](a.md#REQ-001) -> [REQ-999](a.md#REQ-999)
    assert!(!edits_b.is_empty());
    for edit in edits_b {
        assert_eq!(edit.get("newText").unwrap(), "REQ-999");
    }

    c.send_request("shutdown", json!({})).await?;
    c.send_notification("exit", json!({})).await?;
    Ok(())
}

#[tokio::test]
async fn e2e_rename_from_reference() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let root_path = dir.path().canonicalize()?;

    let config_path = root_path.join("docgraph.toml");
    fs::write(
        &config_path,
        r#"[nodes.REQ]
desc = "Requirement"
"#,
    )?;

    let file_a = root_path.join("a.md");
    let uri_a = format!("file://{}", file_a.to_str().unwrap());
    fs::write(&file_a, r#"<a id="REQ-001"></a>"#)?;

    let file_b = root_path.join("b.md");
    let uri_b = format!("file://{}", file_b.to_str().unwrap());
    fs::write(
        &file_b,
        r#"
Reference: [REQ-001](a.md#REQ-001)
    "#,
    )?;

    let mut c = LspClient::spawn(&server_bin(), &["lsp"]).await?;

    c.send_request(
        "initialize",
        json!({
            "processId": null,
            "rootUri": format!("file://{}", root_path.to_str().unwrap()),
            "capabilities": { "textDocument": { "rename": { "prepareSupport": true } } }
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

    // Rename from B (Reference)
    // [REQ-001] starts at line 1, char 12
    let rename_res: Value = c
        .send_request(
            "textDocument/rename",
            json!({
                "textDocument": {"uri": uri_b},
                "position": {"line": 1, "character": 15},
                "newName": "REQ-888"
            }),
        )
        .await?;

    let edits = rename_res
        .get("result")
        .and_then(|r| r.get("changes"))
        .expect("Rename from ref should return changes");
    let edits_obj = edits.as_object().unwrap();

    assert!(edits_obj.contains_key(&uri_a));
    assert!(edits_obj.contains_key(&uri_b));

    c.send_request("shutdown", json!({})).await?;
    c.send_notification("exit", json!({})).await?;
    Ok(())
}
