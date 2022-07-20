use figment::{Figment, providers::{Toml, Format}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScreenLockConfigs {
    pub enabled: bool,
    pub images: Option<Vec<String>>,
    pub grab_input: Option<bool>
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
    pub screenlock: Option<ScreenLockConfigs>
}

impl Configs {
    pub fn get(config_file: String) -> Self {
        Figment::new()
            .merge(Toml::file(config_file))
            .extract()
            .expect("Error loading config file")
    }
}