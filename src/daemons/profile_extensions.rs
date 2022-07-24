use super::configs::ScreenLockProfileConfigs;
use rand::prelude::SliceRandom;
use walkdir::WalkDir;

pub fn get_image(profile: &ScreenLockProfileConfigs) -> Option<String> {
    let mut imgs_list = Vec::new();

    if let Some(imgs) = profile.images.clone() {
        for img in imgs {
            imgs_list.push(img);
        }
    }

    if let Some(dirs) = profile.images_dirs.clone() {
        for dir in dirs {
            for file in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
                imgs_list.push(file.path().display().to_string());
            }
        }
    }

    if let Some(img) = imgs_list.choose(&mut rand::thread_rng()) {
        Some(img.to_string())
    } else {
        None
    }
}
