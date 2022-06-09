mod btn_handler;
mod configs;
mod device_helpers;
mod block_manager;
mod prints_redirector;
mod serial;
use std::thread;
use configs::Configs;
use tokio::task;

#[tokio::main]
async fn main() {
    let configs = Configs::get();

    if let Some(screenlock_configs) = configs.screenlock_configs {
        if screenlock_configs.enabled {
            if let Some(serial_port) = screenlock_configs.serial_port {
                task::spawn(async move {
                    serial::iniciar_serial(&serial_port).await;
                });
            }

            if let Some(block_img) = screenlock_configs.screenlock_img {
                block_manager::set_img(&block_img).await;
            }
        }
    }

    if let Some(screenshot_configs) = configs.screenshot_configs {
        if screenshot_configs.enabled && screenshot_configs.screenshots_watch_dir.is_some() {
            task::spawn(async move {
                prints_redirector::iniciar(screenshot_configs).await;
            });
        }
    }

    thread::park();
}
