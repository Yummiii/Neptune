use crate::gui_manager;

pub async fn btn_released(time_pressed: i64) {
    if time_pressed <= 3000 {
        gui_manager::block_screen().await;
    }

    if time_pressed > 3000 {
        gui_manager::kill_screen().await;
    }
}
