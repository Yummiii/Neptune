use serde::Deserialize;
use tokio::task;
use ws::{listen, Message};

use crate::daemons::screenlock;

#[derive(Debug, Deserialize)]
struct MsgLegal {
    pub op: u8,
    pub image: Option<String>,
    pub grab_input: Option<bool>,
    pub windowed: Option<bool>
}

pub fn start_websocket(bind_addr: String) {
    listen(bind_addr, |out| {
        move |msg: Message| {
            let result: Result<MsgLegal, serde_json::Error> = serde_json::from_str(msg.as_text().unwrap());
            if let Ok(result) = result {
                task::spawn(async move {
                    if result.op == 0 {
                        screenlock::block_screen(result.image, result.grab_input, result.windowed).await;
                    } else if result.op == 1 {
                        screenlock::kill_screen_block().await;
                    }                
                });            
                out.send(Message::Text("massa".to_string()))
            } else {
                out.send(Message::Text("não massa".to_string()))
            }
        }
    })
    .unwrap();
}
