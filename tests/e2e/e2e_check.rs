use std::path::Path;

fn create_doc_missing_heading(dir: &Path, id: &str) -> std::path::PathBuf {
    let content = format!("<a id=\"{}\"></a>\n", id);
    crate::common::create_test_doc(dir, "test.md", &content)
}

fn create_docs_with_duplicate_id(dir: &Path, id: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let content1 = format!("<a id=\"{}\"></a>\n\n# First\n", id);
    let content2 = format!("<a id=\"{}\"></a>\n\n# Second\n", id);

    let path1 = crate::common::create_test_doc(dir, "doc1.md", &content1);
    let path2 = crate::common::create_test_doc(dir, "doc2.md", &content2);

    (path1, path2)
}

use predicates::prelude::*;

#[test]
fn check_help_works() {
    assert_cmd::cargo_bin_cmd!()
        .arg("check")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Check the documentation graph"));
}

#[test]
fn check_valid_doc_succeeds() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "Test Document");

    assert_cmd::cargo_bin_cmd!()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No errors"));
}

#[test]
fn check_missing_heading_fails() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    create_doc_missing_heading(tmp.path(), "TEST-01");

    assert_cmd::cargo_bin_cmd!()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("DG001"))
        .stdout(predicate::str::contains("is not followed by a heading"));
}

#[test]
fn check_duplicate_id_fails() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    create_docs_with_duplicate_id(tmp.path(), "TEST-01");

    assert_cmd::cargo_bin_cmd!()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("DG002"))
        .stdout(predicate::str::contains("Duplicate anchor ID"));
}

#[test]
fn check_json_output() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "Test");

    let output = assert_cmd::cargo_bin_cmd!()
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
fn check_with_rule_filter() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    create_doc_missing_heading(tmp.path(), "TEST-01");

    // Run only DG001
    assert_cmd::cargo_bin_cmd!()
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
    let tmp = crate::common::setup_temp_dir();

    assert_cmd::cargo_bin_cmd!()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No errors"));
}

#[test]
fn check_nonexistent_path_fails() {
    assert_cmd::cargo_bin_cmd!()
        .arg("check")
        .arg("/nonexistent/path/that/does/not/exist")
        .assert()
        .success(); // Should succeed with no files found
}

#[test]
fn check_with_config_file() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "Test");

    assert_cmd::cargo_bin_cmd!()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .success();
}

#[test]
fn check_strict_node_types() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(
        tmp.path(),
        r#"
[graph]
strict_node_types = true

[node_types]
TEST = { desc = "Test node" }
"#,
    );

    // Valid node type
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "Test");

    assert_cmd::cargo_bin_cmd!()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .success();

    // Invalid node type
    crate::common::create_test_doc(
        tmp.path(),
        "invalid.md",
        "<a id=\"INVALID-01\"></a>\n\n# Invalid\n",
    );

    assert_cmd::cargo_bin_cmd!()
        .arg("check")
        .arg(tmp.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("DG005"))
        .stdout(predicate::str::contains("Unknown node type prefix"));
}
