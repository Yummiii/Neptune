use figment::{Figment, providers::{Toml, Format}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct ScreenshotConfigs {
    pub enabled: bool,
    pub initial_check: Option<bool>,
    pub screenshots_watch_dir: Option<String>,
    pub screenshots_target_dir: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct ScreenLockConfigs {
    pub enabled: bool,
    pub images: Option<Vec<String>>,
    pub images_dirs: Option<Vec<String>>,
    pub grab_input: Option<bool>,
    pub windowed: Option<bool>
}

#[derive(Debug, Deserialize)]
pub struct SerialPortConfigs {
    pub enabled: bool,
    pub path: Option<String>,
    pub rate: Option<u32>
}

#[derive(Debug, Deserialize)]
pub struct WebsocketConfigs {
    pub enabled: bool,
    pub bind_addr: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct InteractionsConfigs {
    pub serial_port: Option<SerialPortConfigs>,
    pub websocket: Option<WebsocketConfigs>
}

#[derive(Deserialize, Debug)]
pub struct Configs {
    pub interactions: Option<InteractionsConfigs>,
    pub screenlock: Option<ScreenLockConfigs>,
    pub screenshots: Option<ScreenshotConfigs>
}

impl Configs {
    pub fn get(config_file: String) -> Self {
        Figment::new()
            .merge(Toml::file(config_file))
            .extract()
            .expect("Error loading config file")
    }
}