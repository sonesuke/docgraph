use predicates::prelude::*;

#[test]
fn trace_help_works() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("trace")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Trace relationships between spec blocks",
        ));
}

#[test]
fn trace_direct_path() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_test_doc(tmp.path(), "a.md", "<a id=\"TEST-A\"></a>\n\n# A\n");
    crate::common::create_test_doc(
        tmp.path(),
        "b.md",
        "<a id=\"TEST-B\"></a>\n\n# B\n\n- [link](a.md#TEST-A)\n",
    );

    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("trace")
        .arg("TEST-B")
        .arg("TEST-A")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-B -> TEST-A"));
}

#[test]
fn trace_no_path_found() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_test_doc(tmp.path(), "a.md", "<a id=\"TEST-A\"></a>\n\n# A\n");
    crate::common::create_test_doc(tmp.path(), "b.md", "<a id=\"TEST-B\"></a>\n\n# B\n");

    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("trace")
        .arg("TEST-B")
        .arg("TEST-A")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No paths found"));
}

#[test]
fn trace_invalid_direction() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());

    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("trace")
        .arg("--direction")
        .arg("invalid")
        .arg("A")
        .arg("B")
        .arg(tmp.path())
        .assert()
        .failure();
}

#[test]
fn trace_with_direction_down() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-A", "A");
    crate::common::create_test_doc(
        tmp.path(),
        "b.md",
        "<a id=\"TEST-B\"></a>\n\n# B\n\n- [link](a.md#TEST-A)\n",
    );

    // B uses A (B -> A)
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("trace")
        .arg("--direction")
        .arg("down")
        .arg("TEST-B")
        .arg("TEST-A")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-B -> TEST-A"));
}

#[test]
fn trace_with_direction_up() {
    let tmp = crate::common::setup_temp_dir();
    crate::common::create_config(tmp.path(), crate::common::default_config());
    crate::common::create_valid_doc(tmp.path(), "TEST-A", "A");
    crate::common::create_test_doc(
        tmp.path(),
        "b.md",
        "<a id=\"TEST-B\"></a>\n\n# B\n\n- [link](a.md#TEST-A)\n",
    );

    // A is used by B (A <- B)
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("trace")
        .arg("--direction")
        .arg("up")
        .arg("TEST-A")
        .arg("TEST-B")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-A <- TEST-B"));
}
