use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]

pub struct ScreenshotConfigs {
    pub initial_check: Option<bool>,
    pub watch_dir: Option<String>,
    pub target_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScreenLockProfileConfigs {
    pub keys: Option<Vec<String>>,
    pub images: Option<Vec<String>>,
    pub block_input: Option<bool>,
    pub windowed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GrpcConfigs {
    pub bind_addr: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InteractionsConfigs {
    pub grpc: Option<GrpcConfigs>,
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
