use predicates::prelude::*;

#[test]
fn graph_help_works() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("graph")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate graph data"));
}

#[test]
fn graph_summary() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-01", "Test");

    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("graph")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"id\": \"TEST-01\""));
}

#[test]
fn graph_with_edges() {
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
        "<a id=\"ADR-01\"></a>\n\n# ADR\n\n- [REQ-01](a.md)\n",
    );

    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("graph")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"id\": \"REQ-01\""))
        .stdout(predicate::str::contains("\"id\": \"ADR-01\""));
}
