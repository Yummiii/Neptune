use std::{ffi::CString, os::raw::c_char};
use tokio::{sync::Mutex, task};

//use crate::device_helpers;

lazy_static::lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static ref BLOCK_IMG: Mutex<CString> = Mutex::new(CString::new("").unwrap());
}

extern {
    fn top_nep(path: *const c_char);
    fn down_nep();
}

pub async fn set_img(img: &String) {
    println!("Block img: {}", img);
    let c_str = CString::new(img.as_str()).unwrap();
    *BLOCK_IMG.lock().await = c_str;
}

pub async fn block_screen() {
    if PROCESS_LIST.lock().await.len() == 0 {
        unsafe {
            task::spawn(async {
                top_nep(BLOCK_IMG.lock().await.as_ptr());
            });
            task::yield_now().await;
        }
        //PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_keyboard_num())).unwrap().id());
        //PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_mouse_num())).unwrap().id());
    }
}

pub async fn kill_screen() {
    unsafe {
        down_nep();
    }
    PROCESS_LIST.lock().await.iter().for_each(|id| {
        run_script::spawn_script!(format!("kill -9 $(pstree -p {} | grep -o '[0-9]*')", id)).unwrap();
    });
    PROCESS_LIST.lock().await.clear();
}
