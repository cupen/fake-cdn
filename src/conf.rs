use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub dir: Option<String>,
    pub token: Option<String>,
}

pub fn load_from_file(path: &Path) -> Config {
    if !path.exists() {
        return Config::default();
    }
    let content = fs::read_to_string(path).unwrap_or_default();
    toml::from_str(&content).unwrap_or_default()
}
