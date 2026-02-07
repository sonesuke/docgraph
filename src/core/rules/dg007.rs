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
                let text = get_event_text(&mut parser);
                let optional = text.contains("(Optional)");
                elements.push(TemplateElement::Header {
                    level,
                    text_pattern: text,
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
    let mut block_events = parser.into_iter().peekable();
    let mut section_missing = false;

    for expected in &template.elements {
        match expected {
            TemplateElement::Header {
                level,
                text_pattern,
                optional,
            } => {
                let mut found = false;
                while let Some(e) = block_events.peek() {
                    match e {
                        Event::Start(Tag::Heading { level: l, .. }) if *l == *level => {
                            found = true;
                            break;
                        }
                        _ => {
                            block_events.next();
                        }
                    }
                }

                if found {
                    section_missing = false;
                    block_events.next(); // Consume Start(Heading)
                    let text = get_event_text(&mut block_events);
                    if !match_text(text_pattern, &text) {
                        return Err(format!(
                            "Header text mismatch for {}. Expected '{}', found '{}'",
                            level_to_str(*level),
                            text_pattern,
                            text
                        ));
                    }
                } else if !optional {
                    return Err(format!("Missing required Header: {}", level_to_str(*level)));
                } else {
                    section_missing = true;
                }
            }
            TemplateElement::List {
                item_patterns,
                optional,
            } => {
                if section_missing {
                    continue;
                }
                let mut found = false;
                while let Some(e) = block_events.peek() {
                    match e {
                        Event::Start(Tag::List(_)) => {
                            found = true;
                            break;
                        }
                        Event::Start(Tag::Heading { .. }) => break, // Hit next header
                        _ => {
                            block_events.next();
                        }
                    }
                }

                if found {
                    block_events.next(); // Consume Start(List)
                    while let Some(e) = block_events.next() {
                        match e {
                            Event::Start(Tag::Item) => {
                                let item_text = get_event_text(&mut block_events);
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
                            Event::End(TagEnd::List(_)) => break,
                            _ => {}
                        }
                    }
                } else if !optional {
                    return Err("Missing required List".to_string());
                }
            }
            TemplateElement::Text { pattern } => {
                if section_missing {
                    continue;
                }
                let mut found = false;
                while let Some(e) = block_events.peek() {
                    match e {
                        Event::Text(t) | Event::Code(t) => {
                            if match_text(pattern, t) {
                                found = true;
                                break;
                            }
                            block_events.next();
                        }
                        Event::Start(Tag::Heading { .. }) => break,
                        _ => {
                            block_events.next();
                        }
                    }
                }
                if found {
                    block_events.next();
                }
            }
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
    // use super::*;
}
