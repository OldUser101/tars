use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub site: Site,
    #[serde(default)]
    pub build: Build,
    #[serde(default)]
    pub extra: HashMap<String, toml::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub title: Option<String>,
    pub base_url: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    #[serde(default = "default_template_name")]
    pub default_template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    #[serde(default = "default_content_dir")]
    pub content_dir: String,
    #[serde(default = "default_template_dir")]
    pub template_dir: String,
    #[serde(default = "default_static_dir")]
    pub static_dir: String,
    #[serde(default = "default_build_dir")]
    pub build_dir: String,
    #[serde(default)]
    pub include_drafts: bool,
}

impl Default for Site {
    fn default() -> Self {
        Self {
            title: None,
            base_url: None,
            author: None,
            description: None,
            default_template: default_template_name(),
        }
    }
}

impl Default for Build {
    fn default() -> Self {
        Self {
            content_dir: default_content_dir(),
            template_dir: default_template_dir(),
            static_dir: default_static_dir(),
            build_dir: default_build_dir(),
            include_drafts: false,
        }
    }
}

fn default_template_name() -> String {
    "default.html".to_string()
}
fn default_content_dir() -> String {
    "content".to_string()
}
fn default_static_dir() -> String {
    "static".to_string()
}
fn default_template_dir() -> String {
    "template".to_string()
}
fn default_build_dir() -> String {
    "build".to_string()
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let cfg: Self = toml::from_str(&data)?;

        Ok(cfg)
    }
}
