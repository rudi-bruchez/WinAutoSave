use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Interval in seconds between auto-save keypresses
    pub interval_seconds: u64,

    /// List of application executable names to auto-save (e.g., "notepad.exe", "code.exe")
    /// If empty, auto-save works for all applications
    pub target_apps: Vec<String>,

    /// Hotkey combination for pause/resume (currently fixed to Ctrl+Shift+P)
    pub hotkey_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interval_seconds: 60,
            target_apps: vec![
                "notepad.exe".to_string(),
                "Code.exe".to_string(),
                "WINWORD.EXE".to_string(),
                "EXCEL.EXE".to_string(),
            ],
            hotkey_enabled: true,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&contents)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, json)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().ok_or("Failed to get exe directory")?;
        Ok(exe_dir.join("config.json"))
    }
}
