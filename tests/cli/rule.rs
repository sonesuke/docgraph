use predicates::prelude::*;

#[test]
fn rule_help_works() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("rule")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Show information about available rules",
        ));
}

#[test]
fn rule_list_all() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("rule")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available rules"))
        .stdout(predicate::str::contains("DG001"))
        .stdout(predicate::str::contains("DG002"))
        .stdout(predicate::str::contains("DG007"));
}

#[test]
fn rule_show_dg007() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("rule")
        .arg("DG007")
        .assert()
        .success()
        .stdout(predicate::str::contains("DG007"))
        .stdout(predicate::str::contains("template adherence"));
}

#[test]
fn rule_show_specific() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("rule")
        .arg("DG001")
        .assert()
        .success()
        .stdout(predicate::str::contains("DG001"));
}

#[test]
fn rule_not_found() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("rule")
        .arg("NONEXISTENT")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn rule_case_insensitive() {
    assert_cmd::cargo_bin_cmd!("docgraph")
        .arg("rule")
        .arg("dg001")
        .assert()
        .success()
        .stdout(predicate::str::contains("DG001"));
}

#[test]
fn all_implemented_rules_have_descriptions() {
    let rules_dir = std::path::Path::new("src/core/rules");
    let entries = std::fs::read_dir(rules_dir).expect("Failed to read rules directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with("dg") && file_name.ends_with(".rs") && file_name != "mod.rs"
                {
                    let rule_id = file_name.trim_end_matches(".rs").to_ascii_uppercase();

                    // Every implemented rule file must have a description in the CLI
                    assert_cmd::cargo_bin_cmd!("docgraph")
                        .arg("rule")
                        .arg(&rule_id)
                        .assert()
                        .success()
                        .stdout(predicate::str::contains(&rule_id));
                }
            }
        }
    }
}
