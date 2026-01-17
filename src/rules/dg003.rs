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

        // Find links: [text](#ID)
        // Captures: 1=text, 2=#ID
        let link_re = Regex::new(r"\[([^\]]*)\]\((#[^)\s]+)\)").unwrap();

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
            if let Some(target_cap) = caps.get(2) {
                let target_ref = target_cap.as_str(); // e.g. "#DAT-TENANT"
                let target_id = &target_ref[1..]; // "DAT-TENANT"

                let (start_line, start_col) = get_pos(target_cap.start());
                let (end_line, end_col) = get_pos(target_cap.end());

                // 1. Check Local (if index exists)
                let is_local = if let Some(idx) = local_index {
                    idx.has_anchor(target_id)
                } else {
                    false
                };

                if is_local {
                    continue;
                }

                // 2. Check Global
                let mut found_path = None;
                for (path, idx) in workspace_index.files() {
                    if idx.has_anchor(target_id) {
                        found_path = Some(path);
                        break;
                    }
                }

                if let Some(target_path) = found_path {
                    // Found global but not local -> Error: Implicit global link
                    // FIX: Replace #ID with relative/path/to/file.md#ID

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
                            range: target_cap.range(),
                        }),
                        rule_name: Some("DG003".to_string()),
                    });
                } else {
                    // Not found anywhere -> Error: Non-existent ID
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
