use chrono::{Datelike, Utc};
use glob::glob;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs, path::Path, process::Command, sync::mpsc::channel, time::Duration};

fn enviar_img_pra_clipboard(local: String) {
    Command::new("/usr/bin/xclip")
        .arg("-selection")
        .arg("clipboard")
        .arg("-t")
        .arg("image/png")
        .arg("-i")
        .arg(local)
        .spawn().expect("Erro ao escrever na clipboard");
}

fn main() -> notify::Result<()> {
    //println!("{:?}", glob("/home/yummi/Taiga/Printis/**/*.*").unwrap().count());
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch("/home/yummi/teste", RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                if let DebouncedEvent::Create(path) = event {
                    let path = path.to_str().unwrap().to_string();
                    enviar_img_pra_clipboard(path.clone());

                    let utc_now = Utc::now();
                    let local_prints = format!("/home/yummi/Taiga/Printis/{}-{:02}", utc_now.year(), utc_now.month());
                    let local_prints = Path::new(&local_prints);
                    let quantidade = glob(&format!("{}/**/*.*", local_prints.parent().unwrap().to_str().unwrap())).unwrap().count();
                    
                    fs::create_dir_all(local_prints).unwrap();
                    Command::new("/usr/bin/mv")
                        .arg(path)
                        .arg(format!("{}/{}.png", local_prints.to_str().unwrap(), quantidade + 1))
                        .spawn().unwrap();
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}