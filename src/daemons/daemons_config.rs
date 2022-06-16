use figment::{Figment, providers::{Toml, Format}};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DaemonsConfig {

}

impl DaemonsConfig {
    pub fn get(config_file: String) -> Self {
        Figment::new()
            .merge(Toml::file(config_file))
            .extract()
            .expect("Error parsing the daemons configs")
    }
}