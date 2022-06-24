use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]

pub struct ScreenshotConfigs {
    pub enabled: bool,
    pub initial_check: Option<bool>,
    pub screenshots_watch_dir: Option<String>,
    pub screenshots_target_dir: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ScreenLockConfigs {
    pub enabled: bool,
    pub grab_input: Option<bool>,
    pub serial_port: Option<String>,
    pub screenlock_img: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Configs {
    pub screenshot_configs: Option<ScreenshotConfigs>,
    pub screenlock_configs: Option<ScreenLockConfigs>
}

impl Configs {
    pub fn get(config_file: String) -> Self {
        Figment::new()
            .merge(Toml::file(config_file))
            .extract()
            .expect("Error loading config file")
    }
}