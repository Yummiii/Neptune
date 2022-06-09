use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]

pub struct ScreenshotConfigs {
    pub enabled: bool,
    pub screenshots_path: String,
    pub screenshots_redirect_path: String
}

#[derive(Deserialize, Debug)]
pub struct ScreenLockConfigs {
    pub enabled: bool,
    pub serial_port: Option<String>,
    pub screenlock_img: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Configs {
    pub screenshot_configs: Option<ScreenshotConfigs>,
    pub screenlock_configs: Option<ScreenLockConfigs>
}

impl Configs {
    pub fn get() -> Self {
        Figment::new()
            .merge(Toml::file("neptune.toml"))
            .extract()
            .expect("Erro ao abrir as configurações")
    }
}
