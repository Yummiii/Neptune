use self::daemons_config::DaemonsConfig;

mod daemons_config;

pub fn start_daemons(config_file: String) {
    let cfgs = DaemonsConfig::get(config_file);


}