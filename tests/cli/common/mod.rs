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

/// Create a docgraph.toml config file
pub fn create_config(dir: &Path, content: &str) -> std::path::PathBuf {
    let path = dir.join("docgraph.toml");
    fs::write(&path, content).expect("Failed to write config");
    path
}

/// Default docgraph.toml with basic node types
pub fn default_config() -> &'static str {
    r#"
[nodes.TEST]
desc = "Test node"
[nodes.REQ]
desc = "Requirement"
[nodes.ADR]
desc = "Architecture Decision Record"
"#
}
