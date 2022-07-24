use crate::daemons::{configs::ScreenLockProfileConfigs, profile_extensions::get_image};
use std::{process::Child, str::FromStr, env::current_exe};
use command_macros::command;
use evdev::{Device, Key};
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<Child>> = Mutex::new(Vec::new());
    static ref PROFILES: Mutex<Vec<ScreenLockProfileConfigs>> = Mutex::new(Vec::new());
}

pub async fn add_profile(profile: ScreenLockProfileConfigs) {
    PROFILES.lock().await.push(profile);
}

pub async fn block_screen() {
    let mut procs = PROCESS_LIST.lock().await;

    //foi o jeito q deu pra fazer
    let mut device = Device::open("/dev/input/event2").unwrap();
    println!("{:?}", device.grab());

    let profiles = PROFILES.lock().await;
    let profiles: Vec<&ScreenLockProfileConfigs> = profiles.iter().filter(|x| {
        let keys: Vec<Key> = x.keys.as_ref().unwrap().iter().map(|key| Key::from_str(key).unwrap()).collect();
        keys == device.get_key_state().unwrap().iter().collect::<Vec<Key>>()
    }).collect();
    let profile = profiles.first().unwrap();
    
    let img = get_image(profile);
    let mut gui = command!((current_exe().unwrap()) gui 
        -i (if img.is_some() { img.unwrap() } else { "".to_string() }) 
        -w
        //(if !grab_input { "-s" } else { "-n" })
        //(if windowed.unwrap_or(*WINDOWED.lock().await) { "-w" } else { "-n" })
    );     

    procs.push(gui.spawn().unwrap());
}

pub async fn kill_screen_block() {
    info!("Kill screen block");
    let mut procs = PROCESS_LIST.lock().await;
    procs.iter_mut().for_each(|proc| proc.kill().unwrap());
    procs.clear();
}