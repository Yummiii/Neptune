use super::configs::ScreenLockProfileConfigs;
use evdev::Key;
use rand::{seq::SliceRandom, thread_rng};
use std::{path::Path, str::FromStr, env::current_exe};
use tokio::sync::Mutex;
use walkdir::WalkDir;

#[derive(Debug)]
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
                        images.push(file.path().display().to_string());
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
    static ref PROFILES: Mutex<Vec<ScreenlockProfile>> = Mutex::new(Vec::new());
}

pub async fn add_profile(profile: ScreenlockProfile) {
    info!("Perfil: \"{}\" carregado", profile.profile_name);
    debug!("{:#?}", profile);
    PROFILES.lock().await.push(profile);
}

pub async fn block_screen() {
    let profile = &PROFILES.lock().await[0];
    println!("{}", profile.profile_name);
    let img = profile.images.choose(&mut thread_rng()).unwrap();

    run_script::spawn_script!(format!("{} gui -i {} -w", current_exe().unwrap().to_str().unwrap(), img)).ok();
}