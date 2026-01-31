use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Create a temporary directory for testing
pub fn setup_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

/// Create a test document with given content
pub fn create_test_doc(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    fs::write(&path, content).expect("Failed to write test file");
    path
}

/// Create a valid document with anchor and heading
pub fn create_valid_doc(dir: &Path, id: &str, title: &str) -> std::path::PathBuf {
    let content = format!("<a id=\"{}\"></a>\n\n# {}\n", id, title);
    create_test_doc(dir, "test.md", &content)
}

/// Create a document with missing heading (DG001 violation)
pub fn create_doc_missing_heading(dir: &Path, id: &str) -> std::path::PathBuf {
    let content = format!("<a id=\"{}\"></a>\n", id);
    create_test_doc(dir, "test.md", &content)
}

/// Create a document with duplicate ID (DG002 violation)
pub fn create_docs_with_duplicate_id(
    dir: &Path,
    id: &str,
) -> (std::path::PathBuf, std::path::PathBuf) {
    let content1 = format!("<a id=\"{}\"></a>\n\n# First\n", id);
    let content2 = format!("<a id=\"{}\"></a>\n\n# Second\n", id);

    let path1 = create_test_doc(dir, "doc1.md", &content1);
    let path2 = create_test_doc(dir, "doc2.md", &content2);

    (path1, path2)
}

/// Create a docgraph.toml config file
pub fn create_config(dir: &Path, content: &str) -> std::path::PathBuf {
    let path = dir.join("docgraph.toml");
    fs::write(&path, content).expect("Failed to write config");
    path
}

/// Default docgraph.toml with basic node types
pub fn default_config() -> &'static str {
    r#"
[node_types]
TEST = { desc = "Test node" }
REQ = { desc = "Requirement" }
ADR = { desc = "Architecture Decision Record" }
"#
}
