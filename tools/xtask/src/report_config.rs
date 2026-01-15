use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct ReportgenConfig {
    pub reports: Vec<ReportConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReportConfig {
    pub name: String,
    pub description: String,
    pub root: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub output: String,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub pre_command: Option<String>,
    #[serde(default)]
    pub ci_setup: Option<String>,
    #[serde(default)]
    pub ci_env: HashMap<String, String>,
}

impl ReportgenConfig {
    pub fn load(project_root: &Path) -> anyhow::Result<Self> {
        let config_path = project_root.join("reportgen.toml");
        let content = fs::read_to_string(&config_path)?;
        let config: ReportgenConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&ReportConfig> {
        self.reports.iter().find(|r| r.name == name)
    }

    /// Output report names as JSON for CI matrix generation
    pub fn to_json(&self) -> String {
        let names: Vec<&str> = self.reports.iter().map(|r| r.name.as_str()).collect();
        serde_json::to_string(&names).unwrap_or_else(|_| "[]".to_string())
    }
}
