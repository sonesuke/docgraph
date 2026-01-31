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

        // Strip code blocks and inline code to avoid false positives
        let clean_content = crate::core::utils::strip_markdown_code(&content);

        for caps in link_re.captures_iter(&clean_content) {
            let url_range = caps.get(2).unwrap().range();
            let url_part = &content[url_range.clone()];

            // Case 1: Implicit local link (#ID)
            if let Some(target_id) = url_part.strip_prefix('#') {
                let (start_line, start_col) = get_pos(url_range.start);
                let (end_line, end_col) = get_pos(url_range.end);

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

#[cfg(test)]
mod tests {
    use super::*;
    use rumdl_lib::config::MarkdownFlavor;
    use tempfile::tempdir;

    use crate::core::rules::dg002::DG002;

    fn index_content(content: &str, path: &Path) -> FileIndex {
        let rule_dg003 = DG003;
        let rule_dg002 = DG002; // DG002 contributes anchors to index
        let rules: Vec<Box<dyn rumdl_lib::rule::Rule>> =
            vec![Box::new(rule_dg003), Box::new(rule_dg002)];
        let (_, index) = rumdl_lib::lint_and_index(
            content,
            &rules,
            false,
            MarkdownFlavor::Standard,
            Some(path.to_path_buf()),
            None,
        );
        index
    }

    #[test]
    fn test_dg003_valid_local_link() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.md");

        let content = r#"<a id="ID-1"></a>
Link to [myself](#ID-1)"#;
        std::fs::write(&path, content).unwrap();

        let idx = index_content(content, &path);

        let mut ws = WorkspaceIndex::new();
        ws.insert_file(path.clone(), idx.clone());

        let rule = DG003;
        let warnings = rule.cross_file_check(&path, &idx, &ws).unwrap();
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_dg003_valid_cross_file_link() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("file1.md");
        let p2 = dir.path().join("file2.md");

        let c1 = r#"Link to [other](file2.md#ID-2)"#;
        let c2 = r#"<a id="ID-2"></a>"#;

        std::fs::write(&p1, c1).unwrap();
        std::fs::write(&p2, c2).unwrap();

        let idx1 = index_content(c1, &p1);
        let idx2 = index_content(c2, &p2);

        let mut ws = WorkspaceIndex::new();
        ws.insert_file(p1.clone(), idx1.clone());
        ws.insert_file(p2.clone(), idx2.clone());

        let rule = DG003;
        let warnings = rule.cross_file_check(&p1, &idx1, &ws).unwrap();
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_dg003_broken_link() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.md");

        // Write content to disk because DG003 reads it
        let content = r#"Link to [nowhere](#MISSING)"#;
        std::fs::write(&path, content).unwrap();

        let idx = index_content(content, &path);
        let mut ws = WorkspaceIndex::new();
        ws.insert_file(path.clone(), idx.clone());

        let rule = DG003;
        let warnings = rule.cross_file_check(&path, &idx, &ws).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("MISSING anchor ID"));
    }

    #[test]
    fn test_dg003_ambiguous_link() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("file1.md");
        let p2 = dir.path().join("file2.md");

        let c1 = r#"Link to [implicit](#ID-REMOTE)"#;
        let c2 = r#"<a id="ID-REMOTE"></a>"#;

        std::fs::write(&p1, c1).unwrap();
        std::fs::write(&p2, c2).unwrap();

        let idx1 = index_content(c1, &p1);
        let idx2 = index_content(c2, &p2);

        let mut ws = WorkspaceIndex::new();
        ws.insert_file(p1.clone(), idx1.clone());
        ws.insert_file(p2.clone(), idx2.clone());

        let rule = DG003;
        let warnings = rule.cross_file_check(&p1, &idx1, &ws).unwrap();

        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("defined in another file"));
        assert!(warnings[0].fix.is_some());
    }

    #[test]
    fn test_dg003_explicit_link_missing_file() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("file1.md");
        let content = r#"Link to [nowhere](missing.md#ID)"#;
        std::fs::write(&p1, content).unwrap();

        let idx = index_content(content, &p1);
        let mut ws = WorkspaceIndex::new();
        ws.insert_file(p1.clone(), idx.clone());

        let rule = DG003;
        // Should not panic, just ignore
        let warnings = rule.cross_file_check(&p1, &idx, &ws).unwrap();
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_dg003_explicit_link_ambiguous() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("file1.md");
        // Linking to file2.md#COMMON, but file2.md doesn't have it.
        // Both file3.md and file4.md DO have it.
        let p2 = dir.path().join("file2.md");
        let p3 = dir.path().join("file3.md");
        let p4 = dir.path().join("file4.md");

        let c1 = r#"Link to [ambiguous](file2.md#COMMON)"#;
        let c2 = "";
        let c3 = r#"<a id="COMMON"></a>"#;
        let c4 = r#"<a id="COMMON"></a>"#;

        std::fs::write(&p1, c1).unwrap();
        std::fs::write(&p2, c2).unwrap();
        std::fs::write(&p3, c3).unwrap();
        std::fs::write(&p4, c4).unwrap();

        let idx1 = index_content(c1, &p1);
        let idx2 = index_content(c2, &p2);
        let idx3 = index_content(c3, &p3);
        let idx4 = index_content(c4, &p4);

        let mut ws = WorkspaceIndex::new();
        ws.insert_file(p1.clone(), idx1.clone());
        ws.insert_file(p2.clone(), idx2.clone());
        ws.insert_file(p3.clone(), idx3.clone());
        ws.insert_file(p4.clone(), idx4.clone());

        let rule = DG003;
        let warnings = rule.cross_file_check(&p1, &idx1, &ws).unwrap();

        // Should be 1 warning about being found in multiple files
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("found in multiple files"));
        assert!(warnings[0].fix.is_none());
    }

    #[test]
    fn test_dg003_read_error() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let dir = tempdir().unwrap();
            let p1 = dir.path().join("unreadable.md");
            std::fs::write(&p1, "content").unwrap();

            let mut perms = std::fs::metadata(&p1).unwrap().permissions();
            perms.set_mode(0o000);
            std::fs::set_permissions(&p1, perms).unwrap();

            let idx = index_content("content", &p1);
            let mut ws = WorkspaceIndex::new();
            ws.insert_file(p1.clone(), idx.clone());

            let rule = DG003;
            let warnings = rule.cross_file_check(&p1, &idx, &ws).unwrap();
            assert!(warnings.is_empty()); // Should just return empty on read error
        }
    }
}
