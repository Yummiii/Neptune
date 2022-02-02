use std::fs::{File, self};
use chrono::Utc;
use daemonize::Daemonize;
use fs_extra::{file::CopyOptions};
use hotwatch::{blocking::{Hotwatch, Flow}, Event};
use run_script::ScriptOptions;

const WORK_DIR: &str = "/home/yummi/.nepnep/";

fn main() {
    fs::create_dir_all(format!("{}prints", WORK_DIR)).unwrap();
    let stdout = File::create(format!("{}daemon.out", WORK_DIR)).unwrap();
    let stderr = File::create(format!("{}daemon.err", WORK_DIR)).unwrap();

    let daemonize = Daemonize::new()
        .working_directory(WORK_DIR)
        .stdout(stdout)  
        .stderr(stderr); 

    match daemonize.start() {
        Ok(_) => {
            let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
            hotwatch.watch(format!("{}prints/", WORK_DIR), |event: Event| {
                println!("{:#?}", event);
                if let Event::Create(path) = event {
                    let mut local = Utc::now().format("/home/yummi/Taiga/Printis/%Y-%m/").to_string();
                    fs::create_dir_all(&local).unwrap();
    
                    let (quantidade, _) = run_script::run_script_or_exit!(r#"find $1../ -type f | wc -l"#, &vec![local.clone()], &ScriptOptions::new());
                    let proximo = quantidade.trim().parse::<i32>().unwrap() + 1;
                    local.push_str(&format!("{}.png", proximo));
                    
                    println!("Original: {}\nPara: {}", path.to_str().unwrap(), &local);
                    fs_extra::file::move_file(path, local, &CopyOptions::new()).unwrap();
                }
                Flow::Continue
            }).unwrap();
            hotwatch.run();
        }
        Err(e) => eprintln!("Error, {}", e),
    }
}
