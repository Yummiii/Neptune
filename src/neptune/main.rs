mod btn_handler;
mod configs;
mod device_helpers;
mod block_manager;
mod prints_redirector;
mod serial;
use std::{thread, io::Write, fs::File};
use configs::Configs;
use tokio::task;
use whoami::username;

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

            if let Some(mut block_img) = screenlock_configs.screenlock_img {
                if block_img.starts_with("http://") || block_img.starts_with("https://") {
                    let res = reqwest::get(&block_img).await.unwrap().bytes().await.unwrap();
                    let file_name = format!("/home/{}/.config/neptune/block", username());
                    let mut file = File::create(&file_name).unwrap();
                    file.write_all(&res).unwrap();
                    block_img = file_name;

                }
                block_manager::set_img(&block_img).await;
            }

            if let Some(grab_input) = screenlock_configs.grab_input {
                block_manager::set_input_disabled(grab_input).await;
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
