use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use sha256::try_digest;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub site: Site,
    #[serde(default)]
    pub build: Build,
    #[serde(default)]
    pub serve: Serve,
    #[serde(default)]
    pub extra: HashMap<String, toml::Value>,
    #[serde(default)]
    #[serde(rename = "plugin")]
    pub plugins: Vec<Plugin>,
    #[serde(default = "default_config_file")]
    pub path: String,
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
    #[serde(default = "default_plugin_dir")]
    pub plugin_dir: String,
    #[serde(default)]
    pub include_drafts: bool,
    #[serde(default = "default_static_prefix")]
    pub static_prefix: String,
    #[serde(default)]
    pub no_verify: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Serve {
    #[serde(default = "default_serve_host")]
    pub host: String,
    #[serde(default = "default_serve_port")]
    pub port: u16,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HookType {
    Pre,
    Post,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    #[serde(rename = "hook")]
    pub hook_type: HookType,
    pub name: String,
    pub hash: String,
    #[serde(flatten)]
    pub args: HashMap<String, toml::Value>,
}

impl Plugin {
    pub fn get_hash(&self, plugin_dir: &Path) -> Result<String> {
        let mut plugin_path = PathBuf::from(plugin_dir);
        plugin_path.push(&self.name);

        let digest = try_digest(plugin_path)?;

        Ok(digest.to_string())
    }

    pub fn get_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        for arg in &self.args {
            args.push(format!("{}={}", arg.0, arg.1));
        }

        args
    }

    pub fn resolve(&self, plugin_dir: &Path, no_verify: bool) -> Result<PathBuf> {
        let mut plugin_path = PathBuf::from(plugin_dir);
        plugin_path.push(&self.name);

        let plugin_path = plugin_path.canonicalize()?;

        if !plugin_path.is_file() {
            return Err(anyhow!("plugin '{}' could not be located.", self.name));
        }

        if no_verify {
            return Ok(plugin_path);
        }

        let digest = self.get_hash(plugin_dir)?;

        if digest != self.hash {
            return Err(anyhow!(
                "hash {} for plugin '{}' does not match",
                self.hash,
                self.name
            ));
        }

        Ok(plugin_path)
    }

    pub fn run(&self, config: &Config, root_dir: &Path, no_verify: bool) -> Result<()> {
        println!("Running plugin {}...", &self.name);

        let plugin_file = self.resolve(Path::new(&config.build.plugin_dir), no_verify)?;
        let args = self.get_args();

        let status = std::process::Command::new(&plugin_file)
            .args(args)
            .current_dir(root_dir)
            .status()?;
        let code = status
            .code()
            .ok_or(anyhow!("plugin returned no status code"))?;

        if code != 0 {
            return Err(anyhow!(
                "error executing {:#?}, error {}",
                plugin_file,
                code
            ));
        }

        Ok(())
    }
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
            plugin_dir: default_plugin_dir(),
            include_drafts: false,
            static_prefix: default_static_prefix(),
            no_verify: false,
        }
    }
}

impl Default for Serve {
    fn default() -> Self {
        Self {
            host: default_serve_host(),
            port: default_serve_port(),
        }
    }
}

fn default_config_file() -> String {
    "tars.toml".to_string()
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
fn default_plugin_dir() -> String {
    "plugin".to_string()
}
fn default_static_prefix() -> String {
    "static".to_string()
}
fn default_serve_host() -> String {
    "127.0.0.1".to_string()
}
fn default_serve_port() -> u16 {
    8080u16
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let mut cfg: Self = toml::from_str(&data)?;
        cfg.path = path.to_string();

        Ok(cfg)
    }
}
