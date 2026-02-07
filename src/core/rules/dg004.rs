use crate::core::types::{Diagnostic, Range, Severity, SpecBlock};
use pulldown_cmark::{Event, LinkType, Options, Parser, Tag};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

static RE_WS_SINGLE: OnceLock<regex::Regex> = OnceLock::new();

pub fn check_link_text(files: &[PathBuf], blocks: &[SpecBlock]) -> Vec<Diagnostic> {
    RE_WS_SINGLE.get_or_init(|| regex::Regex::new(r" +").unwrap());
    let mut diagnostics = Vec::new();

    // Build a map of ID to Title from SpecBlocks (O(1) lookup)
    let mut id_to_title = HashMap::new();
    for block in blocks {
        if let Some(name) = &block.name {
            id_to_title.insert(block.id.clone(), name.clone());
        }
    }

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

            let parser = Parser::new_ext(&content, Options::all()).into_offset_iter();

            for (event, range) in parser {
                if let Event::Start(Tag::Link {
                    link_type,
                    dest_url,
                    ..
                }) = event
                {
                    // Only check inline, reference, or shortcut links with fragments
                    if matches!(
                        link_type,
                        LinkType::Inline | LinkType::Reference | LinkType::Shortcut
                    ) {
                        let dest = dest_url.as_ref();
                        // Check if it's a link to an ID (either #ID or path/to/file#ID)
                        if let Some(hash_idx) = dest.find('#') {
                            let target_id = &dest[hash_idx + 1..];
                            if target_id.is_empty() {
                                continue;
                            }

                            if let Some(title) = id_to_title.get(target_id) {
                                // Extract the display text from the range
                                // Note: pulldown-cmark range covers the whole link [text](url) or [text][ref]
                                // We need to extract just the text part.
                                // For standard links [text](url), regex helper is safest/easiest as pulldown doesn't give inner text range directly in Start event
                                let full_link_str = &content[range.clone()];
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

                                if let Some(open_bracket) = full_link_str.find('[')
                                    && let Some(close_bracket) = full_link_str.rfind(']')
                                {
                                    let current_text =
                                        &full_link_str[open_bracket + 1..close_bracket];

                                    let clean_current = current_text
                                        .chars()
                                        .map(|c| if c.is_whitespace() { ' ' } else { c })
                                        .collect::<String>();
                                    let clean_expected = expected_text
                                        .chars()
                                        .map(|c| if c.is_whitespace() { ' ' } else { c })
                                        .collect::<String>();

                                    let re_ws_single = RE_WS_SINGLE.get().unwrap();
                                    let c_norm = re_ws_single
                                        .replace_all(clean_current.trim(), " ")
                                        .to_lowercase();
                                    let e_norm = re_ws_single
                                        .replace_all(clean_expected.trim(), " ")
                                        .to_lowercase();

                                    if c_norm != e_norm
                                        && !c_norm.contains(&e_norm)
                                        && !e_norm.contains(&c_norm)
                                    {
                                        let (start_line, start_col) =
                                            get_pos(range.start + open_bracket + 1);
                                        let (end_line, end_col) =
                                            get_pos(range.start + close_bracket);

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
                }
            }
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parse::extract_all;

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

        // Extract blocks to simulate Pass 3 collection
        let (blocks1, _) = extract_all(c1, &p1);
        let (blocks2, _) = extract_all(c2, &p2); // No blocks here but checking logical consistency

        let mut all_blocks = Vec::new();
        all_blocks.extend(blocks1);
        all_blocks.extend(blocks2);

        // Run checking
        let files = vec![p2.clone()];
        let warnings = check_link_text(&files, &all_blocks);

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
        let p2 = dir.path().join("test.md");

        // Title has ID inside it often: "ID-2 (Complex Title)"
        let c1 = "## ID-2 (Complex Title) <a id=\"ID-2\"></a>";
        std::fs::write(&p1, c1).unwrap();

        let c2 = "[ID-2 (Complex Title)](#ID-2)"; // Correct
        std::fs::write(&p2, c2).unwrap();

        let (blocks1, _) = extract_all(c1, &p1);
        let (blocks2, _) = extract_all(c2, &p2);

        let mut all_blocks = Vec::new();
        all_blocks.extend(blocks1);
        all_blocks.extend(blocks2);

        let files = vec![p2.clone()];
        let warnings = check_link_text(&files, &all_blocks);

        assert!(warnings.is_empty());
    }
}
