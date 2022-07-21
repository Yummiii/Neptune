use std::{env::current_exe};
use command_macros::command;
use rand::prelude::SliceRandom;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static ref GRAB_INPUT: Mutex<bool> = Mutex::new(false);
    static ref ACTIVE: Mutex<bool> = Mutex::new(false);
    static ref IMGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub async fn init(grab_input: bool) {
    *GRAB_INPUT.lock().await = grab_input;
}

pub async fn add_img(img: String) {
    info!("Added image: [{}]", img);
    IMGS.lock().await.push(img);
}

pub async fn block_screen(image: Option<String>, grab_input: Option<bool>) {
    if !*ACTIVE.lock().await {
        info!("Screen block start");

        let mut img = image.clone();
        if image.is_none() && IMGS.lock().await.len() >= 1 {
            let img_list = IMGS.lock().await;
            img = Some(img_list.choose(&mut rand::thread_rng()).unwrap().to_string());
        }
    
        let mut gui = command!((current_exe().unwrap()) gui 
            (if img.is_some() { format!("-i {}", img.unwrap()) } else { "-n".to_string() }) 
            (if !grab_input.unwrap_or(*GRAB_INPUT.lock().await) { "-s" } else { "-n" })
        );     
        
        info!("{:?}", gui);
        PROCESS_LIST.lock().await.push(gui.spawn().unwrap().id());

        *ACTIVE.lock().await = true;
    }
}

pub async fn kill_screen_block() {
    info!("Kill screen block");
    PROCESS_LIST.lock().await.iter().for_each(|id| {
        run_script::spawn_script!(format!("kill $(pstree -p {id} | grep -o '[0-9]*')")).unwrap();
    });
    PROCESS_LIST.lock().await.clear();
    *ACTIVE.lock().await = false;
}
