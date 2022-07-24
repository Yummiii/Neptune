use self::configs::Configs;
use std::thread;
use tokio::task;

mod configs;
mod interactions;
mod screenlock;
mod screenshots;
mod profile_extensions;

pub async fn start_daemons(config_file: Option<String>) {
    info!("Starting daemon");
    let configs = Configs::get(
        config_file.unwrap_or(format!("{}/.config/neptune/config.toml", env!("HOME"))),
    );
    debug!("Daemon configs: {:?}", configs);

    if let Some(interactions) = configs.interactions {
        interactions::start_interactions(interactions).await;
    }

    if let Some(profiles) = configs.screenlock {
        for profile in profiles {
            info!("Loading profile: {}", profile.0);
            debug!("{:?}", profile.1);

            screenlock::add_profile(profile.1).await;
        }
    }

    if let Some(screenshots) = configs.screenshots {
        if screenshots.enabled && screenshots.screenshots_watch_dir.is_some() {
            info!("Screenshots redirector enabled");
            task::spawn(async move {
                screenshots::iniciar(screenshots).await;
            });
        }
    }

    thread::park();
}
