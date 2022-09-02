use super::{configs::ScreenLockProfileConfigs, input};
use async_process::{Child, Command};
use evdev::Key;
use rand::{seq::SliceRandom, thread_rng};
use std::{env::current_exe, path::Path, str::FromStr};
use tokio::sync::Mutex;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct ScreenlockProfile {
    pub profile_name: String,
    pub images: Vec<String>,
    pub block_input: bool,
    pub windowed: bool,
    pub keys: Vec<Key>,
}
impl ScreenlockProfile {
    pub fn from_config(profile_name: String, config: ScreenLockProfileConfigs) -> Self {
        let mut images = Vec::new();
        let mut keys = Vec::new();

        if let Some(cfg_imgs) = config.images {
            cfg_imgs.into_iter().for_each(|i| {
                if Path::new(&i).exists() {
                    for file in WalkDir::new(i).into_iter().filter_map(|e| e.ok()) {
                        let file = file.path();
                        if file.is_file() {
                            images.push(file.display().to_string());
                        }
                    }
                } else {
                    images.push(i);
                }
            });
        }

        if let Some(cfg_keys) = config.keys {
            cfg_keys.into_iter().for_each(|k| {
                if let Ok(k) = Key::from_str(&k) {
                    keys.push(k);
                } else {
                    error!(
                        "Erro ao tentar adicionar a tecla: \"{}\" do perfil \"{}\", ignorando ela...",
                        k, profile_name
                    )
                }
            });
        }

        Self {
            profile_name,
            images,
            block_input: config.block_input.unwrap_or(false),
            windowed: config.windowed.unwrap_or(false),
            keys,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref PROFILES: Mutex<Vec<ScreenlockProfile>> = Mutex::new(Vec::new());
    pub static ref PROCESS_LIST: Mutex<Vec<Child>> = Mutex::new(Vec::new());
}

pub async fn add_profile(profile: ScreenlockProfile) {
    info!("Perfil: \"{}\" carregado", profile.profile_name);
    debug!("{:#?}", profile);
    PROFILES.lock().await.push(profile);
}

pub async fn block_screen(profile: Option<ScreenlockProfile>) {
    if !PROCESS_LIST
        .lock()
        .await
        .iter_mut()
        .any(|x| x.try_status().unwrap().is_none())
    {
        println!("{}", input::are_keys_pressed(vec![]).await);
        let mut selected_profile = None;

        if profile.is_some() {
            selected_profile = Some(profile.unwrap());
        } else {
            let profiles_list = &*PROFILES.lock().await;
            for prof in profiles_list {
                if input::are_keys_pressed(prof.keys.clone()).await {
                    selected_profile = Some(prof.clone());
                }
            }
        }

        let selected_profile = selected_profile.unwrap_or(ScreenlockProfile {
            profile_name: "default".to_owned(),
            images: vec![],
            block_input: false,
            windowed: false,
            keys: vec![],
        });

        //ta foda rust
        let desgraca = &String::new();
        let img = selected_profile.images.choose(&mut thread_rng()).unwrap_or(desgraca);
        let mut gui = Command::new(current_exe().unwrap().to_str().unwrap());

        gui.args(&["gui", "-i", img, "-t", &selected_profile.profile_name]);
        if selected_profile.windowed {
            gui.arg("-w");
        }
        if selected_profile.block_input {
            gui.arg("-H");
        }

        PROCESS_LIST.lock().await.push(gui.spawn().unwrap());
    }
}

pub async fn kill_screen_block() {
    PROCESS_LIST.lock().await.iter_mut().for_each(|x| {
        x.kill().unwrap();
    });
    PROCESS_LIST.lock().await.clear();
}

pub async fn get_profile_by_name(name: String) -> Option<ScreenlockProfile> {
    let profiles_list = &*PROFILES.lock().await;
    let profile = profiles_list.iter().find(|x| x.profile_name == name);
    profile.map(|x| x.clone())
}
