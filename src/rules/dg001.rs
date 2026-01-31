use regex::Regex;
use rumdl_lib::lint_context::LintContext;
use rumdl_lib::rule::{LintError, LintResult, LintWarning, Rule, Severity};

#[derive(Debug, Clone, Default)]
pub struct DG001;

impl Rule for DG001 {
    fn name(&self) -> &'static str {
        "DG001"
    }

    fn description(&self) -> &'static str {
        "Anchor must be followed by a heading"
    }

    fn check(&self, ctx: &LintContext) -> LintResult {
        let mut warnings = Vec::new();
        let content = ctx.content;
        let lines: Vec<&str> = content.lines().collect();

        // Regex logic from extract_anchor_headings
        let anchor_re = Regex::new(r#"<a\s+id=["']([^"']+)["']\s*>\s*</a>"#).unwrap();
        let heading_re = Regex::new(r"^(#{1,6})\s+(.+)$").unwrap();

        for (i, line) in lines.iter().enumerate() {
            if let Some(caps) = anchor_re.captures(line.trim()) {
                let id = caps.get(1).unwrap().as_str();

                // Look for heading in next non-empty lines
                let mut j = i + 1;
                while j < lines.len() && lines[j].trim().is_empty() {
                    j += 1;
                }

                let found_heading = if j < lines.len() {
                    heading_re.is_match(lines[j].trim())
                } else {
                    false
                };

                if !found_heading {
                    warnings.push(LintWarning {
                        message: format!(
                            "Anchor '{}' is not followed by a heading of a section",
                            id
                        ),
                        line: i + 1,
                        column: 1,
                        end_line: i + 1,
                        end_column: line.len() + 1,
                        severity: Severity::Error,
                        fix: None,
                        rule_name: Some("DG001".to_string()),
                    });
                }
            }
        }

        Ok(warnings)
    }

    fn fix(&self, _ctx: &LintContext) -> Result<String, LintError> {
        Ok(String::new()) // Unfixable
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
