use serde::Deserialize;
use std::env;
use std::path::PathBuf;
use std::fs::{create_dir_all, read_to_string};

/// Config struct
#[derive(Deserialize, Debug, Default)]
pub struct RicatConfig {
    pub number_feature: bool,
    pub dollar_sign_feature: bool,
    pub tabs_feature: bool,
    pub compress_empty_line_feature: bool,
    pub pagination_feature: bool,
}

/// Loading the config from $HOME/.config/ricat/ricat_cfg.toml
pub fn load_config() -> RicatConfig {
    let config_dir = env::var("RICAT_CONFIG_DIR").unwrap_or_else(|_| {
        let home_dir = dirs::home_dir().expect("Failed to find home directory");
        let default_config_dir = home_dir.join(".config/ricat");
        create_dir_all(&default_config_dir).expect("Failed to create config directory");
        default_config_dir.to_str().unwrap().to_string()
    });

    let config_file = PathBuf::from(config_dir).join("ricat_cfg.toml");

    if config_file.exists() {
        if let Ok(config_content) = read_to_string(config_file) {
            if let Ok(config) = toml::from_str(&config_content) {
                return config;
            }
        }
    }

    RicatConfig::default()
}
