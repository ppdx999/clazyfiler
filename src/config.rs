use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub keymaps: HashMap<String, String>,
    pub ui: UiConfig,
    pub external_commands: ExternalCommands,
    pub general: GeneralConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub panel_width_ratio: u8,
    pub show_borders: bool,
    pub show_hidden_files: bool,
    pub file_list_margin: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCommands {
    pub fuzzy_finder: String,
    pub editor: String,
    pub file_manager: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub default_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut keymaps = HashMap::new();
        // Navigation
        keymaps.insert("q".to_string(), "quit".to_string());
        keymaps.insert("k".to_string(), "up".to_string());
        keymaps.insert("Up".to_string(), "up".to_string());
        keymaps.insert("j".to_string(), "down".to_string());
        keymaps.insert("Down".to_string(), "down".to_string());
        keymaps.insert("h".to_string(), "back".to_string());
        keymaps.insert("Left".to_string(), "back".to_string());
        keymaps.insert("l".to_string(), "select".to_string());
        keymaps.insert("Right".to_string(), "select".to_string());
        keymaps.insert("Enter".to_string(), "select".to_string());
        keymaps.insert("Escape".to_string(), "back".to_string());
        keymaps.insert("r".to_string(), "refresh".to_string());
        keymaps.insert("F5".to_string(), "refresh".to_string());

        Config {
            keymaps,
            ui: UiConfig::default(),
            external_commands: ExternalCommands::default(),
            general: GeneralConfig::default(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            panel_width_ratio: 50,
            show_borders: true,
            show_hidden_files: false,
            file_list_margin: 1,
        }
    }
}

impl Default for ExternalCommands {
    fn default() -> Self {
        ExternalCommands {
            fuzzy_finder: "fzf".to_string(),
            editor: std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string()),
            file_manager: "xdg-open".to_string(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            default_directory: "~".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("clazyfiler");

        let config_file = config_dir.join("config.toml");

        if !config_file.exists() {
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_file)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("clazyfiler");

        fs::create_dir_all(&config_dir)?;

        let config_file = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_file, content)?;

        Ok(())
    }
}
