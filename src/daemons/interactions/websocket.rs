use crate::daemons::screenlock::send_message;
use serde::Deserialize;
use tokio::task;
use ws::{listen, Message};

#[derive(Debug, Deserialize)]
struct MsgLegal {
    pub op: u8,
    pub image: Option<String>,
    pub grab_input: Option<bool>
}

pub fn start_websocket(bind_addr: String) {
    listen(bind_addr, |out| {
        move |msg: Message| {
            task::spawn(async move {
                let result: Result<MsgLegal, serde_json::Error> = serde_json::from_str(msg.as_text().unwrap());
                if let Ok(result) = result {
                    send_message(result.op, result.image, result.grab_input).await;
                } 
            });

            out.send(Message::Text("massa".to_string()))
        }
    })
    .unwrap();
}
