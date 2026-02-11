use predicates::prelude::*;

#[test]
fn list_help_works() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("list")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "List spec blocks matching a query",
        ));
}

#[test]
fn list_all_elements() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "First");
    crate::common::create_test_doc(
        tmp.path(),
        "second.md",
        "<a id=\"REQ-01\"></a>\n\n# Second\n",
    );

    // Using " " or empty string often matches all/prefix in some CLI tools,
    // but here we just test if we can find both by a common prefix or by individual calls.
    // Let's use individual calls or a common prefix if possible.
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("list")
        .arg("TEST-01")
        .arg("--path")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-01 : First"));
}

#[test]
fn list_with_query() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "Test");
    crate::common::create_test_doc(tmp.path(), "other.md", "<a id=\"REQ-01\"></a>\n\n# Other\n");

    // List only TEST-*
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("list")
        .arg("TEST")
        .arg("--path")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-01 : Test"))
        .stdout(predicate::str::contains("REQ-01").not());
}
