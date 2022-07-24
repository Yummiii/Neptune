use crate::daemons::{configs::ScreenLockProfileConfigs, profile_extensions::get_image};
use command_macros::command;
use evdev::{Device, Key};
use std::{env::current_exe, process::Child, str::FromStr};
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<Child>> = Mutex::new(Vec::new());
    static ref PROFILES: Mutex<Vec<ScreenLockProfileConfigs>> = Mutex::new(Vec::new());
    static ref KEYBOARD: Mutex<Option<Device>> = Mutex::new(None);
    static ref MOUSE: Mutex<Option<Device>> = Mutex::new(None);
}

pub async fn init() {
    *KEYBOARD.lock().await = Some(Device::open("/dev/input/event2").unwrap());
    *MOUSE.lock().await = Some(Device::open("/dev/input/event3").unwrap());
}

pub async fn add_profile(profile: ScreenLockProfileConfigs) {
    PROFILES.lock().await.push(profile);
}

pub async fn block_screen() {
    let mut procs = PROCESS_LIST.lock().await;

    let kb = KEYBOARD.lock().await;
    let kb = kb.as_ref().unwrap();

    //foi o jeito q deu pra fazer

    // task::spawn(async move {   
    //     keyboard.grab().unwrap();     
    //     mouse.grab().unwrap();
    //     loop {
    //         keyboard.fetch_events().unwrap();
    //         mouse.fetch_events().unwrap();
    //     }
    // });

    let profiles = PROFILES.lock().await;
    let profiles: Vec<&ScreenLockProfileConfigs> = profiles
        .iter()
        .filter(|x| {
            let keys: Vec<Key> = x.keys.as_ref().unwrap_or(&vec![]).iter().map(|key| Key::from_str(key).unwrap()).collect();
            keys == kb.get_key_state().unwrap().iter().collect::<Vec<Key>>()
        })
        .collect();

    let profile = *profiles.first().unwrap_or(&&ScreenLockProfileConfigs {
        enabled: true,
        keys: None,
        images: None,
        images_dirs: None,
        grab_input: Some(false),
        windowed: Some(false),
    });

    let img = get_image(profile);
    let mut gui = command!((current_exe().unwrap()) gui
        -i (if img.is_some() { img.unwrap() } else { "".to_string() })
        (if !profile.grab_input.unwrap() { "-s" } else { "-n" })
        (if profile.windowed.unwrap() { "-w" } else { "-n" })
    );

    procs.push(gui.spawn().unwrap());
}

pub async fn kill_screen_block() {
    info!("Kill screen block");
    let mut procs = PROCESS_LIST.lock().await;
    procs.iter_mut().for_each(|proc| proc.kill().unwrap());
    procs.clear();
}
