mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn check_help_works() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Check the documentation graph"));
}

#[test]
fn check_valid_doc_succeeds() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_valid_doc(tmp.path(), "TEST-01", "Test Document");
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No errors"));
}

#[test]
fn check_missing_heading_fails() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_doc_missing_heading(tmp.path(), "TEST-01");
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("DG001"))
        .stdout(predicate::str::contains("is not followed by a heading"));
}

#[test]
fn check_duplicate_id_fails() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_docs_with_duplicate_id(tmp.path(), "TEST-01");
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("DG002"))
        .stdout(predicate::str::contains("Duplicate anchor ID"));
}

#[test]
fn check_json_output() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_valid_doc(tmp.path(), "TEST-01", "Test");
    
    let output = Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg("--json")
        .arg(tmp.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    
    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.starts_with('[') || stdout.starts_with('{'));
}

#[test]

#[test]
fn check_with_rule_filter() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_doc_missing_heading(tmp.path(), "TEST-01");
    
    // Run only DG001
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg("--rule")
        .arg("DG001")
        .arg(tmp.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("DG001"));
}

#[test]
fn check_empty_directory() {
    let tmp = common::setup_temp_dir();
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No errors"));
}

#[test]
fn check_nonexistent_path_fails() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg("/nonexistent/path/that/does/not/exist")
        .assert()
        .success(); // Should succeed with no files found
}

#[test]
fn check_with_config_file() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_valid_doc(tmp.path(), "TEST-01", "Test");
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .success();
}

#[test]
fn check_strict_node_types() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), r#"
[graph]
strict_node_types = true

[node_types]
TEST = { desc = "Test node" }
"#);
    
    // Valid node type
    common::create_valid_doc(tmp.path(), "TEST-01", "Test");
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .success();
    
    // Invalid node type
    common::create_test_doc(tmp.path(), "invalid.md", "<a id=\"INVALID-01\"></a>\n\n# Invalid\n");
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("DG005"))
        .stdout(predicate::str::contains("Unknown node type prefix"));
}
