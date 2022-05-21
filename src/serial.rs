use std::time::Duration;
use chrono::Utc;
use crate::btn_handler;

pub async fn iniciar_serial() {
    println!("Iniciando serial...");
    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_secs(604800))
        .open()
        .expect("Não foi encontrada a porta serial");


    let mut serial_buf = [0; 1];
    let mut apertado = 0;
    while let Ok(()) = port.read_exact(&mut serial_buf) {      
        if serial_buf[0] == 1 {
            btn_handler::btn_released(Utc::now().timestamp_millis() - apertado).await;
        } else {
            apertado = Utc::now().timestamp_millis();
        }
    }
}
