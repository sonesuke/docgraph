use crate::core::config::Config;
use crate::core::types::{Diagnostic, Severity, SpecBlock};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use regex::Regex;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

static RE_WS: OnceLock<Regex> = OnceLock::new();
static RE_PLACEHOLDER: OnceLock<Regex> = OnceLock::new();

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
}

struct Template {
    elements: Vec<TemplateElement>,
    _root_anchor_pattern: Regex,
}

pub fn check_templates(root: &Path, spec_blocks: &[SpecBlock], config: &Config) -> Vec<Diagnostic> {
    RE_WS.get_or_init(|| Regex::new(r" +").unwrap());
    RE_PLACEHOLDER.get_or_init(|| Regex::new(r"\\\{[^}]+\\\}").unwrap());
    let mut diagnostics = Vec::new();

    for (type_name, node_type) in &config.node_types {
        if let Some(template_path) = &node_type.template {
            let full_template_path = if template_path.exists() {
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
    let mut parser = Parser::new(content);
    let mut elements = Vec::new();
    let mut root_anchor_pattern = Regex::new(".*").unwrap();

    if let Some(Event::Html(html)) = parser.next() {
        let re = Regex::new(r#"<a\s+id=["']([^"']+)["']"#).unwrap();
        if let Some(caps) = re.captures(&html) {
            let id_pattern = caps.get(1).unwrap().as_str();
            let pattern_str = format!("^{}$", regex::escape(id_pattern).replace("\\*", ".*"));
            root_anchor_pattern = Regex::new(&pattern_str).map_err(|e| e.to_string())?;
        }
    }

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let raw_text = get_event_text(&mut parser);
                let optional = raw_text.contains("(Optional)");
                let text_pattern = raw_text.replace("(Optional)", "").trim().to_string();
                elements.push(TemplateElement::Header {
                    level,
                    text_pattern,
                    optional,
                });
            }
            Event::Start(Tag::List(_)) => {
                let mut item_patterns = Vec::new();
                let mut depth = 1;
                while let Some(event) = parser.next() {
                    match event {
                        Event::Start(Tag::Item) => {
                            item_patterns.push(get_event_text(&mut parser));
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
                elements.push(TemplateElement::List {
                    item_patterns,
                    optional: false,
                });
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

fn validate_block(block: &SpecBlock, template: &Template) -> Result<(), String> {
    let parser = Parser::new(&block.content);
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
                // Look ahead to find a header at the expected level
                let mut found_idx = None;
                let mut header_text = String::new();
                
                for i in event_idx..events.len() {
                    if let Event::Start(Tag::Heading { level: l, .. }) = &events[i] {
                        if *l == *level {
                            // Extract the header text
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
                        } else if *l > *level {
                            // It's a subsection (e.g. H3 inside H2 search), continue looking
                            continue;
                        } else {
                            // Hit a same or higher level heading (e.g. H1 inside H2 search) - stop looking
                            break;
                        }
                    }
                }

                if let Some(idx) = found_idx {
                    let normalized_header = header_text.replace("(Optional)", "").trim().to_string();
                    if match_text(text_pattern, &normalized_header) {
                        // Header matches - consume events up to and including this header
                        section_missing = false;
                        event_idx = idx;
                        // Skip to end of this heading
                        while event_idx < events.len() {
                            if matches!(events[event_idx], Event::End(TagEnd::Heading(_))) {
                                event_idx += 1;
                                break;
                            }
                            event_idx += 1;
                        }
                    } else if *optional {
                        // Header found but doesn't match, and it's optional - don't consume, mark as missing
                        section_missing = true;
                    } else {
                        // Header found but doesn't match, and it's required - error
                        return Err(format!(
                            "Header text mismatch for {}. Expected '{}', found '{}'",
                            level_to_str(*level),
                            text_pattern,
                            header_text
                        ));
                    }
                } else if *optional {
                    // Header not found and it's optional - skip
                    section_missing = true;
                } else {
                    // Header not found and it's required - error
                    return Err(format!("Missing required Header: {}", level_to_str(*level)));
                }
            }
            TemplateElement::List {
                item_patterns,
                optional,
            } => {
                if section_missing {
                    continue;
                }
                
                // Look for a list
                let mut found_idx = None;
                for i in event_idx..events.len() {
                    match &events[i] {
                        Event::Start(Tag::List(_)) => {
                            found_idx = Some(i);
                            break;
                        }
                        Event::Start(Tag::Heading { .. }) => break, // Hit next header
                        _ => {}
                    }
                }

                if let Some(idx) = found_idx {
                    event_idx = idx + 1; // Skip Start(List)
                    
                    while event_idx < events.len() {
                        match &events[event_idx] {
                            Event::Start(Tag::Item) => {
                                event_idx += 1;
                                let mut item_text = String::new();
                                while event_idx < events.len() {
                                    match &events[event_idx] {
                                        Event::Text(t) | Event::Code(t) => item_text.push_str(t),
                                        Event::Start(Tag::Link { dest_url, .. }) => {
                                            item_text.push('[');
                                            event_idx += 1;
                                            while event_idx < events.len() {
                                                match &events[event_idx] {
                                                    Event::Text(t) | Event::Code(t) => item_text.push_str(t),
                                                    Event::End(TagEnd::Link) => break,
                                                    _ => {}
                                                }
                                                event_idx += 1;
                                            }
                                            item_text.push_str("](");
                                            item_text.push_str(dest_url);
                                            item_text.push(')');
                                        }
                                        Event::End(TagEnd::Item) => break,
                                        _ => {}
                                    }
                                    event_idx += 1;
                                }
                                
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
                                event_idx += 1;
                                break;
                            }
                            _ => {}
                        }
                        event_idx += 1;
                    }
                } else if !*optional {
                    return Err("Missing required List".to_string());
                }
            }
            TemplateElement::Text { pattern } => {
                if section_missing {
                    continue;
                }
                
                // Look for matching text
                let mut found = false;
                for i in event_idx..events.len() {
                    match &events[i] {
                        Event::Text(t) | Event::Code(t) => {
                            if match_text(pattern, t) {
                                found = true;
                                event_idx = i + 1;
                                break;
                            }
                        }
                        Event::Start(Tag::Heading { .. }) => break,
                        _ => {}
                    }
                }
                
                if !found {
                    return Err(format!("Missing required text pattern: '{}'", pattern));
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
            Event::End(TagEnd::Link)
            | Event::End(TagEnd::Heading(_))
            | Event::End(TagEnd::Item) => break,
            _ => {}
        }
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

    #[test]
    fn test_optional_header_can_be_skipped() {
        // Initialize regex patterns
        RE_WS.get_or_init(|| Regex::new(r" +").unwrap());
        RE_PLACEHOLDER.get_or_init(|| Regex::new(r"\{[^}]+\}").unwrap());

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
        assert!(result.is_ok(), "Optional header should be skippable, but got error: {:?}", result.err());
    }

    #[test]
    fn test_optional_header_with_list_can_be_skipped() {
        // Initialize regex patterns
        RE_WS.get_or_init(|| Regex::new(r" +").unwrap());
        RE_PLACEHOLDER.get_or_init(|| Regex::new(r"\{[^}]+\}").unwrap());

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
        assert!(result.is_ok(), "Should allow skipping optional 'Qualified by' section, but got error: {:?}", result.err());
    }

    #[test]
    fn test_extra_sections_should_be_detected() {
        // Initialize regex patterns
        RE_WS.get_or_init(|| Regex::new(r" +").unwrap());
        RE_PLACEHOLDER.get_or_init(|| Regex::new(r"\{[^}]+\}").unwrap());

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
        assert!(result.is_err(), "Should detect extra sections not in template, but validation passed");
        
        if let Err(e) = result {
            assert!(e.contains("Overview") || e.contains("extra") || e.contains("unexpected"), 
                "Error message should mention the extra section, got: {}", e);
        }
    }
    #[test]
    fn test_extra_h2_sections_should_be_detected() {
        // Initialize regex patterns
        RE_WS.get_or_init(|| Regex::new(r" +").unwrap());
        RE_PLACEHOLDER.get_or_init(|| Regex::new(r"\{[^}]+\}").unwrap());

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
        assert!(result.is_err(), "Should detect extra H2 sections not in template, but validation passed");
        
        if let Err(e) = result {
            assert!(e.contains("Extra H2 Section") || e.contains("extra") || e.contains("unexpected"), 
                "Error message should mention the extra section, got: {}", e);
        }
    }
    #[test]
    fn test_missing_text_should_be_detected() {
        // Initialize regex patterns
        RE_WS.get_or_init(|| Regex::new(r" +").unwrap());
        RE_PLACEHOLDER.get_or_init(|| Regex::new(r"\\\{[^}]+\\\}").unwrap());

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
        assert!(result.is_err(), "Should detect missing text, but validation passed");
        
        if let Err(e) = result {
            assert!(e.contains("Missing required text") || e.contains("Pattern not found"), 
                "Error message should mention missing text, got: {}", e);
        }
    }
}
