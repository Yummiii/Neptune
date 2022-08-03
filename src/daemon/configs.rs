use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]

pub struct ScreenshotConfigs {
    pub initial_check: Option<bool>,
    pub screenshots_watch_dir: Option<String>,
    pub screenshots_target_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScreenLockProfileConfigs {
    pub keys: Option<Vec<String>>,
    pub images: Option<Vec<String>>,
    pub block_input: Option<bool>,
    pub windowed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SerialPortConfigs {
    pub path: Option<String>,
    pub rate: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct WebsocketConfigs {
    pub bind_addr: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InteractionsConfigs {
    pub serial_port: Option<SerialPortConfigs>,
    pub websocket: Option<WebsocketConfigs>,
}

#[derive(Deserialize, Debug)]
pub struct Configs {
    pub interactions: Option<InteractionsConfigs>,
    pub screenlock: Option<HashMap<String, ScreenLockProfileConfigs>>,
    pub screenshots: Option<ScreenshotConfigs>,
}

impl Configs {
    pub fn get(config_file: String) -> Self {
        Figment::new()
            .merge(Toml::file(config_file))
            .extract()
            .expect("Error loading config file")
    }
}
