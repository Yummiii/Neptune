use chrono::Utc;
use std::time::Duration;
use crate::daemon::screenlock::btn_handler;

pub async fn iniciar_serial(serial_port: &String) {
    info!("Reading from serial port: {}", serial_port);
    let mut port = serialport::new(serial_port, 9600)
        .timeout(Duration::from_secs(604800))
        .open()
        .expect("Serial port not found");

    let mut serial_buf = [0; 1];
    let mut apertado = 0;
    while let Ok(()) = port.read_exact(&mut serial_buf) {
        trace!("Serial data: {:?}", &serial_buf);
        trace!("Time: {}", apertado);

        if serial_buf[0] == 1 && apertado != 0 {
            btn_handler::btn_released(Utc::now().timestamp_millis() - apertado).await;
        } else {
            apertado = Utc::now().timestamp_millis();
        }
    }
}
