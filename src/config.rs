use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub keymaps: Keymaps,
    pub ui: UiConfig,
    pub external_commands: ExternalCommands,
    pub general: GeneralConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keymaps {
    pub quit: String,
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub select: String,
    pub back: String,
    pub refresh: String,
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
        Config {
            keymaps: Keymaps::default(),
            ui: UiConfig::default(),
            external_commands: ExternalCommands::default(),
            general: GeneralConfig::default(),
        }
    }
}

impl Default for Keymaps {
    fn default() -> Self {
        Keymaps {
            quit: "q".to_string(),
            up: "k".to_string(),
            down: "j".to_string(),
            left: "h".to_string(),
            right: "l".to_string(),
            select: "Enter".to_string(),
            back: "Escape".to_string(),
            refresh: "r".to_string(),
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

    pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("clazyfiler");
        Ok(config_dir)
    }

    pub fn key_matches(&self, key_char: char, action: &str) -> bool {
        let key_str = key_char.to_string();
        match action {
            "quit" => self.keymaps.quit == key_str,
            "up" => self.keymaps.up == key_str,
            "down" => self.keymaps.down == key_str,
            "left" => self.keymaps.left == key_str,
            "right" => self.keymaps.right == key_str,
            "refresh" => self.keymaps.refresh == key_str,
            _ => false,
        }
    }
}