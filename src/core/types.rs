use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SpecBlock {
    pub id: String,
    pub node_type: String, // Extracted from ID prefix (e.g. "UC")
    pub name: Option<String>,
    pub edges: Vec<EdgeUse>,
    pub file_path: PathBuf,
    pub line_start: usize, // 1-based
    pub line_end: usize,   // 1-based
    pub content: String,   // Raw markdown content
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EdgeUse {
    pub id: String,
    pub name: Option<String>,
    pub line: usize,
    pub col_start: usize,
    pub col_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RefUse {
    pub target_id: String,
    pub file_path: PathBuf,
    pub line: usize,
    pub col_start: usize,
    pub col_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub path: PathBuf,
    pub range: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}
