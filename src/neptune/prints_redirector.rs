use std::fs;
use chrono::Utc;
use fs_extra::file::CopyOptions;
use hotwatch::{blocking::{Hotwatch, Flow}, Event};
use run_script::ScriptOptions;

use crate::configs::ScreenshotConfigs;

pub async fn iniciar(cfgs: ScreenshotConfigs) {
    let watch_dir =  &cfgs.screenshots_watch_dir.unwrap();
    let target_dir = cfgs.screenshots_target_dir.unwrap();

    println!("Screenshots watch dir: {}", watch_dir);
    println!("Screenshots target dir: {}", target_dir);

    if cfgs.initial_check.unwrap_or(false) {
        println!("Initial screenshot check");
        for file in fs::read_dir(watch_dir).unwrap() {
            let file = file.unwrap();
            mover(&target_dir, &file.path().as_os_str().to_str().unwrap().to_string());
        }
    }

    fs::create_dir_all(watch_dir).unwrap();
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch.watch(watch_dir, move |event: Event| {
        if let Event::Create(path) = event {
            mover(&target_dir, &path.to_str().unwrap().to_string());
        }
        Flow::Continue
    }).expect("hotwatch failed to watch!");
    hotwatch.run();
}

fn mover(target_dir: &String, source: &String) {
    let mut local = Utc::now().format(target_dir).to_string();
    fs::create_dir_all(&local).unwrap();

    let (quantidade, _) = run_script::run_script_or_exit!(r#"find $1../ -type f | wc -l"#, &vec![local.clone()], &ScriptOptions::new());
    let proximo = quantidade.trim().parse::<i32>().unwrap() + 1;
    local.push_str(&format!("{}.png", proximo));
    
    println!("\nOriginal: {}\nPara: {}", source, local);
    fs_extra::file::move_file(source, local, &CopyOptions::new()).unwrap();
}