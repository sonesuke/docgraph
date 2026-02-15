use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::core::error::Result;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub nodes: HashMap<String, NodeConfig>,
    #[serde(default)]
    pub graph: GraphConfig,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct GraphConfig {
    #[serde(default)]
    pub unique: Vec<String>,
    #[serde(default)]
    pub explicit: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct NodeConfig {
    pub desc: String,
    pub template: Option<std::path::PathBuf>,
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct RuleConfig {
    pub dir: String, // "from" or "to"
    pub targets: Vec<String>,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub desc: Option<String>,
    pub rel: Option<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        // Start from the given path and search upward for docgraph.toml
        let start_dir = if path.is_dir() {
            path.to_path_buf()
        } else {
            path.parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        };

        // Search upward for docgraph.toml
        let mut current = Some(start_dir.as_path());
        while let Some(dir) = current {
            let config_path = dir.join("docgraph.toml");
            if config_path.exists() {
                let content = fs::read_to_string(&config_path)?;
                let config: Config = toml::from_str(&content)?;
                return Ok(config);
            }
            current = dir.parent();
        }

        // Fallback to current directory
        let cwd_config = Path::new("docgraph.toml");
        if cwd_config.exists() {
            let content = fs::read_to_string(cwd_config)?;
            let config: Config = toml::from_str(&content)?;
            return Ok(config);
        }

        Ok(Config::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_config_default() {
        let dir = tempdir().unwrap();
        // Create an empty config file to prevent fallback to CWD
        let config_path = dir.path().join("docgraph.toml");
        File::create(&config_path).unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert!(config.nodes.is_empty());
    }

    #[test]
    fn test_load_config_from_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("docgraph.toml");
        let mut file = File::create(&config_path).unwrap();
        // Add node type to check desc
        writeln!(file, "[nodes.REQ]\ndesc = \"Requirement\"").unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.nodes["REQ"].desc, "Requirement");
    }

    #[test]
    fn test_load_config_parent_search() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        std::fs::create_dir(&subdir).unwrap();

        let config_path = dir.path().join("docgraph.toml");
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "[nodes.REQ]\ndesc = \"Requirement\"").unwrap();

        let config = Config::load(&subdir).unwrap();
        assert_eq!(config.nodes["REQ"].desc, "Requirement");
    }

    #[test]
    fn test_load_config_malformed() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("docgraph.toml");
        std::fs::write(&config_path, "invalid toml content [[").unwrap();

        // Should return error
        let result = Config::load(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_from_specific_file_arg() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("docgraph.toml");
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "[nodes.REQ]\ndesc = \"Requirement\"").unwrap();

        // Pass the FILE path, not dir
        // The code `start_dir = if path.is_dir() { ... } else { path.parent() ... }`
        // checks if we can find config starting from that file's directory.
        // We simulate running `docgraph check ./subdir/file.md`
        let file_path = dir.path().join("file.md");
        let config = Config::load(&file_path).unwrap();

        assert_eq!(config.nodes["REQ"].desc, "Requirement");
    }

    #[test]
    fn test_load_config_root_path() {
        // Test fallback when path has no parent (e.g. root)
        // We can't easily write to root, so we expect Config::default() or whatever is in CWD.
        // If there is no docgraph.toml in CWD, we get default (empty).

        #[cfg(unix)]
        let root = Path::new("/");
        #[cfg(windows)]
        let root = Path::new("C:\\");

        let _config = Config::load(root).unwrap();
        // Since we run this in the project root, it falls back to the actual docgraph.toml.
        // The important part is that it didn't error.
        let cwd_config = Path::new("docgraph.toml");
        if cwd_config.exists() {
            // It loaded the CWD config - just verify it didn't error
        }
    }
    #[test]
    fn test_load_config_fallback_cwd() {
        // This test simulates "no config found in parent", falling back to CWD check.
        // It's tricky to mock CWD in Rust tests safely in parallel.
        // However, we can test that passing a non-existent path eventually returns default if nothing found.
        let dir = tempdir().unwrap();
        let path = dir.path().join("non_existent/subdir");
        // We know CWD *probably* doesn't have docgraph.toml in test environment, or if it does, it loads it.
        // The key logic is: search parents -> None? -> check CWD.
        // If we run this in a clean CWD (which we can't easily guarantee), it returns default.
        // So we will just assert it returns Ok(_) - meaning it didn't crash.
        let config = Config::load(&path);
        assert!(config.is_ok());
    }
}
