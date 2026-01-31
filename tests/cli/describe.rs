use predicates::prelude::*;

#[test]
fn describe_help_works() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("describe")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Describe a spec block and its relationships",
        ));
}

#[test]
fn describe_specific_id() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "Test Document");

    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("describe")
        .arg("TEST-01")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-01"))
        .stdout(predicate::str::contains("Test Document"));
}

#[test]
fn describe_with_standalone_ref() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_test_doc(
        tmp.path(),
        "a.md",
        "<a id=\"REQ-01\"></a>\n\n# Requirement\n",
    );
    crate::common::create_test_doc(
        tmp.path(),
        "b.md",
        "This is a reference to [REQ-01](a.md).\n",
    );

    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("describe")
        .arg("REQ-01")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("REQ-01"))
        .stdout(predicate::str::contains("Requirement"));
}

#[test]
fn describe_not_found() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());

    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("describe")
        .arg("NONEXISTENT")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}
