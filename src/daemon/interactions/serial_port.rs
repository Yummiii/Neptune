use std::time::Duration;
use crate::daemon::screenlock;

pub async fn start_serial(path: String, rate: u32) {
    info!("Reading from serial port in: {}", path);
    let mut port = serialport::new(path, rate)
        .timeout(Duration::from_secs(604800))
        .open()
        .expect("Serial port not found");

    let mut serial_buf = vec![0; 1];
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => println!("[({})] {:?}", t, serial_buf),
            Err(e) => eprintln!("{:?}", e)
        }
        //port.clear(buffer_to_clear)
    }
}