use predicates::prelude::*;

#[test]
fn describe_help_works() {
    assert_cmd::cargo_bin_cmd!()
        .arg("describe")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Describe a spec block"));
}

#[test]
fn describe_existing_node() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "Test Document");

    assert_cmd::cargo_bin_cmd!()
        .arg("describe")
        .arg("TEST-01")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-01"))
        .stdout(predicate::str::contains("Test Document"));
}

#[test]
fn describe_with_references() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_test_doc(
        tmp.path(),
        "source.md",
        "<a id=\"TEST-01\"></a>\n\n# Source\n\nReferences [TEST-02](./target.md#TEST-02)\n",
    );
    crate::common::create_test_doc(
        tmp.path(),
        "target.md",
        "<a id=\"TEST-02\"></a>\n\n# Target\n",
    );

    assert_cmd::cargo_bin_cmd!()
        .arg("describe")
        .arg("TEST-01")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-01"))
        .stdout(predicate::str::contains("TEST-02"));
}

#[test]
fn describe_nonexistent_node() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());

    assert_cmd::cargo_bin_cmd!()
        .arg("describe")
        .arg("NONEXISTENT")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}
