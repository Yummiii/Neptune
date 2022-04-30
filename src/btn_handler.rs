use std::sync::Arc;

use ms_converter::get_max_possible_duration_long;
use tokio::sync::mpsc::Sender;

pub async fn btn_released(time_pressed: i64, tx: Arc<Sender<i32>>) {
    println!("Solto: {} =-= {}", time_pressed, get_max_possible_duration_long(time_pressed).unwrap());

    if time_pressed <= 3000 {
        tx.send(1).await.unwrap();
    }

    if time_pressed > 3000 {
        tx.send(2).await.unwrap();
    }
}
