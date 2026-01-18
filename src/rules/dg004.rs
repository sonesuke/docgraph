use crate::parse::extract_anchor_headings;
use regex::Regex;
use rumdl_lib::lint_context::LintContext;
use rumdl_lib::rule::{CrossFileScope, LintError, LintResult, LintWarning, Rule, Severity};
use rumdl_lib::workspace_index::{FileIndex, WorkspaceIndex};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct DG004;

impl Rule for DG004 {
    fn name(&self) -> &'static str {
        "DG004"
    }

    fn description(&self) -> &'static str {
        "Internal link text should follow 'ID (Title)' format"
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

        // Build a global map of ID to Title
        let mut id_to_title = HashMap::new();
        for (path, _) in workspace_index.files() {
            if let Ok(c) = std::fs::read_to_string(path) {
                let blocks = extract_anchor_headings(&c, path);
                for block in blocks {
                    if let Some(name) = block.name {
                        id_to_title.insert(block.id, name);
                    }
                }
            }
        }

        // Find links: [text](#ID) or [text](path#ID)
        // Captures: 1=text, 2=path#ID, 3=ID
        let link_re = Regex::new(r"\[([^\]]*)\]\(([^)]*#([^)]+))\)").unwrap();

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

        for caps in link_re.captures_iter(&content) {
            if let (Some(text_cap), Some(id_cap)) = (caps.get(1), caps.get(3)) {
                let current_text = text_cap.as_str();
                let target_id = id_cap.as_str();

                if let Some(title) = id_to_title.get(target_id) {
                    // Remove all occurrences of target_id from the title string
                    let clean_title = title.replace(target_id, "");
                    // Remove empty bracket pairs that remain after ID removal
                    let clean_title = clean_title
                        .replace("[]", "")
                        .replace("()", "")
                        .replace("{}", "");
                    // Remove remaining dangling separators and extra whitespace
                    let clean_title = clean_title.trim();
                    let clean_title = clean_title.trim_start_matches(|c: char| {
                        c == ' ' || c == ':' || c == '-' || c == '(' || c == '[' || c == ']' || c == ')'
                    });
                    let clean_title = clean_title.trim_end_matches(|c: char| {
                        c == ' ' || c == ':' || c == '-' || c == ')' || c == ']' || c == '(' || c == '['
                    });
                    let clean_title = clean_title.trim().to_string();


                    let expected_text = if clean_title.is_empty() {
                        target_id.to_string()
                    } else {
                        format!("{} ({})", target_id, clean_title)
                    };

                    if current_text != expected_text {
                        let (start_line, start_col) = get_pos(text_cap.start());
                        let (end_line, end_col) = get_pos(text_cap.end());

                        warnings.push(LintWarning {
                            message: format!(
                                "Link text for '{}' should be '{}' but found '{}'",
                                target_id, expected_text, current_text
                            ),
                            line: start_line,
                            column: start_col,
                            end_line,
                            end_column: end_col,
                            severity: Severity::Error,
                            fix: None,
                            rule_name: Some("DG004".to_string()),
                        });
                    }
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
