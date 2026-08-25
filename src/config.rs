use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub global: GlobalConfig,
    #[serde(default)]
    pub tree: TreeConfig,
    #[serde(default)]
    pub all: AllConfig,
    #[serde(default)]
    pub replace: ReplaceConfig,
    #[serde(default)]
    pub custom_commands: HashMap<String, CustomCommand>,
}

impl Default for Config {
    fn default() -> Self {
        let default_json = include_str!("../assets/config.json");
        serde_json::from_str(default_json).unwrap_or_else(|e| {
            eprintln!("[ERROR] Failed to parse embedded assets/config.json: {}", e);
            Self {
                global: GlobalConfig::default(),
                tree: TreeConfig {
                    default_output: "tree.txt".to_string(),
                },
                all: AllConfig {
                    default_output: "merged_code.txt".to_string(),
                },
                replace: ReplaceConfig::default(),
                custom_commands: HashMap::new(),
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GlobalConfig {
    #[serde(default)]
    pub ignore: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TreeConfig {
    #[serde(default)]
    pub default_output: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AllConfig {
    #[serde(default)]
    pub default_output: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplaceConfig {
    #[serde(default)]
    pub rules: HashMap<String, ReplaceRule>,
    #[serde(default)]
    pub rulesets: HashMap<String, ReplaceRuleSet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplaceRule {
    pub find: String,
    pub replace: String,
    #[serde(default = "default_true")]
    pub literal: bool,
}

impl ReplaceRule {
    pub fn new(find: String, replace: String, literal: bool) -> Self {
        Self { find, replace, literal }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReplaceRuleSet {
    pub rules: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CustomCommand {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub steps: Vec<String>,
}

pub fn get_asset_path(filename: &str) -> PathBuf {
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        exe_path.push(filename);
        return exe_path;
    }
    PathBuf::from(filename)
}

pub fn load_config() -> Config {
    let config_file = get_asset_path("config.json");

    if !config_file.exists() {
        return Config::default();
    }

    if let Ok(content) = fs::read_to_string(&config_file) {
        if let Ok(config) = serde_json::from_str::<Config>(&content) {
            return config;
        }
    }
    Config::default()
}

pub fn save_config(config: &Config) -> io::Result<()> {
    let config_file = get_asset_path("config.json");
    let content = serde_json::to_string_pretty(config)?;
    fs::write(config_file, content)?;
    Ok(())
}

pub fn fetch_online_default_config() -> Result<Config, Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/PerseusShade/Nexenal/main/assets/config.json";
    let client = reqwest::blocking::Client::builder()
        .user_agent("Nexenal-App")
        .build()?;
    let res = client.get(url).send()?;

    if res.status().is_success() {
        let online_config = res.json::<Config>()?;
        Ok(online_config)
    } else {
        Err(format!("HTTP Error {}", res.status()).into())
    }
}