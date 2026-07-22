use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Persistent configuration for the resident remapper. Mirrors the original
/// .NET design but serializes as pretty JSON under the user's config dir.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub version: u32,
    pub device: DeviceFilter,
    pub action: ActionConfig,
    pub settings: Settings,
    /// Persisted UI language. Empty string means "use system default".
    /// Values: "zh" / "en".
    pub language: String,
}

/// Which physical Dock collection carries the Teams key. Hex strings (no 0x
/// prefix) so the file stays human-readable. Defaults match the observed real
/// Dock (VID 045E / PID 084D, UsagePage FF99 / Usage 0001).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DeviceFilter {
    pub vendor_id: String,
    pub product_id: String,
    pub usage_page: String,
    pub usage: String,
}

/// The action fired on a Teams-key press. `app_target` is the Shell parsing
/// name of an item registered in `shell:AppsFolder`; `app_name` is retained for
/// display and recovery if Windows changes that parsing name. The other fields
/// are kept so version-1 configs can still execute until the user saves a
/// registered application selection.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ActionConfig {
    pub app_name: String,
    pub app_target: String,
    pub preset_id: String,
    pub command: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub start_with_windows: bool,
    pub minimize_to_tray: bool,
    pub play_confirmation_beep: bool,
    pub enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: 2,
            device: DeviceFilter {
                vendor_id: "045E".into(),
                product_id: "084D".into(),
                usage_page: "FF99".into(),
                usage: "0001".into(),
            },
            action: ActionConfig {
                app_name: String::new(),
                app_target: String::new(),
                preset_id: "teams".into(),
                command: String::new(),
                arguments: String::new(),
            },
            settings: Settings {
                start_with_windows: false,
                minimize_to_tray: false,
                play_confirmation_beep: true,
                enabled: true,
            },
            language: String::new(),
        }
    }
}

impl Config {
    pub fn path() -> PathBuf {
        let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push("ms-audio-dock-remapper");
        dir.push("config.json");
        dir
    }

    pub fn load() -> Config {
        let path = Config::path();
        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Ok(parsed) = serde_json::from_str::<Config>(&text) {
                return parsed;
            }
        }
        Config::default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Config::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(&path, text)
    }
}
