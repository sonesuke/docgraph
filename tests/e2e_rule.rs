mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn rule_help_works() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("rule")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Show information about available rules"));
}

#[test]
fn rule_list_all() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("rule")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available rules"))
        .stdout(predicate::str::contains("DG001"))
        .stdout(predicate::str::contains("DG002"));
}

#[test]
fn rule_show_specific() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("rule")
        .arg("DG001")
        .assert()
        .success()
        .stdout(predicate::str::contains("DG001"));
}

#[test]
fn rule_not_found() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("rule")
        .arg("NONEXISTENT")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn rule_case_insensitive() {
    Command::cargo_bin("docgraph")
        .unwrap()
        .arg("rule")
        .arg("dg001")
        .assert()
        .success()
        .stdout(predicate::str::contains("DG001"));
}
