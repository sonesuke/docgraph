mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn graph_help_works() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("graph")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate graph data"));
}

#[test]
fn graph_json_output() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_valid_doc(tmp.path(), "TEST-01", "Test");
    
    let output = Command::cargo_bin("docgraph")
        .unwrap()
        .arg("graph")
        .arg(tmp.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    
    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains("TEST-01"));
    assert!(stdout.starts_with('{') || stdout.starts_with('['));
}

#[test]
fn graph_with_references() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_test_doc(
        tmp.path(),
        "source.md",
        "<a id=\"TEST-01\"></a>\n\n# Source\n\n[TEST-02](./target.md#TEST-02)\n",
    );
    common::create_test_doc(tmp.path(), "target.md", "<a id=\"TEST-02\"></a>\n\n# Target\n");
    
    let output = Command::cargo_bin("docgraph")
        .unwrap()
        .arg("graph")
        .arg(tmp.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    
    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains("TEST-01"));
    assert!(stdout.contains("TEST-02"));
}

#[test]
fn graph_empty_directory() {
    let tmp = common::setup_temp_dir();
    
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("graph")
        .arg(tmp.path())
        .assert()
        .success();
}
