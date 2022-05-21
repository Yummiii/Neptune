use std::{
    ffi::CString,
    os::raw::c_char,
};
use tokio::{sync::Mutex, task};

lazy_static::lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<u32>> = Mutex::new(Vec::new());
}

extern "C" {
    fn top_nep(path: *const c_char);
    fn down_nep();
}

pub async fn block_screen() {
    if PROCESS_LIST.lock().await.len() == 0 {
        unsafe {
            task::spawn(async {
                let a = CString::new("").unwrap();
                top_nep(a.as_ptr());
            });
            task::yield_now().await;
        }
        PROCESS_LIST.lock().await.push(1);
    }
    //PROCESS_LIST.lock().await.push(run_script::spawn_script!("/home/yummi/Taiga/CodigosFodas/Neptune/telas_legais/build/src/./nepnep").unwrap().id());
    //PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_keyboard_num())).unwrap().id());
    //PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_mouse_num())).unwrap().id());
}

pub async fn kill_screen() {
    //println!("{:?}", PROCESS_LIST.lock().await);
    unsafe {
        down_nep();
    }
    // PROCESS_LIST.lock().await.iter().for_each(|id| {
    //     run_script::spawn_script!(format!("kill -9 $(pstree -p {} | grep -o '[0-9]*')", id))
    //         .unwrap();
    // });
    PROCESS_LIST.lock().await.clear();
}
