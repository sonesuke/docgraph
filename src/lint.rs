use crate::types::Diagnostic;
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
    path: &Path,
    fix: bool,
    rule_filter: Option<Vec<String>>,
    use_docgraph_filter: bool,
    config: &crate::config::Config,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let files = crate::walk::find_markdown_files(path);

    let rumdl_config = rumdl_lib::config::Config::default();
    let mut all_rules = rumdl_lib::rules::all_rules(&rumdl_config);
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
        match fs::read_to_string::<&Path>(file_path) {
            Ok(content) => {
                let mut working_content = content.clone();

                // Fix if requested
                if fix {
                    let coordinator = FixCoordinator::new();
                    let fix_result = coordinator.apply_fixes_iterative(
                        &rules,
                        &[],
                        &mut working_content,
                        &rumdl_config,
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
                    Some(file_path.to_path_buf()),
                    Some(&rumdl_config),
                );

                // Add to workspace index
                workspace_index.insert_file(file_path.to_path_buf(), file_index);

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
                                rumdl_lib::rule::Severity::Error => crate::types::Severity::Error,
                                _ => crate::types::Severity::Warning,
                            },
                            code: rule_name.to_string(),
                            message: warning.message,
                            path: file_path.to_path_buf(),
                            range: crate::types::Range {
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

    // Pass 2: Run cross-file checks for rules that support it (DG002, DG003, DG004)
    for file_path in &files {
        let file_index = match workspace_index.get_file(file_path) {
            Some(idx) => idx,
            None => continue,
        };

        // Collect fixes to apply for this file
        let mut fixes_to_apply: Vec<rumdl_lib::rule::Fix> = Vec::new();

        for rule in &rules {
            if rule.cross_file_scope() != rumdl_lib::rule::CrossFileScope::None
                && let Ok(warnings) = rule.cross_file_check(file_path, file_index, &workspace_index)
            {
                for warning in &warnings {
                    let rule_name = warning.rule_name.as_deref().unwrap_or("");

                    diagnostics.push(Diagnostic {
                        severity: match warning.severity {
                            rumdl_lib::rule::Severity::Error => crate::types::Severity::Error,
                            _ => crate::types::Severity::Warning,
                        },
                        code: rule_name.to_string(),
                        message: warning.message.clone(),
                        path: file_path.to_path_buf(),
                        range: crate::types::Range {
                            start_line: warning.line,
                            start_col: warning.column,
                            end_line: warning.line,
                            end_col: warning.column,
                        },
                    });

                    // Collect fixes if fix mode is enabled
                    if fix
                        && let Some(fix_info) = &warning.fix
                    {
                        fixes_to_apply.push(fix_info.clone());
                    }
                }
            }
        }

        // Apply fixes for this file (if any)
        if fix && !fixes_to_apply.is_empty()
            && let Ok(mut content) = fs::read_to_string(file_path)
        {
            // Sort fixes by range.start in descending order to avoid offset issues
            fixes_to_apply.sort_by(|a, b| b.range.start.cmp(&a.range.start));
            
            for fix_info in fixes_to_apply {
                // Replace the byte range with the replacement content
                if fix_info.range.start <= content.len() && fix_info.range.end <= content.len() {
                    content.replace_range(fix_info.range.clone(), &fix_info.replacement);
                }
            }
            
            if let Err(e) = fs::write(file_path, &content) {
                eprintln!("Failed to write fixed file {:?}: {}", file_path, e);
            }
        }
    }

    // Pass 3: Run custom docgraph workspace-level checks (DG005, DG006)
    // Collect docgraph's own SpecBlock data
    let (spec_blocks, _refs) = crate::collect::collect_workspace_all(path);

    // DG005: Strict Node Types
    let dg005_diags = crate::rules::dg005::check_strict_node_types(&spec_blocks, config);
    diagnostics.extend(dg005_diags);

    // DG006: Strict Relations
    let dg006_diags = crate::rules::dg006::check_strict_relations(&spec_blocks, config);
    diagnostics.extend(dg006_diags);

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
