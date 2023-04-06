use crate::daemon::configs::Configs;
use std::thread;
use tokio::task;

mod configs;
mod screenshots;

pub async fn start_daemon(config_file: Option<String>) {
    let configs = Configs::get(
        config_file.unwrap_or(format!("{}/.config/neptune/config.toml", env!("HOME"))),
    );
    trace!("{:?}", configs);

    if let Some(screenshots) = configs.screenshots {
        task::spawn(async move {
            if let Some(watch_dir) = screenshots.watch_dir {
                if let Some(target_dir) = screenshots.target_dir {
                    screenshots::start(
                        watch_dir,
                        target_dir,
                        screenshots.initial_check.unwrap_or(false),
                    );
                }
            }
        });
    }

    thread::park();
}
