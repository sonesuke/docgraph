use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpecBlock {
    pub id: String,
    pub kind: Option<String>,
    pub edges: Vec<EdgeUse>,
    pub refs: Vec<RefUse>,
    pub file_path: PathBuf,
    pub line_start: usize, // 1-based
    pub line_end: usize,   // 1-based
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EdgeUse {
    pub edge_type: String, // verifies, depends_on, etc.
    pub target_id: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefUse {
    pub target_id: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub path: PathBuf,
    pub range: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
