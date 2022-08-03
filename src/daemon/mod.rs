use crate::daemon::{configs::Configs, screenlock::ScreenlockProfile};
use std::thread;
use tokio::task;

mod configs;
mod screenlock;
mod interactions;

pub async fn start_daemon(config_file: Option<String>) {
    let configs = Configs::get(
        config_file.unwrap_or(format!("{}/.config/neptune/config.toml", env!("HOME"))),
    );
    trace!("{:?}", configs);

    if let Some(interactions) = configs.interactions {
        interactions::start_interactions(interactions).await;
    }

    if let Some(profiles_cfg) = configs.screenlock {
        task::spawn(async move {
            for profile in profiles_cfg {
                screenlock::add_profile(ScreenlockProfile::from_config(profile.0, profile.1)).await;
            }
        });
    }

    thread::park();
}
