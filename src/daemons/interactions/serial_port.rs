use crate::daemons::screenlock::send_message;
use chrono::Utc;
use std::time::Duration;

pub async fn start_serial(path: String, rate: u32) {
    info!("Reading from serial port in: {}", path);
    let mut port = serialport::new(path, rate)
        .timeout(Duration::from_secs(604800))
        .open()
        .expect("Serial port not found");

    let mut serial_buf = [0; 1];
    let mut apertado = 0;
    while let Ok(()) = port.read_exact(&mut serial_buf) {
        trace!("Serial data: {:?}", &serial_buf);
        trace!("Time: {}", apertado);

        if serial_buf[0] == 1 && apertado != 0 {
            btn_released(Utc::now().timestamp_millis() - apertado).await;
        } else {
            apertado = Utc::now().timestamp_millis();
        }
    }
}

async fn btn_released(time_pressed: i64) {
    if time_pressed != 0 {
        debug!("Button pressed for {}ms", time_pressed);
        let mut cmd = 0;

        if time_pressed <= 1000 {
            cmd = 1;
        }
        if time_pressed > 1000 {
            cmd = 2;
        }

        send_message(cmd, None, None).await;
    }
}
