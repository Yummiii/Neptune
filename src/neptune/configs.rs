use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
use whoami::username;

#[derive(Deserialize, Debug)]

pub struct ScreenshotConfigs {
    pub enabled: bool,
    pub initial_check: Option<bool>,
    pub screenshots_watch_dir: Option<String>,
    pub screenshots_target_dir: Option<String>
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
        let configs_file = format!("/home/{}/.config/neptune/neptune.toml", username());
        Figment::new()
            .merge(Toml::file(configs_file))
            .extract()
            .expect("Erro ao abrir as configurações")
    }
}
