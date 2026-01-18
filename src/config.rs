use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub node_types: HashMap<String, NodeType>,
    #[serde(default)]
    pub graph: GraphConfig,
    #[serde(default)]
    pub references: HashMap<String, ReferenceConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct NodeType {
    pub desc: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct GraphConfig {
    #[serde(default)]
    pub strict_node_types: bool,
    #[serde(default)]
    pub strict_relations: bool,
    #[serde(default)]
    pub doc_types: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReferenceConfig {
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RuleConfig {
    pub dir: String, // "from" or "to"
    pub targets: Vec<String>,
    pub min: Option<usize>,
    #[allow(dead_code)]
    pub max: Option<usize>,
}

impl Config {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
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
