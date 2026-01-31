mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn trace_help_works() {
    Command::cargo_bin("docgraph")
        .unwrap()
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
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_test_doc(
        tmp.path(),
        "a.md",
        "<a id=\"TEST-A\"></a>\n\n# A\n\n[TEST-B](./b.md#TEST-B)\n",
    );
    common::create_test_doc(tmp.path(), "b.md", "<a id=\"TEST-B\"></a>\n\n# B\n");

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("trace")
        .arg("TEST-A")
        .arg("TEST-B")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST-A"))
        .stdout(predicate::str::contains("TEST-B"));
}

#[test]
fn trace_with_direction_down() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_test_doc(
        tmp.path(),
        "a.md",
        "<a id=\"TEST-A\"></a>\n\n# A\n\n[TEST-B](./b.md#TEST-B)\n",
    );
    common::create_test_doc(tmp.path(), "b.md", "<a id=\"TEST-B\"></a>\n\n# B\n");

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("trace")
        .arg("TEST-A")
        .arg("TEST-B")
        .arg("--direction")
        .arg("down")
        .arg(tmp.path())
        .assert()
        .success();
}

#[test]
fn trace_with_direction_up() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_test_doc(
        tmp.path(),
        "a.md",
        "<a id=\"TEST-A\"></a>\n\n# A\n\n[TEST-B](./b.md#TEST-B)\n",
    );
    common::create_test_doc(tmp.path(), "b.md", "<a id=\"TEST-B\"></a>\n\n# B\n");

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("trace")
        .arg("TEST-B")
        .arg("TEST-A")
        .arg("--direction")
        .arg("up")
        .arg(tmp.path())
        .assert()
        .success();
}

#[test]
fn trace_no_path_found() {
    let tmp = common::setup_temp_dir();
    common::create_config(tmp.path(), common::default_config());
    common::create_valid_doc(tmp.path(), "TEST-A", "A");
    common::create_test_doc(tmp.path(), "b.md", "<a id=\"TEST-B\"></a>\n\n# B\n");

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("trace")
        .arg("TEST-A")
        .arg("TEST-B")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No paths found"));
}

#[test]
fn trace_invalid_direction() {
    let tmp = common::setup_temp_dir();

    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("trace")
        .arg("TEST-A")
        .arg("TEST-B")
        .arg("--direction")
        .arg("invalid")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid direction"));
}
