use evdev::{MiscType, Device, Key};
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref MOUSE: Mutex<Option<Device>> = Mutex::new(None);
    pub static ref KB: Mutex<Option<Device>> = Mutex::new(None);
}


pub async fn get_devices() {
    for device in evdev::enumerate() {
        if let Some(props) = device.1.misc_properties() {            
            if props.contains(MiscType::MSC_SCAN) && device.1.supported_absolute_axes().is_none() {
                if device.1.supported_relative_axes().is_some() {
                    *MOUSE.lock().await = Some(device.1);
                } else {
                    *KB.lock().await = Some(device.1);
                }
            }
        }
    }
}

pub async fn are_keys_pressed(keys: Vec<Key>) -> bool {
    let kb = KB.lock().await;
    let kb = kb.as_ref().unwrap();
    kb.get_key_state().unwrap().iter().collect::<Vec<Key>>() == keys
}