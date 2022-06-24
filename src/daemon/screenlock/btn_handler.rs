use super::block_manager;


pub async fn btn_released(time_pressed: i64) {
    if time_pressed != 0 {
        debug!("Button pressed for {}ms", time_pressed);

        if time_pressed <= 1000 {
            block_manager::block_screen().await;
        }
        if time_pressed > 1000 {
            block_manager::kill_screen().await;
        }
    }
}