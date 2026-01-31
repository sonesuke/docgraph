use crate::core::types;

pub fn glob_to_regex(glob: &str) -> String {
    let mut regex = String::from("^");
    let has_wildcard = glob.contains('*') || glob.contains('?');

    for c in glob.chars() {
        match c {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            '.' | '+' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '\\' => {
                regex.push('\\');
                regex.push(c);
            }
            _ => regex.push(c),
        }
    }

    if has_wildcard {
        regex.push('$');
    }
    regex
}

pub fn print_diagnostics(diagnostics: &[types::Diagnostic]) {
    for d in diagnostics {
        println!(
            "{}[{}] {}:{}:{}: {}",
            match d.severity {
                types::Severity::Error => "error",
                types::Severity::Warning => "warning",
            },
            d.code,
            d.path.display(),
            d.range.start_line,
            d.range.start_col,
            d.message
        );
    }
}
