use sysinfo::{System, SystemExt};
use tokio::sync::Mutex;
use crate::device_helpers;

lazy_static::lazy_static!{
    static ref PROCESS_LIST: Mutex<Vec<u32>> = Mutex::new(Vec::new());
}

pub async fn block_screen() {
    if System::new_all().processes_by_exact_name("nepnep").count() <= 0 {         
        PROCESS_LIST.lock().await.push(run_script::spawn_script!("/home/yummi/Taiga/CodigosFodas/Neptune/telas_legais/build/src/./nepnep").unwrap().id());
        PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_keyboard_num())).unwrap().id());
        PROCESS_LIST.lock().await.push(run_script::spawn_script!(format!("evtest --grab /dev/input/event{} > /dev/null", device_helpers::get_mouse_num())).unwrap().id());
    } 
}

pub async fn kill_screen() {
    println!("{:?}", PROCESS_LIST.lock().await);
    PROCESS_LIST.lock().await.iter().for_each(|id| {
        run_script::spawn_script!(format!("kill -9 $(pstree -p {} | grep -o '[0-9]*')", id)).unwrap();
    });
    PROCESS_LIST.lock().await.clear();
}