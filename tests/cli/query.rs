use predicates::prelude::*;

#[test]
fn test_query_basic_match() {
    let tmp = crate::common::setup_temp_dir();
    
    // Define UC and FR node types
    let config = r#"
[nodes.UC]
desc = "Use Case"
[nodes.FR]
desc = "Functional Requirement"
"#;
    crate::common::create_config(tmp.path(), config);

    // Create docs with anchors
    crate::common::create_test_doc(
        tmp.path(),
        "uc.md",
        "<a id=\"UC_001\"></a>\n\n# User Login\nUser logs in.\n"
    );

    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (n:UC) RETURN n.id")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("UC_001"));
}

#[test]
fn test_query_filtering() {
    let tmp = crate::common::setup_temp_dir();
    let config = r#"
[nodes.UC]
desc = "Use Case"
"#;
    crate::common::create_config(tmp.path(), config);

    crate::common::create_test_doc(
        tmp.path(),
        "uc1.md",
        "<a id=\"UC_001\"></a>\n\n# User Login\nUser logs in.\n"
    );
    crate::common::create_test_doc(
        tmp.path(),
        "uc2.md",
        "<a id=\"UC_002\"></a>\n\n# User Logout\nUser logs out.\n"
    );

    // Filter by name (which comes from the heading)
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (n:UC) WHERE n.name CONTAINS \"Logout\" RETURN n.id")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("UC_002"))
        .stdout(predicate::str::contains("UC_001").not());
}

#[test]
fn test_query_properties() {
    let tmp = crate::common::setup_temp_dir();
    let config = r#"
[nodes.UC]
desc = "Use Case"
"#;
    crate::common::create_config(tmp.path(), config);

    // Create subdir
    let sub = tmp.path().join("sub");
    std::fs::create_dir(&sub).unwrap();

    crate::common::create_test_doc(
        &sub,
        "export.md",
        "<a id=\"UC_003\"></a>\n\n# Data Export\nExport user data.\n"
    );

    // Query name, file, line
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (n:UC) RETURN n.name, n.file, n.line")
        .arg("--format")
        .arg("json")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"n.name\": \"Data Export\""))
        .stdout(predicate::str::contains("sub/export.md"))
        .stdout(predicate::str::contains("\"n.line\": \"1\""));
}

#[test]
fn test_query_variable_length_path() {
    let tmp = crate::common::setup_temp_dir();
    let config = r#"
[nodes.UC]
desc = "Use Case"
[nodes.FR]
desc = "Functional Requirement"
[nodes.MOD]
desc = "Module"
"#;
    crate::common::create_config(tmp.path(), config);

    // Create a chain: UC_001 <- FR_001 <- MOD_001
    // In docgraph, a link [Target](#Target) creates an edge Source -> Target.
    crate::common::create_test_doc(
        tmp.path(),
        "chain.md",
        r#"
<a id="UC_001"></a>

# User Login
User logs in.

<a id="FR_001"></a>

# Authentication
Realizes: [UC_001](#UC_001)

<a id="MOD_001"></a>

# AuthModule
Realizes: [FR_001](#FR_001)
"#
    );

    // Query with variable length path: UC -> ... -> MOD
    // Note: The relationship direction in doc is "realizes", so FR realizes UC (FR -> UC). 
    // MOD realizes FR (MOD -> FR).
    // So distinct path is MOD -> FR -> UC.
    // Let's query: MATCH (m:MOD)-[*1..2]->(u:UC) RETURN m.id, u.id
    
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (m:MOD)-[*1..2]->(u:UC) RETURN m.id, u.id")
        .arg("--format")
        .arg("json")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"m.id\": \"MOD_001\""))
        .stdout(predicate::str::contains("\"u.id\": \"UC_001\""));
}

#[test]
fn test_query_direction() {
    let tmp = crate::common::setup_temp_dir();
    let config = r#"
[nodes.A]
desc = "Type A"
[nodes.B]
desc = "Type B"
[nodes.C]
desc = "Type C"
"#;
    crate::common::create_config(tmp.path(), config);

    // Chain: A_001 -> B_001 -> C_001
    crate::common::create_test_doc(
        tmp.path(),
        "chain_abc.md",
        r#"
<a id="A_001"></a>
# Node A
Link to B: [B_001](#B_001)

<a id="B_001"></a>
# Node B
Link to C: [C_001](#C_001)

<a id="C_001"></a>
# Node C
End.
"#
    );

    // 1. Forward: (a)-[right]->(c)
    // A -> ... -> C. Should match.
    let assert = assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (a:A)-[*1..2]->(c:C) RETURN a.id, c.id")
        .arg("--format")
        .arg("json")
        .arg(tmp.path())
        .assert()
        .success();
    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Failed to parse JSON output");
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert!(!arr.is_empty(), "Forward match failed: empty result");
    // Check content
    let found = arr.iter().any(|row| 
        row["a.id"] == "A_001" && row["c.id"] == "C_001"
    );
    assert!(found, "Forward match failed: content mismatch");

    // 2. Backward: (c)<-[left]-(a)
    // C <- ... <- A. Meaning A -> ... -> C. Should match.
    let assert = assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (c:C)<-[*1..2]-(a:A) RETURN a.id, c.id")
        .arg("--format")
        .arg("json")
        .arg(tmp.path())
        .assert()
        .success();
    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Failed to parse JSON output");
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert!(!arr.is_empty(), "Backward match failed: empty result");
    let found = arr.iter().any(|row| 
        row["a.id"] == "A_001" && row["c.id"] == "C_001"
    );
    assert!(found, "Backward match failed: content mismatch");

    // 3. Wrong Direction Forward: (c)-[right]->(a)
    // C -> ... -> A. Should be empty.
    let assert = assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (c:C)-[*1..2]->(a:A) RETURN a.id, c.id")
        .arg("--format")
        .arg("json")
        .arg(tmp.path())
        .assert()
        .success();
    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Expect empty JSON array "[]" or similar null result depending on implementation
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
         if let Some(arr) = json.as_array() {
             assert!(arr.is_empty(), "Wrong Forward match succeeded (should be empty): {}", stdout);
         }
    } else {
        // If it outputs "No results found." text (which it shouldn't in JSON mode ideally, but handlers/query.rs handles it)
        // Wait, handlers/query.rs for JSON:
        // if result.rows.is_empty() -> "[]" ?
        // Let's check logic:
        // OutputFormat::Json => { ... let json_out = serde_json::to_string_pretty(&json_rows)... println!("{}", json_out); }
        // It always outputs a valid JSON array, even if empty.
    }

    // 4. Wrong Direction Backward: (a)<-[left]-(c)
    // A <- ... <- C. Should be empty.
    let assert = assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (a:A)<-[*1..2]-(c:C) RETURN a.id, c.id")
        .arg("--format")
        .arg("json")
        .arg(tmp.path())
        .assert()
        .success();
    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
         if let Some(arr) = json.as_array() {
             assert!(arr.is_empty(), "Wrong Backward match succeeded (should be empty): {}", stdout);
         }
    }
}

#[test]
fn test_query_output_format() {
    let tmp = crate::common::setup_temp_dir();
    let config = r#"
[nodes.UC]
desc = "Use Case"
"#;
    crate::common::create_config(tmp.path(), config);

    crate::common::create_test_doc(
        tmp.path(),
        "uc.md",
        "<a id=\"UC_JSON\"></a>\n\n# JSON Test\nContent.\n"
    );

    // 1. JSON Output
    let assert = assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (n:UC) RETURN n.id")
        .arg("--format")
        .arg("json")
        .arg(tmp.path())
        .assert()
        .success();
    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Validate JSON structure
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Failed to parse JSON output");
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["n.id"], "UC_JSON");

    // 2. Table Output (Tidy)
    let assert = assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (n:UC) RETURN n.id")
        .arg("--format")
        .arg("table") // Explicit
        .arg(tmp.path())
        .assert()
        .success();
    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for UTF-8 borders (e.g., top-left corner '┌')
    // Note: Depends on comfy-table implementation details, but specific enough.
    assert!(stdout.contains("UC_JSON"));
    assert!(stdout.contains("┌")); 
    assert!(stdout.contains("n.id"));
}

#[test]
fn test_query_node_expansion() {
    let tmp = crate::common::setup_temp_dir();
    let config = r#"
[nodes.UC]
desc = "Use Case"
"#;
    crate::common::create_config(tmp.path(), config);

    crate::common::create_test_doc(
        tmp.path(),
        "test.md",
        r#"
<a id="UC_001"></a>
# User Login
This is content.
"#,
    );

    let assert = assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("query")
        .arg("MATCH (n:UC) RETURN n")
        .arg("--format")
        .arg("json")
        .arg(tmp.path())
        .assert()
        .success();

    let output = assert.get_output();
    let json_output: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let results = json_output.as_array().unwrap();
    
    assert_eq!(results.len(), 1);
    let row = &results[0];
    assert_eq!(row["n.id"], "UC_001");
    assert_eq!(row["n.type"], "UC");
    assert_eq!(row["n.name"], "User Login");
    assert!(row["n.content"].as_str().unwrap().contains("This is content."));
}
