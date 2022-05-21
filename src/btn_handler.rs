use std::time;

use crate::gui_manager;

pub async fn btn_released(time_pressed: i64) {
    //println!("Solto: {} =-= {}", time_pressed, get_max_possible_duration_long(time_pressed).unwrap());

    if time_pressed <= 3000 {
        gui_manager::block_screen().await;
    }

    if time_pressed > 3000 {
        gui_manager::kill_screen().await;
    }
}
