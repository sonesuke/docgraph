use crate::types::{Diagnostic, Range, Severity};
use crate::walk::find_markdown_files;
use rumdl_lib::fix_coordinator::FixCoordinator;
use rumdl_lib::workspace_index::WorkspaceIndex;

use std::fs;
use std::path::Path;

/// Check if a rumdl warning should be skipped.
/// - MD013 (line length): Always skip
/// - MD033 (inline HTML): Skip only `<a id="...">` tags (docgraph anchor syntax)
/// - MD041 (first line heading): Always skip - docgraph allows anchor tags before H1
fn should_skip_rumdl_warning(rule_name: &str, message: &str) -> bool {
    match rule_name {
        "MD013" => true,                       // Skip line length warnings
        "MD033" => message.contains("<a id="), // Skip anchor id tags only
        "MD041" => true, // Skip first-line-heading - we allow <a id="..."> before # heading
        _ => false,
    }
}

pub fn check_workspace(
    root: &Path,
    fix: bool,
    rule_filter: Option<Vec<String>>,
    use_docgraph_filter: bool,
) -> Vec<Diagnostic> {
    // Run rumdl on each markdown file
    run_rumdl_on_workspace(root, fix, rule_filter, use_docgraph_filter)
}

/// Run rumdl markdown linter on all markdown files in the workspace
fn run_rumdl_on_workspace(
    root: &Path,
    fix: bool,
    rule_filter: Option<Vec<String>>,
    use_docgraph_filter: bool,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let files = find_markdown_files(root);

    // Create default rumdl config and rules
    let config = rumdl_lib::config::Config::default();
    let mut all_rules = rumdl_lib::rules::all_rules(&config);
    // Add custom docgraph rules
    all_rules.push(Box::new(crate::rules::dg001::DG001));
    all_rules.push(Box::new(crate::rules::dg002::DG002));
    all_rules.push(Box::new(crate::rules::dg003::DG003));
    all_rules.push(Box::new(crate::rules::dg004::DG004));

    let rules: Vec<Box<dyn rumdl_lib::rule::Rule>> = if let Some(names) = rule_filter {
        // Run only specific rules
        let allowed_names: std::collections::HashSet<_> =
            names.iter().map(|s| s.as_str()).collect();
        all_rules
            .into_iter()
            .filter(|r| allowed_names.contains(r.name()))
            .collect()
    } else if use_docgraph_filter {
        // Run all rules except MD013 and MD051 (docgraph default)
        all_rules
            .into_iter()
            .filter(|r| r.name() != "MD013" && r.name() != "MD051")
            .collect()
    } else {
        // Run all rules (rumdl default)
        all_rules
    };

    let mut workspace_index = WorkspaceIndex::new();

    // Pass 1: Lint individual files and build index
    for file_path in &files {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                let mut working_content = content.clone();

                // Fix if requested
                if fix {
                    let coordinator = FixCoordinator::new();
                    let fix_result = coordinator.apply_fixes_iterative(
                        &rules,
                        &[],
                        &mut working_content,
                        &config,
                        10,
                    );
                    if let Some(_result) = fix_result.ok().filter(|r| r.rules_fixed > 0) {
                        let write_result = fs::write(file_path, &working_content);
                        if let Err(e) = write_result {
                            eprintln!("Failed to write fixed file {:?}: {}", file_path, e);
                        }
                    }
                }

                // Use lint_and_index to get both warnings and the FileIndex
                let (result, file_index) = rumdl_lib::lint_and_index(
                    &working_content,
                    &rules,
                    false,
                    rumdl_lib::config::MarkdownFlavor::Standard,
                    Some(file_path.clone()),
                    Some(&config),
                );

                // Add to workspace index
                workspace_index.insert_file(file_path.clone(), file_index);

                if let Ok(warnings) = result {
                    for warning in warnings {
                        let rule_name = warning.rule_name.as_deref().unwrap_or("");

                        // Skip MD033 warnings for docgraph anchor syntax
                        if use_docgraph_filter
                            && should_skip_rumdl_warning(rule_name, &warning.message)
                        {
                            continue;
                        }

                        diagnostics.push(Diagnostic {
                            severity: match warning.severity {
                                rumdl_lib::rule::Severity::Error => Severity::Error,
                                rumdl_lib::rule::Severity::Warning
                                | rumdl_lib::rule::Severity::Info => Severity::Warning,
                            },
                            code: rule_name.to_string(),
                            message: warning.message,
                            path: file_path.clone(),
                            range: Range {
                                start_line: warning.line,
                                start_col: warning.column,
                                end_line: warning.line,
                                end_col: warning.column,
                            },
                        });
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read file {:?}: {}", file_path, e);
            }
        }
    }

    // Pass 2: Run cross-file checks
    for (path, file_index) in workspace_index.files() {
        if let Ok(warnings) = rumdl_lib::run_cross_file_checks(
            path,
            file_index,
            &rules,
            &workspace_index,
            Some(&config),
        ) {
            let mut triggers_write = false;
            let mut fixes_to_apply = Vec::new();

            for warning in warnings {
                // Apply same filtering if needed
                let rule_name = warning.rule_name.as_deref().unwrap_or("");
                if use_docgraph_filter && should_skip_rumdl_warning(rule_name, &warning.message) {
                    continue;
                }

                if let Some(f) = warning.fix.filter(|_| fix) {
                    fixes_to_apply.push(f);
                    triggers_write = true;
                } else {
                    diagnostics.push(Diagnostic {
                        severity: match warning.severity {
                            rumdl_lib::rule::Severity::Error => Severity::Error,
                            rumdl_lib::rule::Severity::Warning
                            | rumdl_lib::rule::Severity::Info => Severity::Warning,
                        },
                        code: rule_name.to_string(),
                        message: warning.message,
                        path: path.to_path_buf(),
                        range: Range {
                            start_line: warning.line,
                            start_col: warning.column,
                            end_line: warning.line,
                            end_col: warning.column,
                        },
                    });
                }
            }

            if let Some(mut content) = triggers_write
                .then(|| fs::read_to_string(path).ok())
                .flatten()
            {
                fixes_to_apply.sort_by(|a, b| b.range.start.cmp(&a.range.start));

                for f in fixes_to_apply {
                    content.replace_range(f.range, &f.replacement);
                }

                let write_result = fs::write(path, content);
                if let Err(e) = write_result {
                    eprintln!("Failed to write fixed file {:?}: {}", path, e);
                }
            }
        }
    }

    diagnostics
}

// Legacy tests removed (migrated to DG002/DG003 integration tests)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_skip_rumdl_anchor_id() {
        // <a id="..."> should be skipped (docgraph anchor syntax)
        assert!(should_skip_rumdl_warning(
            "MD033",
            "Inline HTML found: <a id=\"TEST\">"
        ));
        assert!(should_skip_rumdl_warning(
            "MD033",
            "Inline HTML found: <a id='TEST'>"
        ));
    }

    #[test]
    fn test_should_not_skip_rumdl_other_html() {
        // Other HTML tags should NOT be skipped
        assert!(!should_skip_rumdl_warning(
            "MD033",
            "Inline HTML found: <div>"
        ));
        assert!(!should_skip_rumdl_warning(
            "MD033",
            "Inline HTML found: <span>"
        ));
        assert!(!should_skip_rumdl_warning(
            "MD033",
            "Inline HTML found: <a href=\"...\">"
        ));
        assert!(!should_skip_rumdl_warning(
            "MD033",
            "Inline HTML found: <p>"
        ));
    }

    #[test]
    fn test_skip_logic_for_other_rules() {
        // MD013 (Line length) should be skipped
        assert!(should_skip_rumdl_warning(
            "MD013",
            "Line length exceeds 80 characters"
        ));

        // Other random rules should NOT be skipped
        assert!(!should_skip_rumdl_warning(
            "MD001",
            "Inline HTML found: <a id=\"TEST\">"
        ));
    }
}
