use std::{env::current_exe, process::{Child, Stdio}};
use command_macros::command;
use rand::prelude::SliceRandom;
use run_script::ScriptOptions;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<Child>> = Mutex::new(Vec::new());
    static ref GRAB_INPUT: Mutex<bool> = Mutex::new(false);
    static ref WINDOWED: Mutex<bool> = Mutex::new(false);
    static ref IMGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub async fn init(grab_input: bool, windowed: bool) {
    *GRAB_INPUT.lock().await = grab_input;
    *WINDOWED.lock().await = windowed;
}

pub async fn add_img(img: String) {
    info!("Added image: [{}]", img);
    IMGS.lock().await.push(img);
}

pub async fn block_screen(image: Option<String>, grab_input: Option<bool>, windowed: Option<bool>) {
    let mut procs = PROCESS_LIST.lock().await;
    if procs.len() == 0 {
        info!("Screen block start");

        let grab_input = grab_input.unwrap_or(*GRAB_INPUT.lock().await);
        let mut img = image.clone();
        if image.is_none() && IMGS.lock().await.len() >= 1 {
            let img_list = IMGS.lock().await;
            img = Some(img_list.choose(&mut rand::thread_rng()).unwrap().to_string());
        }
    
        let mut gui = command!((current_exe().unwrap()) gui 
            -i (if img.is_some() { img.unwrap() } else { "".to_string() }) 
            (if !grab_input { "-s" } else { "-n" })
            (if windowed.unwrap_or(*WINDOWED.lock().await) { "-w" } else { "-n" })
        );     
        
        if grab_input {
            let mut keyboard = command!(evtest --grab /dev/input/event(get_keyboard_num()));
            keyboard.stdout(Stdio::null());
            procs.push(keyboard.spawn().unwrap());

            let mut mouse = command!(evtest --grab /dev/input/event(get_mouse_num()));
            mouse.stdout(Stdio::null());
            procs.push(mouse.spawn().unwrap());
        }

        info!("{:?}", gui);
        procs.push(gui.spawn().unwrap());
    }
}

pub async fn kill_screen_block() {
    info!("Kill screen block");
    let mut procs = PROCESS_LIST.lock().await;
    procs.iter_mut().for_each(|proc| proc.kill().unwrap());
    procs.clear();
}


//por algum motivo isso sempre funciona
fn get_keyboard_num() -> String {
    let (code, output, _) = run_script::run_script!(r#"grep -E 'Handlers|EV=' /proc/bus/input/devices | grep -B1 'EV=120013' | grep -Eo 'event[0-9]+' | grep -Eo '[0-9]+' | tr -d '\n'"#, &vec![], &ScriptOptions::new()).unwrap();
    if code == 0 {
        output
    } else {
        panic!("não achei o numero do teclado")
    }
}

fn get_mouse_num() -> String {
    let (code, output, _) = run_script::run_script!(r#"grep -E 'Handlers|EV=' /proc/bus/input/devices | grep -B1 'EV=17' | grep -Eo 'event[0-9]+' | grep -Eo '[0-9]+' | tr -d '\n'"#, &vec![], &ScriptOptions::new()).unwrap();
    if code == 0 {
        output
    } else {
        panic!("não achei o numero do mouse")
    }
}