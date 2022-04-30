use crate::btn_handler;
use chrono::Utc;
use std::{io::Read, sync::Arc, time::Duration};
use tokio::{sync::mpsc::Sender, task};

pub fn iniciar_serial(tx: Sender<i32>) {
    println!("Iniciando serial...");
    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_secs(604800))
        .open()
        .unwrap();
    let tx = Arc::new(tx);

    task::spawn(async move {
        let mut pressed_time = 0;
        loop {
            let mut serial_buf = [0; 1];

            let tx_tmp = Arc::clone(&tx);
            port.read(serial_buf.as_mut_slice())
                .expect("Found no data!");
            if serial_buf[0] == 0 {
                pressed_time = Utc::now().timestamp_millis();
            } else {
                task::spawn(async move {
                    btn_handler::btn_released(Utc::now().timestamp_millis() - pressed_time, tx_tmp).await;
                });
            }
        }
    });
}
