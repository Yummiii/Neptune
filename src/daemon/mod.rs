use self::configs::Configs;
use crate::daemon::screenlock::{block_manager, serial};
use futures::future;
use tokio::task;
use walkdir::WalkDir;

mod configs;
mod screenlock;
mod screenshots;

pub async fn start(config_file: Option<String>) {
    let configs = Configs::get(config_file.unwrap_or(format!("{}/.config/neptune/config.toml", env!("HOME"))),);
    trace!("Loaded configs: {:?}", configs);

    if let Some(cfgs) = configs.screenlock_configs {
        if cfgs.enabled {
            info!("Screenlock enabled");
            if let Some(serial) = cfgs.serial_port {
                task::spawn(async move {
                    serial::iniciar_serial(&serial).await;
                });
            }
            task::spawn(async move {
                if let Some(imgs) = &cfgs.screenlock_imgs {
                    for img in imgs {
                        block_manager::set_img(img).await;
                    }
                }

                if let Some(dir) = &cfgs.screenlock_imgs_dir {
                    for file in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
                        block_manager::set_img(&file.path().display().to_string()).await;
                    }
                }

                block_manager::set_grab_input(cfgs.grab_input.unwrap_or(false)).await;
            });
        }
    }

    if let Some(cfgs) = configs.screenshot_configs {
        if cfgs.enabled && cfgs.screenshots_watch_dir.is_some() {
            info!("Screenshots redirector enabled");
            task::spawn(async move {
                screenshots::iniciar(cfgs).await;
            });
        }
    }

    future::pending::<()>().await;
    unreachable!("Daemon stopped")
}
