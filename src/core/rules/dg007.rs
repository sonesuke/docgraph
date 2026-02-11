use crate::core::config::Config;
use crate::core::types::{Diagnostic, RuleMetadata, Severity, SpecBlock};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static RE_WS: OnceLock<Regex> = OnceLock::new();
static RE_PLACEHOLDER: OnceLock<Regex> = OnceLock::new();

fn init_regex() {
    RE_WS.get_or_init(|| Regex::new(r" +").unwrap());
    RE_PLACEHOLDER.get_or_init(|| Regex::new(r"\\\{[^}]+\\\}").unwrap());
}

fn get_safe_options() -> Options {
    let mut options = Options::all();
    options.remove(Options::ENABLE_HEADING_ATTRIBUTES);
    options
}

#[derive(Debug, Clone)]
enum TemplateElement {
    Header {
        level: HeadingLevel,
        text_pattern: String,
        optional: bool,
    },
    Text {
        pattern: String,
    },
    List {
        item_patterns: Vec<String>,
        optional: bool,
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
}

struct Template {
    elements: Vec<TemplateElement>,
    _root_anchor_pattern: Regex,
}

pub fn metadata() -> RuleMetadata {
    RuleMetadata {
        code: "DG007",
        summary: "Enforce template adherence for node types",
        description: r#"Validates that nodes conform to their associated Markdown templates.

Usage Examples:

- Placeholders: Use '{...}' to match variable text (e.g., '# {Title}').
- Wildcards: Use '*' within links or IDs (e.g., '[MOD_*(*)](*)').
- Optional Sections: Append '(Optional)' to a header to make that section optional.
- Repeating Lists: A list item in the template acts as a schema. The document can have multiple items matching the pattern.
- Repeating Tables: A table row in the template acts as a schema. The document can have multiple rows matching the pattern.

Template Example:

  <a id="FR_*"></a>
  # {Title}
  
  ## Realized by
  - [MOD_*(*)](*#MOD*)
  
  ## Parameters (Optional)
  | Name | Type |
  | :--- | :--- |
  | {Name} | {Type} |"#,
    }
}

pub fn check(root: &Path, config: &Config, spec_blocks: &[SpecBlock]) -> Vec<Diagnostic> {
    init_regex();
    let mut diagnostics = Vec::new();

    for (type_name, node_config) in &config.nodes {
        if let Some(template_path) = &node_config.template {
            let full_template_path: PathBuf = if template_path.exists() {
                template_path.to_path_buf()
            } else {
                root.join(template_path)
            };

            let template_content = match fs::read_to_string(&full_template_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let template = match parse_template(&template_content) {
                Ok(t) => t,
                Err(_e) => continue,
            };

            for block in spec_blocks {
                if block.node_type == *type_name
                    && let Err(msg) = validate_block(block, &template)
                {
                    diagnostics.push(Diagnostic {
                        path: block.file_path.clone(),
                        range: crate::core::types::Range {
                            start_line: block.line_start,
                            start_col: 1,
                            end_line: block.line_end,
                            end_col: 1,
                        },
                        code: "DG007".to_string(),
                        message: format!("Template validation failed for {}: {}", block.id, msg),
                        severity: Severity::Error,
                    });
                }
            }
        }
    }

    diagnostics
}

fn parse_template(content: &str) -> Result<Template, String> {
    let parser = Parser::new_ext(content, get_safe_options());
    let mut elements = Vec::new();
    let mut root_anchor_pattern = Regex::new(".*").unwrap();

    let mut parser = parser.peekable();

    if let Some(Event::Html(html)) = parser.peek() {
        let re = Regex::new(r#"<a\s+id=["']([^"']+)["']"#).unwrap();
        if let Some(caps) = re.captures(html) {
            let id_pattern = caps.get(1).unwrap().as_str();
            let pattern_str = format!("^{}$", regex::escape(id_pattern).replace("\\*", ".*"));
            root_anchor_pattern = Regex::new(&pattern_str).map_err(|e| e.to_string())?;
            parser.next(); // Consume only if it was an anchor
        }
    }

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                elements.push(parse_header(&mut parser, level));
            }
            Event::Start(Tag::List(_)) => {
                elements.push(parse_list(&mut parser));
            }
            Event::Start(Tag::Table(_)) => {
                elements.push(parse_table(&mut parser));
            }
            Event::Text(t) => {
                let s = t.trim();
                if !s.is_empty() {
                    elements.push(TemplateElement::Text {
                        pattern: s.to_string(),
                    });
                }
            }
            _ => {}
        }
    }

    Ok(Template {
        elements,
        _root_anchor_pattern: root_anchor_pattern,
    })
}

fn parse_header<'a>(
    parser: &mut impl Iterator<Item = Event<'a>>,
    level: HeadingLevel,
) -> TemplateElement {
    let raw_text = get_event_text(parser);
    let optional = raw_text.contains("(Optional)");
    let text_pattern = raw_text.replace("(Optional)", "").trim().to_string();
    TemplateElement::Header {
        level,
        text_pattern,
        optional,
    }
}

fn parse_list<'a>(parser: &mut impl Iterator<Item = Event<'a>>) -> TemplateElement {
    let mut item_patterns = Vec::new();
    let mut depth = 1;
    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::Item) => {
                item_patterns.push(get_event_text(parser));
            }
            Event::Start(Tag::List(_)) => depth += 1,
            Event::End(TagEnd::List(_)) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
    }
    TemplateElement::List {
        item_patterns,
        optional: false,
    }
}

fn parse_table<'a>(parser: &mut impl Iterator<Item = Event<'a>>) -> TemplateElement {
    let mut headers = Vec::new();
    let mut rows = Vec::new();

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::TableHead) => {
                while let Some(event) = parser.next() {
                    match event {
                        Event::Start(Tag::TableCell) => {
                            headers.push(get_event_text(parser));
                        }
                        Event::End(TagEnd::TableHead) => break,
                        _ => {}
                    }
                }
            }
            Event::Start(Tag::TableRow) => {
                let mut row = Vec::new();
                while let Some(event) = parser.next() {
                    match event {
                        Event::Start(Tag::TableCell) => {
                            row.push(get_event_text(parser));
                        }
                        Event::End(TagEnd::TableRow) => break,
                        _ => {}
                    }
                }
                rows.push(row);
            }
            Event::End(TagEnd::Table) => break,
            _ => {}
        }
    }
    TemplateElement::Table { headers, rows }
}

fn validate_block(block: &SpecBlock, template: &Template) -> Result<(), String> {
    let parser = Parser::new_ext(&block.content, get_safe_options());
    let events: Vec<Event> = parser.collect();
    let mut event_idx = 0;
    let mut section_missing = false;

    for expected in &template.elements {
        match expected {
            TemplateElement::Header {
                level,
                text_pattern,
                optional,
            } => {
                section_missing =
                    validate_header(&events, &mut event_idx, *level, text_pattern, *optional)?;
            }
            TemplateElement::List {
                item_patterns,
                optional,
            } => {
                if !section_missing {
                    validate_list(&events, &mut event_idx, item_patterns, *optional)?;
                }
            }
            TemplateElement::Text { pattern } => {
                if !section_missing {
                    validate_text(&events, &mut event_idx, pattern)?;
                }
            }
            TemplateElement::Table { headers, rows } => {
                if !section_missing {
                    validate_table(&events, &mut event_idx, headers, rows)?;
                }
            }
        }
    }

    // After validating all template elements, check for any extra sections
    // that are not defined in the template
    for i in event_idx..events.len() {
        if let Event::Start(Tag::Heading { .. }) = &events[i] {
            // Extract the header text
            let mut header_text = String::new();
            let mut j = i + 1;
            while j < events.len() {
                match &events[j] {
                    Event::Text(t) | Event::Code(t) => header_text.push_str(t),
                    Event::End(TagEnd::Heading(_)) => break,
                    _ => {}
                }
                j += 1;
            }
            return Err(format!(
                "Unexpected section '{}' found. This section is not defined in the template.",
                header_text
            ));
        }
    }

    Ok(())
}

fn validate_header(
    events: &[Event],
    event_idx: &mut usize,
    level: HeadingLevel,
    text_pattern: &str,
    optional: bool,
) -> Result<bool, String> {
    let mut found_idx = None;
    let mut header_text = String::new();

    for i in *event_idx..events.len() {
        if let Event::Start(Tag::Heading { level: l, .. }) = &events[i] {
            if *l == level {
                let mut j = i + 1;
                while j < events.len() {
                    match &events[j] {
                        Event::Text(t) | Event::Code(t) => header_text.push_str(t),
                        Event::End(TagEnd::Heading(_)) => break,
                        _ => {}
                    }
                    j += 1;
                }
                found_idx = Some(i);
                break;
            } else if *l > level {
                continue;
            } else {
                break;
            }
        }
    }

    if let Some(idx) = found_idx {
        let normalized_header = header_text.replace("(Optional)", "").trim().to_string();
        if match_text(text_pattern, &normalized_header) {
            *event_idx = idx;
            while *event_idx < events.len() {
                if matches!(events[*event_idx], Event::End(TagEnd::Heading(_))) {
                    *event_idx += 1;
                    break;
                }
                *event_idx += 1;
            }
            Ok(false)
        } else if optional {
            Ok(true)
        } else {
            Err(format!(
                "Header text mismatch for {}. Expected '{}', found '{}'",
                level_to_str(level),
                text_pattern,
                header_text
            ))
        }
    } else if optional {
        Ok(true)
    } else {
        Err(format!("Missing required Header: {}", level_to_str(level)))
    }
}

fn validate_list(
    events: &[Event],
    event_idx: &mut usize,
    item_patterns: &[String],
    optional: bool,
) -> Result<(), String> {
    let mut found_idx = None;
    for (i, event) in events.iter().enumerate().skip(*event_idx) {
        match event {
            Event::Start(Tag::List(_)) => {
                found_idx = Some(i);
                break;
            }
            Event::Start(Tag::Heading { .. }) => break,
            _ => {}
        }
    }

    if let Some(idx) = found_idx {
        *event_idx = idx + 1;
        while *event_idx < events.len() {
            match &events[*event_idx] {
                Event::Start(Tag::Item) => {
                    *event_idx += 1;
                    let item_text = get_events_text_from_vec(events, event_idx);
                    if !item_patterns.is_empty() {
                        let matched = item_patterns
                            .iter()
                            .any(|pattern| match_text(pattern, &item_text));
                        if !matched {
                            return Err(format!(
                                "List item '{}' does not match any template pattern. Expected one of: {:?}",
                                item_text, item_patterns
                            ));
                        }
                    }
                }
                Event::End(TagEnd::List(_)) => {
                    *event_idx += 1;
                    break;
                }
                _ => {}
            }
            *event_idx += 1;
        }
        Ok(())
    } else if optional {
        Ok(())
    } else {
        Err("Missing required List".to_string())
    }
}

fn validate_text(events: &[Event], event_idx: &mut usize, pattern: &str) -> Result<(), String> {
    let mut found = false;
    for (i, event) in events.iter().enumerate().skip(*event_idx) {
        match event {
            Event::Text(t) | Event::Code(t) => {
                if match_text(pattern, t) {
                    found = true;
                    *event_idx = i + 1;
                    break;
                }
            }
            Event::Start(Tag::Heading { .. }) => break,
            _ => {}
        }
    }

    if !found {
        Err(format!("Missing required text pattern: '{}'", pattern))
    } else {
        Ok(())
    }
}

fn validate_table(
    events: &[Event],
    event_idx: &mut usize,
    expected_headers: &[String],
    expected_rows: &[Vec<String>],
) -> Result<(), String> {
    let mut found_idx = None;
    for (i, event) in events.iter().enumerate().skip(*event_idx) {
        match event {
            Event::Start(Tag::Table(_)) => {
                found_idx = Some(i);
                break;
            }
            Event::Start(Tag::Heading { .. }) => break,
            _ => {}
        }
    }

    if let Some(idx) = found_idx {
        *event_idx = idx + 1;
        let mut row_idx = 0;
        let mut in_head = false;

        while *event_idx < events.len() {
            match &events[*event_idx] {
                Event::Start(Tag::TableHead) => in_head = true,
                Event::End(TagEnd::TableHead) => in_head = false,
                Event::Start(Tag::TableRow) => {
                    if !in_head {
                        row_idx += 1;
                    }
                }
                Event::Start(Tag::TableCell) => {
                    *event_idx += 1;
                    let actual_cell = get_events_text_from_vec(events, event_idx);

                    if in_head {
                        let col_idx = count_cells_in_current_row(events, idx, *event_idx);
                        if col_idx > expected_headers.len() {
                            return Err(format!(
                                "Table header column count mismatch. Expected {}, found more",
                                expected_headers.len()
                            ));
                        }
                        let expected = &expected_headers[col_idx - 1];
                        if !match_text(expected, &actual_cell) {
                            return Err(format!(
                                "Table header mismatch at column {}. Expected pattern '{}', found '{}'",
                                col_idx, expected, actual_cell
                            ));
                        }
                    } else {
                        let col_idx = count_cells_in_current_row(events, idx, *event_idx);

                        // We are in a cell. We check if the cell value matches the expected value
                        // for this column in ANY of the allowed row templates.
                        let mut matched_any = false;
                        let mut errors = Vec::new();

                        for (expected_row_idx, expected_row) in expected_rows.iter().enumerate() {
                            if col_idx <= expected_row.len() {
                                let expected = &expected_row[col_idx - 1];
                                if match_text(expected, &actual_cell) {
                                    matched_any = true;
                                    break;
                                } else {
                                    errors.push(format!(
                                        "Row-pattern {}: '{}'",
                                        expected_row_idx + 1,
                                        expected
                                    ));
                                }
                            }
                        }

                        if !matched_any {
                            return Err(format!(
                                "Table cell mismatch at row {}, col {}. Found '{}', but it doesn't match any allowed patterns: {:?}",
                                row_idx, col_idx, actual_cell, errors
                            ));
                        }
                    }
                }
                Event::End(TagEnd::Table) => {
                    *event_idx += 1;
                    break;
                }
                _ => {}
            }
            *event_idx += 1;
        }

        // Final validation: Ensure it matched at least one row if templates exist
        if !expected_rows.is_empty() && row_idx == 0 {
            return Err("Table row count mismatch. Expected at least 1 row".to_string());
        }

        Ok(())
    } else {
        Err("Missing required Table".to_string())
    }
}

fn count_cells_in_current_row(events: &[Event], table_start: usize, current_idx: usize) -> usize {
    let mut count = 0;
    let mut i = current_idx;
    // Walk backwards to the start of the row to count preceding cells
    while i > table_start {
        match &events[i] {
            Event::Start(Tag::TableCell) => count += 1,
            Event::Start(Tag::TableRow) | Event::Start(Tag::TableHead) => break,
            _ => {}
        }
        i -= 1;
    }
    count
}

fn get_event_text<'a>(iter: &mut impl Iterator<Item = Event<'a>>) -> String {
    let mut text = String::new();
    while let Some(e) = iter.next() {
        match e {
            Event::Text(t) | Event::Code(t) => text.push_str(&t),
            Event::Start(Tag::Link { dest_url, .. }) => {
                let inner = get_event_text(iter);
                text.push_str(&format!("[{}](", inner));
                text.push_str(&dest_url);
                text.push(')');
            }
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            Event::End(TagEnd::Link)
            | Event::End(TagEnd::Heading(_))
            | Event::End(TagEnd::Item)
            | Event::End(TagEnd::TableCell) => break,
            _ => {}
        }
    }
    text
}

fn get_events_text_from_vec(events: &[Event], idx: &mut usize) -> String {
    let mut text = String::new();
    while *idx < events.len() {
        match &events[*idx] {
            Event::Text(t) | Event::Code(t) => text.push_str(t),
            Event::Start(Tag::Link { dest_url, .. }) => {
                *idx += 1;
                let inner = get_events_text_from_vec(events, idx);
                text.push_str(&format!("[{}](", inner));
                text.push_str(dest_url);
                text.push(')');
            }
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            Event::End(TagEnd::Link)
            | Event::End(TagEnd::Heading(_))
            | Event::End(TagEnd::Item)
            | Event::End(TagEnd::TableCell) => break,
            _ => {}
        }
        *idx += 1;
    }
    text
}

fn level_to_str(level: HeadingLevel) -> &'static str {
    match level {
        HeadingLevel::H1 => "H1",
        HeadingLevel::H2 => "H2",
        HeadingLevel::H3 => "H3",
        HeadingLevel::H4 => "H4",
        HeadingLevel::H5 => "H5",
        HeadingLevel::H6 => "H6",
    }
}

fn match_text(pattern: &str, target: &str) -> bool {
    let clean_p = pattern
        .chars()
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .collect::<String>();
    let clean_t = target
        .chars()
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .collect::<String>();

    let re_ws = RE_WS.get().unwrap();
    let re_placeholder = RE_PLACEHOLDER.get().unwrap();

    let p = re_ws.replace_all(clean_p.trim(), " ").to_string();
    let t = re_ws.replace_all(clean_t.trim(), " ").to_string();

    let escaped = regex::escape(&p);
    let with_placeholders = re_placeholder.replace_all(&escaped, ".+");

    // Support * as wildcard
    let final_pattern = with_placeholders.replace(r"\*", ".*");

    let regex_pattern = format!("(?s)^{}$", final_pattern);

    if let Ok(re) = Regex::new(&regex_pattern) {
        re.is_match(&t)
    } else {
        t.contains(&p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_regex() {
        RE_WS.get_or_init(|| Regex::new(r" +").unwrap());
        RE_PLACEHOLDER.get_or_init(|| Regex::new(r"\\\{[^}]+\\\}").unwrap());
    }

    #[test]
    fn test_optional_header_can_be_skipped() {
        init_regex();

        // Template: Required H3 -> Optional H3 -> Required H3
        let template_content = r#"<a id="TEST_*"></a>

## Test

Test description.

### Required Section

- [ITEM_*](*#ITEM_*)

### Optional Section (Optional)

- [OPT_*](*#OPT_*)

### Another Section

- [ANOTHER_*](*#ANOTHER_*)
"#;

        // Document: Required H3 -> (skip Optional) -> Required H3
        let block_content = r#"<a id="TEST_001"></a>

## Test

Test description.

### Required Section

- [ITEM_A](path#ITEM_A)

### Another Section

- [ANOTHER_B](path#ANOTHER_B)
"#;

        let template = parse_template(template_content).expect("Failed to parse template");

        let block = SpecBlock {
            id: "TEST_001".to_string(),
            node_type: "TEST".to_string(),
            name: Some("Test".to_string()),
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 15,
            content: block_content.to_string(),
        };

        // Should allow skipping the optional section
        let result = validate_block(&block, &template);
        assert!(
            result.is_ok(),
            "Optional header should be skippable, but got error: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_optional_header_with_list_can_be_skipped() {
        init_regex();

        // Template: Required H3 -> Optional H3 with list -> Required H3
        let template_content = r#"<a id="FR_*"></a>

## Requirement

Description.

### Realized by

- [MOD_*](*#MOD_*)

### Qualified by (Optional)

- [NFR_*](*#NFR_*)

### Codified in (Optional)

- [CC_*](*#CC_*)
"#;

        // Document: Skip "Qualified by" and go directly to "Codified in"
        let block_content = r#"<a id="FR_001"></a>

## Requirement

Description.

### Realized by

- [MOD_CORE](path#MOD_CORE)

### Codified in (Optional)

- [CC_TEST](path#CC_TEST)
"#;

        let template = parse_template(template_content).expect("Failed to parse template");

        let block = SpecBlock {
            id: "FR_001".to_string(),
            node_type: "FR".to_string(),
            name: Some("Requirement".to_string()),
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 15,
            content: block_content.to_string(),
        };

        // Should allow skipping "Qualified by (Optional)" section
        let result = validate_block(&block, &template);
        assert!(
            result.is_ok(),
            "Should allow skipping optional 'Qualified by' section, but got error: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_extra_sections_should_be_detected() {
        init_regex();

        // Template: Only defines Realized by section
        let template_content = r#"<a id="IF_*"></a>

## Test Interface

This is a test interface.

### Realized by

- [MOD_*](*#MOD_*)
"#;

        // Document: Has Realized by + extra sections (Overview, Exposed Capabilities)
        let block_content = r#"<a id="IF_TEST"></a>

## Test Interface

This is a test interface.

### Realized by

- [MOD_CORE](path#MOD_CORE)

### Overview

This is an overview section.

### Exposed Capabilities

This is an exposed capabilities section.
"#;

        let template = parse_template(template_content).expect("Failed to parse template");

        let block = SpecBlock {
            id: "IF_TEST".to_string(),
            node_type: "IF".to_string(),
            name: Some("Test Interface".to_string()),
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 20,
            content: block_content.to_string(),
        };

        // Should detect extra sections not defined in template
        let result = validate_block(&block, &template);
        assert!(
            result.is_err(),
            "Should detect extra sections not in template, but validation passed"
        );

        if let Err(e) = result {
            assert!(
                e.contains("Overview") || e.contains("extra") || e.contains("unexpected"),
                "Error message should mention the extra section, got: {}",
                e
            );
        }
    }
    #[test]
    fn test_extra_h2_sections_should_be_detected() {
        init_regex();

        // Template: Only defines Realized by section
        let template_content = r#"<a id="IF_*"></a>

## Test Interface

This is a test interface.

### Realized by

- [MOD_*](*#MOD_*)
"#;

        // Document: Has Realized by + extra H2 section
        let block_content = r#"<a id="IF_TEST_H2"></a>

## Test Interface

This is a test interface.

### Realized by

- [MOD_CORE](path#MOD_CORE)

## Extra H2 Section

This should be detected as unexpected.
"#;

        let template = parse_template(template_content).expect("Failed to parse template");

        let block = SpecBlock {
            id: "IF_TEST_H2".to_string(),
            node_type: "IF".to_string(),
            name: Some("Test Interface".to_string()),
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 20,
            content: block_content.to_string(),
        };

        // Should detect extra H2 sections not defined in template
        let result = validate_block(&block, &template);
        assert!(
            result.is_err(),
            "Should detect extra H2 sections not in template, but validation passed"
        );

        if let Err(e) = result {
            assert!(
                e.contains("Extra H2 Section") || e.contains("extra") || e.contains("unexpected"),
                "Error message should mention the extra section, got: {}",
                e
            );
        }
    }
    #[test]
    fn test_missing_text_should_be_detected() {
        init_regex();

        // Template: H1 -> Text({Description}) -> H2
        let template_content = r#"<a id="ADR_*"></a>

# {Title}

{Description}

## Decision
"#;

        // Document: H1 -> (Missing Text) -> H2
        let block_content = r#"<a id="ADR_001"></a>

# My Title

## Decision
"#;

        let template = parse_template(template_content).expect("Failed to parse template");

        let block = SpecBlock {
            id: "ADR_001".to_string(),
            node_type: "ADR".to_string(),
            name: Some("My Title".to_string()),
            edges: vec![],
            file_path: "adr.md".into(),
            line_start: 1,
            line_end: 10,
            content: block_content.to_string(),
        };

        // Should detect missing text
        let result = validate_block(&block, &template);
        assert!(
            result.is_err(),
            "Should detect missing text, but validation passed"
        );

        if let Err(e) = result {
            assert!(
                e.contains("Missing required text")
                    || e.contains("Pattern not found")
                    || e.contains("Header text mismatch"),
                "Error message should mention missing text or mismatch, got: {}",
                e
            );
        }
    }

    #[test]
    fn test_table_formatting_flexibility() {
        init_regex();

        // Template: Table with minimal formatting
        let template_content = r#"<a id="TBL_*"></a>

| col1 | col2 |
|---|---|
| {val1} | {val2} |
"#;

        // Document: Table with expanded formatting
        let block_content = r#"<a id="TBL_001"></a>

| col1     | col2     |
|----------|----------|
| value A  | value B  |
"#;

        let template = parse_template(template_content).expect("Failed to parse template");

        let block = SpecBlock {
            id: "TBL_001".to_string(),
            node_type: "TBL".to_string(),
            name: Some("Table Test".to_string()),
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 10,
            content: block_content.to_string(),
        };

        // This should pass if table formatting is ignored or handled flexibly
        let result = validate_block(&block, &template);
        assert!(
            result.is_ok(),
            "Table formatting should be flexible, but got error: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_header_placeholder_support() {
        init_regex();

        // Template used in docgraph (simplified):
        // ## {Title}
        let template_content = r#"<a id="TEST_*"></a>

## {Title}

Some content.
"#;

        // Document
        let block_content = r#"<a id="TEST_001"></a>

## Actual Title

Some content.
"#;

        let template = parse_template(template_content).expect("Failed to parse template");

        // Verify that the header pattern was parsed correctly and is NOT empty
        if let Some(TemplateElement::Header { text_pattern, .. }) = template.elements.first() {
            assert!(
                !text_pattern.is_empty(),
                "Header pattern should not be empty"
            );
            assert!(
                text_pattern.contains("{Title}"),
                "Header pattern should contain {{Title}}, found: '{}'",
                text_pattern
            );
        } else {
            panic!("First element should be a Header");
        }

        let block = SpecBlock {
            id: "TEST_001".to_string(),
            node_type: "TEST".to_string(),
            name: Some("Actual Title".to_string()),
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 10,
            content: block_content.to_string(),
        };

        let result = validate_block(&block, &template);
        assert!(
            result.is_ok(),
            "Validation should pass for header with placeholder, but got: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_table_repeating_rows() {
        init_regex();

        // Template: Table with 1 row (schema)
        let template_content = r#"<a id="TBL_*"></a>
| col1 | col2 |
|---|---|
| {val1} | {val2} |
"#;

        // Document: Table with 3 matching rows
        let block_content = r#"<a id="TBL_001"></a>
| col1 | col2 |
|---|---|
| v1-1 | v1-2 |
| v2-1 | v2-2 |
| v3-1 | v3-2 |
"#;

        let template = parse_template(template_content).expect("Failed to parse template");
        let block = SpecBlock {
            id: "TBL_001".to_string(),
            node_type: "TBL".to_string(),
            name: None,
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 10,
            content: block_content.to_string(),
        };

        // Should pass with multiple matching rows
        let result = validate_block(&block, &template);
        assert!(
            result.is_ok(),
            "Table should allow multiple rows matching a single-row template, but got error: {:?}",
            result.err()
        );

        // Document: Table with a mismatching row
        let block_content_fail = r#"<a id="TBL_001"></a>
| col1 | col2 |
|---|---|
| v1-1 | v1-2 |
| mismatch |
"#;
        let block_fail = SpecBlock {
            id: "TBL_001".to_string(),
            node_type: "TBL".to_string(),
            name: None,
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 10,
            content: block_content_fail.to_string(),
        };

        let result_fail = validate_block(&block_fail, &template);
        assert!(
            result_fail.is_err(),
            "Table should fail if a row doesn't match the column count"
        );
    }

    #[test]
    fn test_table_strict_multiple_rows_become_flexible() {
        init_regex();

        // Template: Table with 2 specific allowed row patterns
        let template_content = r#"<a id="TBL_*"></a>
| col1 | col2 |
|---|---|
| rowA | rowA |
| rowB | rowB |
"#;

        let template = parse_template(template_content).expect("Failed to parse template");

        // Document: Table with matching 2 rows
        let block_content = r#"<a id="TBL_001"></a>
| col1 | col2 |
|---|---|
| rowA | rowA |
| rowB | rowB |
"#;
        let block = SpecBlock {
            id: "TBL_001".to_string(),
            node_type: "TBL".to_string(),
            name: None,
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 10,
            content: block_content.to_string(),
        };

        assert!(validate_block(&block, &template).is_ok());

        // Document: Table with 3 rows (now allowed, as long as each matches one pattern)
        let block_content_extra = r#"<a id="TBL_001"></a>
| col1 | col2 |
|---|---|
| rowA | rowA |
| rowB | rowB |
| rowA | rowA |
"#;
        let block_extra = SpecBlock {
            id: "TBL_001".to_string(),
            node_type: "TBL".to_string(),
            name: None,
            edges: vec![],
            file_path: "test.md".into(),
            line_start: 1,
            line_end: 10,
            content: block_content_extra.to_string(),
        };

        let result_extra = validate_block(&block_extra, &template);
        assert!(
            result_extra.is_ok(),
            "Should now allow extra rows if they match any template pattern"
        );
    }
}
