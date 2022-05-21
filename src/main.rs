mod serial;
mod btn_handler;
mod gui_manager;
mod device_helpers;

use std::fs;
use chrono::Utc;
use fs_extra::file::CopyOptions;
use hotwatch::{blocking::{Hotwatch, Flow}, Event};
use run_script::ScriptOptions;
use tokio::task;

#[tokio::main]
async fn main() {    
    task::spawn(async {
        serial::iniciar_serial().await;
    });
    let work_dir: String = format!("{}/Capturas de tela", xdg_user::pictures().unwrap().unwrap().display());
    println!("Work dir: {}\n", work_dir);

    fs::create_dir_all(&work_dir).unwrap();
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch.watch(work_dir, |event: Event| {
        if let Event::Create(path) = event {
            let mut local = Utc::now().format("/home/yummi/Taiga/Printis/%Y-%m/").to_string();
            fs::create_dir_all(&local).unwrap();

            let (quantidade, _) = run_script::run_script_or_exit!(r#"find $1../ -type f | wc -l"#, &vec![local.clone()], &ScriptOptions::new());
            let proximo = quantidade.trim().parse::<i32>().unwrap() + 1;
            local.push_str(&format!("{}.png", proximo));
            
            println!("\nOriginal: {}\nPara: {}", path.to_str().unwrap(), &local);
            fs_extra::file::move_file(path, local, &CopyOptions::new()).unwrap();
        }
        Flow::Continue
    }).unwrap();
    hotwatch.run();
}
