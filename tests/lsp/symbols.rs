use crate::support::lsp_client::LspClient;
use crate::support::server_bin;
use serde_json::json;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn e2e_document_symbols() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let root_path = dir.path().to_path_buf();

    fs::write(
        root_path.join("docgraph.toml"),
        r#"[node_types.REQ]
desc = "Requirement""#,
    )?;

    let file_path = root_path.join("test.md");
    let file_uri = format!("file://{}", file_path.to_str().unwrap());
    fs::write(
        &file_path,
        r#"
<a id="REQ-001"></a>
# Block One

<a id="REQ-002"></a>
# Block Two
"#,
    )?;

    let mut c = LspClient::spawn(&server_bin(), &["lsp"]).await?;
    c.send_request(
        "initialize",
        json!({
            "rootUri": format!("file://{}", root_path.to_str().unwrap()),
            "capabilities": {}
        }),
    )
    .await?;
    c.send_notification("initialized", json!({})).await?;

    let res: serde_json::Value = c
        .send_request(
            "textDocument/documentSymbol",
            json!({
                "textDocument": { "uri": file_uri }
            }),
        )
        .await?;

    let symbols = res.get("result").and_then(|v| v.as_array()).unwrap();
    assert_eq!(symbols.len(), 2);

    let names: Vec<_> = symbols
        .iter()
        .map(|s| s.get("name").unwrap().as_str().unwrap())
        .collect();
    assert!(names.contains(&"REQ-001 (Block One)"));
    assert!(names.contains(&"REQ-002 (Block Two)"));

    Ok(())
}

#[tokio::test]
async fn e2e_workspace_symbols() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let root_path = dir.path().to_path_buf();

    fs::write(
        root_path.join("docgraph.toml"),
        r#"[node_types.REQ]
desc = "Requirement""#,
    )?;

    fs::write(
        root_path.join("test1.md"),
        "<a id=\"REQ-001\"></a>\n# UniqueOne",
    )?;
    fs::write(
        root_path.join("test2.md"),
        "<a id=\"REQ-002\"></a>\n# CommonPrefix",
    )?;

    let mut c = LspClient::spawn(&server_bin(), &["lsp"]).await?;
    c.send_request(
        "initialize",
        json!({
            "rootUri": format!("file://{}", root_path.to_str().unwrap()),
            "capabilities": {}
        }),
    )
    .await?;
    c.send_notification("initialized", json!({})).await?;

    // Search by ID fragment
    let res: serde_json::Value = c
        .send_request(
            "workspace/symbol",
            json!({
                "query": "001"
            }),
        )
        .await?;
    let symbols = res.get("result").and_then(|v| v.as_array()).unwrap();
    assert_eq!(symbols.len(), 1);
    assert_eq!(
        symbols[0].get("name").unwrap().as_str().unwrap(),
        "REQ-001 (UniqueOne)"
    );

    // Search by Name fragment
    let res: serde_json::Value = c
        .send_request(
            "workspace/symbol",
            json!({
                "query": "prefix"
            }),
        )
        .await?;
    let symbols = res.get("result").and_then(|v| v.as_array()).unwrap();
    assert_eq!(symbols.len(), 1);
    assert_eq!(
        symbols[0].get("name").unwrap().as_str().unwrap(),
        "REQ-002 (CommonPrefix)"
    );

    Ok(())
}
