use pathdiff::diff_paths;
use regex::Regex;
use rumdl_lib::lint_context::LintContext;
use rumdl_lib::rule::{CrossFileScope, Fix, LintError, LintResult, LintWarning, Rule, Severity};
use rumdl_lib::workspace_index::{FileIndex, WorkspaceIndex};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct DG003;

impl Rule for DG003 {
    fn name(&self) -> &'static str {
        "DG003"
    }

    fn description(&self) -> &'static str {
        "Link to non-existent anchor ID"
    }

    fn check(&self, _ctx: &LintContext) -> LintResult {
        Ok(Vec::new())
    }

    fn cross_file_scope(&self) -> CrossFileScope {
        CrossFileScope::Workspace
    }

    fn cross_file_check(
        &self,
        current_path: &Path,
        _file_index: &FileIndex,
        workspace_index: &WorkspaceIndex,
    ) -> LintResult {
        let mut warnings = Vec::new();

        let content = match std::fs::read_to_string(current_path) {
            Ok(c) => c,
            Err(_) => return Ok(warnings),
        };

        // Find links: [text](#ID) or [text](path/to/file.md#ID)
        // Match standard markdown links, capturing the URL part
        let link_re = Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap();

        // Helper to map byte offset to line/col
        let line_starts: Vec<usize> = std::iter::once(0)
            .chain(content.match_indices('\n').map(|(i, _)| i + 1))
            .collect();

        let get_pos = |offset: usize| -> (usize, usize) {
            let line_idx = line_starts
                .partition_point(|&x| x <= offset)
                .saturating_sub(1);
            let line_start = line_starts[line_idx];
            (line_idx + 1, offset - line_start + 1)
        };

        // Check if current file has an index (it should)
        let local_index = workspace_index.get_file(current_path);

        for caps in link_re.captures_iter(&content) {
            let url_part = caps.get(2).unwrap().as_str();

            // Case 1: Implicit local link (#ID)
            if let Some(target_id) = url_part.strip_prefix('#') {
                let (start_line, start_col) = get_pos(caps.get(2).unwrap().start());
                let (end_line, end_col) = get_pos(caps.get(2).unwrap().end());

                if local_index.is_some_and(|idx| idx.has_anchor(target_id)) {
                    continue;
                }

                // Not found locally, check globally
                let mut found_path = None;
                for (path, idx) in workspace_index.files() {
                    if idx.has_anchor(target_id) {
                        found_path = Some(path);
                        break;
                    }
                }

                if let Some(target_path) = found_path {
                    let current_dir = current_path.parent().unwrap_or(Path::new("."));
                    let relative_path =
                        diff_paths(target_path, current_dir).unwrap_or(target_path.to_path_buf());
                    let replacement = format!("{}#{}", relative_path.display(), target_id);

                    warnings.push(LintWarning {
                        message: format!(
                            "Anchor '{}' is defined in another file. Use explicit relative path.",
                            target_id
                        ),
                        line: start_line,
                        column: start_col,
                        end_line,
                        end_column: end_col,
                        severity: Severity::Error,
                        fix: Some(Fix {
                            replacement,
                            range: caps.get(2).unwrap().range(),
                        }),
                        rule_name: Some("DG003".to_string()),
                    });
                } else {
                    warnings.push(LintWarning {
                        message: format!("Link to MISSING anchor ID '{}'", target_id),
                        line: start_line,
                        column: start_col,
                        end_line,
                        end_column: end_col,
                        severity: Severity::Error,
                        fix: None,
                        rule_name: Some("DG003".to_string()),
                    });
                }
            }
            // Case 2: Explicit file link (path/to/file.md#ID)
            else if let Some(hash_pos) = url_part.find('#') {
                let file_part = &url_part[..hash_pos];
                let target_id = &url_part[hash_pos + 1..];

                // Resolve target file path relative to current file
                let current_dir = current_path.parent().unwrap_or_else(|| Path::new("."));
                let target_path = current_dir.join(file_part);

                let (start_line, start_col) = get_pos(caps.get(2).unwrap().start());
                let (end_line, end_col) = get_pos(caps.get(2).unwrap().end());

                // Try to canonicalize the target file path
                let abs_target_path = match target_path.canonicalize() {
                    Ok(p) => p,
                    Err(_) => {
                        // File doesn't exist - skip for now
                        continue;
                    }
                };

                // Find target file in workspace by comparing canonicalized paths
                let mut target_file_index = None;
                for (ws_path, idx) in workspace_index.files() {
                    if let Ok(ws_abs) = ws_path.canonicalize()
                        && ws_abs == abs_target_path
                    {
                        target_file_index = Some(idx);
                        break;
                    }
                }

                if target_file_index.is_some_and(|idx| idx.has_anchor(target_id)) {
                    // All good - ID exists in the specified file
                    continue;
                }

                // ID not found in the specified file. Check if it exists elsewhere.
                let mut found_paths = Vec::new();
                for (path, idx) in workspace_index.files() {
                    if idx.has_anchor(target_id) {
                        found_paths.push(path.to_path_buf());
                    }
                }

                if found_paths.is_empty() {
                    warnings.push(LintWarning {
                        message: format!("Link to MISSING anchor ID '{}'", target_id),
                        line: start_line,
                        column: start_col,
                        end_line,
                        end_column: end_col,
                        severity: Severity::Error,
                        fix: None,
                        rule_name: Some("DG003".to_string()),
                    });
                } else if found_paths.len() == 1 {
                    // Found 1 match elsewhere -> Auto-fix
                    let correct_path = &found_paths[0];
                    let relative_path =
                        diff_paths(correct_path, current_dir).unwrap_or(correct_path.to_path_buf());
                    let replacement = format!("{}#{}", relative_path.display(), target_id);

                    warnings.push(LintWarning {
                        message: format!(
                            "ID '{}' exists but not in specified file. Correct file: {}",
                            target_id,
                            correct_path.display()
                        ),
                        line: start_line,
                        column: start_col,
                        end_line,
                        end_column: end_col,
                        severity: Severity::Error,
                        fix: Some(Fix {
                            replacement,
                            range: caps.get(2).unwrap().range(),
                        }),
                        rule_name: Some("DG003".to_string()),
                    });
                } else {
                    // Found multiple matches -> Error without fix
                    warnings.push(LintWarning {
                        message: format!(
                            "ID '{}' found in multiple files ({:?}), but not in specified file.",
                            target_id, found_paths
                        ),
                        line: start_line,
                        column: start_col,
                        end_line,
                        end_column: end_col,
                        severity: Severity::Error,
                        fix: None,
                        rule_name: Some("DG003".to_string()),
                    });
                }
            }
        }

        Ok(warnings)
    }

    fn fix(&self, _ctx: &LintContext) -> Result<String, LintError> {
        Ok(String::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
