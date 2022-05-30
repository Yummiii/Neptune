use std::fs;
use chrono::Utc;
use fs_extra::file::CopyOptions;
use hotwatch::{blocking::{Hotwatch, Flow}, Event};
use run_script::ScriptOptions;

use crate::configs::ScreenshotConfigs;

pub async fn iniciar(cfgs: ScreenshotConfigs) {
    println!("Screenshots watch dir: {}", cfgs.screenshots_path);
    println!("Screenshots redirect dir: {}", cfgs.screenshots_redirect_path);

    fs::create_dir_all(&cfgs.screenshots_path).unwrap();
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch.watch(&cfgs.screenshots_path, move |event: Event| {
        if let Event::Create(path) = event {
            let mut local = Utc::now().format(&cfgs.screenshots_redirect_path).to_string();
            fs::create_dir_all(&local).unwrap();

            let (quantidade, _) = run_script::run_script_or_exit!(r#"find $1../ -type f | wc -l"#, &vec![local.clone()], &ScriptOptions::new());
            let proximo = quantidade.trim().parse::<i32>().unwrap() + 1;
            local.push_str(&format!("{}.png", proximo));
            
            println!("\nOriginal: {}\nPara: {}", path.to_str().unwrap(), &local);
            fs_extra::file::move_file(path, local, &CopyOptions::new()).unwrap();
        }
        Flow::Continue
    }).expect("hotwatch failed to watch!");
    hotwatch.run();
}