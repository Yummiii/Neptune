use super::configs::InteractionsConfigs;
use tokio::task;

mod serial_port;
mod websocket;

pub async fn start_interactions(interctions_configs: InteractionsConfigs) {
    info!("Starting interactions");

    if let Some(serial_port) = interctions_configs.serial_port {
        if serial_port.enabled && serial_port.path.is_some() {
            task::spawn(async move {
                serial_port::start_serial(
                    serial_port.path.unwrap(),
                    serial_port.rate.unwrap_or(9600),
                )
                .await;
            });
        }
    }

    if let Some(websocket) = interctions_configs.websocket {
        if websocket.enabled {
            task::spawn(async move {
                websocket::start_websocket(websocket.bind_addr.unwrap_or("127.0.0.1:0".to_string()));
            });
        }
    }
}