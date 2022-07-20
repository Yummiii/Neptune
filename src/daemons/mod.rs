use self::configs::Configs;
use std::thread;
use tokio::task;

mod configs;
mod interactions;
mod screenlock;

pub async fn start_daemons(config_file: Option<String>) {
    info!("Starting daemon");
    let configs = Configs::get(
        config_file.unwrap_or(format!("{}/.config/neptune/config.toml", env!("HOME"))),
    );
    debug!("Daemon configs: {:?}", configs);

    if let Some(interactions) = configs.interactions {
        interactions::start_interactions(interactions).await;
    }

    if let Some(screenlock) = configs.screenlock {
        if screenlock.enabled {
            task::spawn(async move {
                if let Some(imgs) = screenlock.images {
                    for img in imgs {
                        screenlock::add_img(img).await;
                    }
                }

                screenlock::init(screenlock.grab_input.unwrap_or(false)).await;
            });
        }
    }

    thread::park();
}
