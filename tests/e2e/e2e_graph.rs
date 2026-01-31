
use predicates::prelude::*;

#[test]
fn graph_help_works() {
    assert_cmd::cargo_bin_cmd!()
        .arg("graph")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate graph data"));
}

#[test]
fn graph_json_output() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "Test");

    let output = assert_cmd::cargo_bin_cmd!()
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
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_test_doc(
        tmp.path(),
        "source.md",
        "<a id=\"TEST-01\"></a>\n\n# Source\n\n[TEST-02](./target.md#TEST-02)\n",
    );
    crate::common::create_test_doc(
        tmp.path(),
        "target.md",
        "<a id=\"TEST-02\"></a>\n\n# Target\n",
    );

    let output = assert_cmd::cargo_bin_cmd!()
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
    let tmp = crate::common::setup_temp_dir();

    assert_cmd::cargo_bin_cmd!()
        .arg("graph")
        .arg(tmp.path())
        .assert()
        .success();
}
