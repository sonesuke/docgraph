use crate::support::lsp_client::LspClient;
use crate::support::server_bin;
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;
use tokio::time::Duration;

#[tokio::test]
async fn e2e_definition_and_references() -> anyhow::Result<()> {
    // 1. Setup workspace with 2 files
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

    // File A: Defines REQ-001
    let file_a_path = root_path.join("a.md");
    let file_a_uri = format!("file://{}", file_a_path.to_str().unwrap());
    fs::write(
        &file_a_path,
        r#"
<a id="REQ-001"></a>
# Requirement 1
    "#,
    )?;

    // File B: References REQ-001
    let file_b_path = root_path.join("b.md");
    let file_b_uri = format!("file://{}", file_b_path.to_str().unwrap());
    fs::write(
        &file_b_path,
        r#"
References: [REQ-001](a.md#REQ-001)
    "#,
    )?;

    // 2. Spawn Client
    let mut c: LspClient = LspClient::spawn(&server_bin(), &["lsp"]).await?;

    // 3. Initialize
    let init_params = json!({
        "processId": null,
        "rootUri": format!("file://{}", root_path.to_str().unwrap()),
        "capabilities": {}
    });
    c.send_request("initialize", init_params).await?;
    c.send_notification("initialized", json!({})).await?;

    // Open both files
    c.send_notification("textDocument/didOpen", json!({
        "textDocument": { "uri": file_a_uri, "languageId": "markdown", "version": 1, "text": fs::read_to_string(&file_a_path)? }
    })).await?;
    c.send_notification("textDocument/didOpen", json!({
        "textDocument": { "uri": file_b_uri, "languageId": "markdown", "version": 1, "text": fs::read_to_string(&file_b_path)? }
    })).await?;

    // Wait for diagnostics on B
    let _ = c
        .wait_notification("textDocument/publishDiagnostics", Duration::from_secs(5))
        .await?;

    // 4. Definition Test
    let def_res: Value = c
        .send_request(
            "textDocument/definition",
            json!({
                "textDocument": {"uri": file_b_uri},
                "position": {"line": 1, "character": 13}
            }),
        )
        .await?;

    let res = def_res.get("result").unwrap();

    // If array check first element
    let loc = if res.is_array() {
        res.as_array()
            .unwrap()
            .first()
            .expect("Empty definition result")
    } else {
        res
    };

    let uri = loc
        .get("uri")
        .and_then(|u| u.as_str())
        .expect("No URI in definition");
    assert!(uri.ends_with("a.md"), "Definition should point to a.md");

    // 5. References Test
    let ref_res: Value = c
        .send_request(
            "textDocument/references",
            json!({
                "textDocument": {"uri": file_a_uri},
                "position": {"line": 1, "character": 8},
                "context": { "includeDeclaration": true }
            }),
        )
        .await?;

    let refs = ref_res
        .get("result")
        .and_then(|r| r.as_array())
        .expect("References result should be array");

    assert!(
        refs.len() >= 2,
        "Should find at least 2 references (def + ref)"
    );
    let found_b = refs.iter().any(|r| {
        r.get("uri")
            .and_then(|u| u.as_str())
            .unwrap_or("")
            .ends_with("b.md")
    });
    assert!(found_b, "Should find reference in b.md");

    // Shutdown
    c.send_request("shutdown", json!({})).await?;
    c.send_notification("exit", json!({})).await?;

    Ok(())
}
