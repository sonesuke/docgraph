use regex::Regex;

/// Replace Markdown code blocks and inline code with spaces to prevent false positives while preserving line/col positions.
pub fn strip_markdown_code(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_code_fence = false;
    let mut current_fence_char = '`';

    let inline_re = Regex::new(r"`[^`]+`").unwrap();

    for line in content.lines() {
        let trimmed = line.trim();

        // Handle code fences
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            let fence_char = if trimmed.starts_with('`') { '`' } else { '~' };
            if !in_code_fence {
                in_code_fence = true;
                current_fence_char = fence_char;
                // Keep the fence line as is (or space it out)
                result.push_str(&" ".repeat(line.len()));
            } else if fence_char == current_fence_char {
                in_code_fence = false;
                result.push_str(&" ".repeat(line.len()));
            } else {
                // Inside a fence, but found a different fence char (e.g. ~~~ inside ```)
                result.push_str(&" ".repeat(line.len()));
            }
        } else if in_code_fence {
            // Inside code fence, replace whole line with spaces
            result.push_str(&" ".repeat(line.len()));
        } else {
            // Outside code fence, handle inline code

            // We need to be careful with column alignment.
            // We replace matched parts with equal number of spaces.
            let mut clean_line = line.to_string();
            for cap in inline_re.captures_iter(line) {
                let m = cap.get(0).unwrap();
                let start = m.start();
                let end = m.end();
                let length = end - start;
                clean_line.replace_range(start..end, &" ".repeat(length));
            }
            result.push_str(&clean_line);
        }
        result.push('\n');
    }

    // Match original trailing newline behavior
    if !content.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_inline_code() {
        let input = "This is `inline code` and normal text.";
        // original: "This is `inline code` and normal text."
        // replaced: "This is              and normal text."
        // spaces:   123456781234567890123
        let expected = "This is               and normal text.";
        assert_eq!(strip_markdown_code(input), expected);
    }

    #[test]
    fn test_strip_code_fence() {
        let input = "Text\n```rust\nlet x = 1;\n```\nMore text";
        let output = strip_markdown_code(input);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[0], "Text");
        assert_eq!(lines[1], "       "); // ```rust
        assert_eq!(lines[2], "          "); // let x = 1;
        assert_eq!(lines[3], "   "); // ```
        assert_eq!(lines[4], "More text");
    }

    #[test]
    fn test_multiple_inline() {
        let input = "One `code` and `another` code.";
        let expected = "One        and           code.";
        assert_eq!(strip_markdown_code(input), expected);
    }
}
