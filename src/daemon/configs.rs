use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct ScreenshotConfigs {
    pub initial_check: Option<bool>,
    pub watch_dir: Option<String>,
    pub target_dir: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Configs {
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
