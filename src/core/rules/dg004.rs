use crate::core::types::{Diagnostic, Range, Severity, SpecBlock};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn check_link_text(files: &[PathBuf], blocks: &[SpecBlock]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Build a map of ID to Title from SpecBlocks (O(1) lookup)
    let mut id_to_title = HashMap::new();
    for block in blocks {
        if let Some(name) = &block.name {
            id_to_title.insert(block.id.clone(), name.clone());
        }
    }

    // Regex for [text](#ID) or [text](path#ID)
    // Captures: 1=text, 2=path#ID, 3=ID
    let link_re = regex::Regex::new(r"\[([^\]]*)\]\(([^)]*#([^)]+))\)").unwrap();

    // Iterate files once (O(N) I/O)
    for file_path in files {
        if let Ok(content) = fs::read_to_string(file_path) {
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

            // Strip code blocks to avoid false positives
            let clean_content = crate::core::utils::strip_markdown_code(&content);

            for caps in link_re.captures_iter(&clean_content) {
                let text_match = caps.get(1).unwrap();
                let id_match = caps.get(3).unwrap();

                let text_range = text_match.range();
                let id_range = id_match.range();

                let current_text = &content[text_range.clone()];
                let target_id = &content[id_range];

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
                        c == ' '
                            || c == ':'
                            || c == '-'
                            || c == '('
                            || c == '['
                            || c == ']'
                            || c == ')'
                    });
                    let clean_title = clean_title.trim_end_matches(|c: char| {
                        c == ' '
                            || c == ':'
                            || c == '-'
                            || c == ')'
                            || c == ']'
                            || c == '('
                            || c == '['
                    });
                    let clean_title = clean_title.trim().to_string();

                    let expected_text = if clean_title.is_empty() {
                        target_id.to_string()
                    } else {
                        format!("{} ({})", target_id, clean_title)
                    };

                    if current_text != expected_text {
                        let (start_line, start_col) = get_pos(text_range.start);
                        let (end_line, end_col) = get_pos(text_range.end);

                        diagnostics.push(Diagnostic {
                            code: "DG004".to_string(),
                            message: format!(
                                "Link text for '{}' should be '{}' but found '{}'",
                                target_id, expected_text, current_text
                            ),
                            path: file_path.clone(),
                            range: Range {
                                start_line,
                                start_col,
                                end_line,
                                end_col,
                            },
                            severity: Severity::Error,
                        });
                    }
                }
            }
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumdl_lib::config::MarkdownFlavor;

    fn index_content(content: &str, path: &Path) -> FileIndex {
        let rule = DG004;
        let rules: Vec<Box<dyn rumdl_lib::rule::Rule>> = vec![Box::new(rule)];
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
    fn test_dg004_link_text() {
        use tempfile::tempdir;
        let dir = tempdir().unwrap();

        let p1 = dir.path().join("defs.md");
        let p2 = dir.path().join("links.md");

        // Define ID-1 with Title "Start"
        let c1 = "<a id=\"ID-1\"></a>\n# Start\n";
        std::fs::write(&p1, c1).unwrap();

        // Link to ID-1 with correct and incorrect text
        let c2 = "Correct: [ID-1 (Start)](#ID-1)\nIncorrect: [Click here](#ID-1)";
        std::fs::write(&p2, c2).unwrap();

        // Index both
        let idx1 = index_content(c1, &p1);
        let idx2 = index_content(c2, &p2);

        let mut ws = WorkspaceIndex::new();
        ws.insert_file(p1.clone(), idx1);
        ws.insert_file(p2.clone(), idx2.clone());

        let rule = DG004;
        let warnings = rule.cross_file_check(&p2, &idx2, &ws).unwrap();

        // Should have 1 warning for "Click here"
        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0]
                .message
                .contains("Link text for 'ID-1' should be 'ID-1 (Start)'")
        );
    }

    #[test]
    fn test_dg004_cleaned_title() {
        use tempfile::tempdir;
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("complex.md");

        // Title has ID inside it often: "ID-2 (Complex Title)"
        let c1 = "## ID-2 (Complex Title) <a id=\"ID-2\"></a>";
        std::fs::write(&p1, c1).unwrap();

        let c2 = "[ID-2 (Complex Title)](#ID-2)"; // Correct
        let p2 = dir.path().join("test.md");
        std::fs::write(&p2, c2).unwrap();

        let idx1 = index_content(c1, &p1);
        let idx2 = index_content(c2, &p2);

        let mut ws = WorkspaceIndex::new();
        ws.insert_file(p1.clone(), idx1);
        ws.insert_file(p2.clone(), idx2.clone());

        let rule = DG004;
        let warnings = rule.cross_file_check(&p2, &idx2, &ws).unwrap();
        assert!(warnings.is_empty());
    }
}
