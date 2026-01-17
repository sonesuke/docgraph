use regex::Regex;
use rumdl_lib::lint_context::LintContext;
use rumdl_lib::rule::{CrossFileScope, LintError, LintResult, LintWarning, Rule, Severity};
use rumdl_lib::workspace_index::{FileIndex, HeadingIndex, WorkspaceIndex};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct DG002;

impl Rule for DG002 {
    fn name(&self) -> &'static str {
        "DG002"
    }

    fn description(&self) -> &'static str {
        "Duplicate anchor ID found in workspace"
    }

    fn check(&self, _ctx: &LintContext) -> LintResult {
        Ok(Vec::new())
    }

    fn contribute_to_index(&self, ctx: &LintContext, file_index: &mut FileIndex) {
        // Extract <a id="..."> tags and add them as headings with custom anchors
        // This allows them to be indexed for cross-file checks
        let anchor_re = Regex::new(r#"<a\s+id=["']([^"']+)["']\s*>\s*</a>"#).unwrap();
        for (i, line) in ctx.content.lines().enumerate() {
            for caps in anchor_re.captures_iter(line) {
                if let Some(cap_id) = caps.get(1) {
                    let id = cap_id.as_str().to_string();
                    file_index.add_heading(HeadingIndex {
                        text: String::new(),
                        auto_anchor: String::new(),
                        custom_anchor: Some(id),
                        line: i + 1,
                    });
                }
            }
        }
    }

    fn cross_file_scope(&self) -> CrossFileScope {
        CrossFileScope::Workspace
    }

    fn cross_file_check(
        &self,
        current_path: &Path,
        file_index: &FileIndex,
        workspace_index: &WorkspaceIndex,
    ) -> LintResult {
        let mut warnings = Vec::new();

        // Check explicit anchors (now stored as headings with custom_anchor)
        for heading in &file_index.headings {
            if let Some(id) = &heading.custom_anchor {
                // Check against all other files
                for (other_path, other_index) in workspace_index.files() {
                    if other_path == current_path {
                        continue;
                    }

                    if other_index.has_anchor(id) {
                        warnings.push(LintWarning {
                            message: format!(
                                "Duplicate anchor ID '{}' found. Also defined in {}",
                                id,
                                other_path.display()
                            ),
                            line: heading.line,
                            column: 1, // We don't track column in HeadingIndex unfortunately, defaulting to 1
                            end_line: heading.line,
                            end_column: 1,
                            severity: Severity::Error,
                            fix: None,
                            rule_name: Some("DG002".to_string()),
                        });
                        break; // Report once per duplicate per file
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
