use rand::prelude::SliceRandom;
use tokio::sync::Mutex;

use super::device_helpers;

lazy_static::lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static ref BLOCK_IMG: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref GRAB_INPUT: Mutex<bool> = Mutex::new(false);
}

pub async fn set_img(img: &String) {
    if img != "" {
        debug!("Added block image: [{}]", img);
        BLOCK_IMG.lock().await.push(img.to_string());
    }
}

pub async fn set_grab_input(grab_input: bool) {
    if grab_input {
        debug!("Grab input enabled");
        *GRAB_INPUT.lock().await = grab_input;
    }
}

pub async fn block_screen() {
    if PROCESS_LIST.lock().await.len() == 0 {
        let grab_input = *GRAB_INPUT.lock().await;
        let img = BLOCK_IMG.lock().await;
        let cmd = format!("neptune gui -i \"{}\" {}", img.choose(&mut rand::thread_rng()).unwrap_or(&"".into()), if !grab_input { "-s" } else { "" });
        
        if grab_input {
            PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_keyboard_num())).unwrap().id());
            PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_mouse_num())).unwrap().id());
        }        

        debug!("Neptune GUI: [{}]", cmd);
        PROCESS_LIST.lock().await.push(run_script::spawn_script!(cmd).unwrap().id());
    }
}

pub async fn kill_screen() {
    PROCESS_LIST.lock().await.iter().for_each(|id| {
        run_script::spawn_script!(format!("kill -9 $(pstree -p {id} | grep -o '[0-9]*')")).unwrap();
    });
    PROCESS_LIST.lock().await.clear();
}