mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn list_help_works() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("list")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("List spec blocks matching a query"));
}

#[test]
fn list_all_nodes() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_valid_doc(tmp.path(), "TEST-01", "First");
    common::create_test_doc(tmp.path(), "doc2.md", "<a id=\"TEST-02\"></a>\n\n# Second\n");
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("list")
        .arg("*")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-01"))
        .stdout(predicate::str::contains("TEST-02"));
}

#[test]
fn list_with_pattern() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_valid_doc(tmp.path(), "TEST-01", "Test");
    common::create_test_doc(tmp.path(), "req.md", "<a id=\"REQ-01\"></a>\n\n# Requirement\n");
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("list")
        .arg("TEST-*")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-01"))
        .stdout(predicate::str::contains("REQ-01").not());
}

#[test]
fn list_empty_directory() {
    let tmp = common::setup_temp_dir();
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("list")
        .arg("*")
        .arg(tmp.path())
        .assert()
        .success();
}
