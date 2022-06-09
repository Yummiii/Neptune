use tokio::sync::Mutex;
use crate::device_helpers;

lazy_static::lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static ref BLOCK_IMG: Mutex<String> = Mutex::new("".to_string());
    static ref DISABLE_INPUT: Mutex<bool> = Mutex::new(true);
}

pub async fn set_img(img: &String) {
    println!("Block img: {}", img);
    *BLOCK_IMG.lock().await = img.to_string();
}

pub async fn set_input_disabled(disabled: bool) {
    *DISABLE_INPUT.lock().await = disabled;
}

pub async fn block_screen() {
    if PROCESS_LIST.lock().await.len() == 0 {
        let disabled = *DISABLE_INPUT.lock().await;
        //isso ta uma gambiarra fudida, mas por enquanto ta funcionando
        PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("neptune_gui_block_manager {} {}", BLOCK_IMG.lock().await, if !disabled { "-s" } else { "" } )).unwrap().id());

        if disabled {
            PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_keyboard_num())).unwrap().id());
            PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_mouse_num())).unwrap().id());
        }
    }
}

pub async fn kill_screen() {
    PROCESS_LIST.lock().await.iter().for_each(|id| {
        run_script::spawn_script!(format!("kill -9 $(pstree -p {id} | grep -o '[0-9]*')")).unwrap();
    });
    PROCESS_LIST.lock().await.clear();
}
