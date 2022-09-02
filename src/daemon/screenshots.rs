use std::{fs, path::Path};

use chrono::Utc;
use notify::{
    event::{AccessKind, AccessMode},
    Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use walkdir::WalkDir;

pub fn start(watch_dir: String, target_dir: String, initial_check: bool) {
    if initial_check {
        for entry in WalkDir::new(&watch_dir) {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    mover(&target_dir, &entry.path().to_str().unwrap().to_string())
                }
            }
        }
    }

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    watcher
        .watch(Path::new(&watch_dir), RecursiveMode::Recursive)
        .unwrap();

    for res in rx {
        match res {
            Ok(event) => {
                if let EventKind::Access(kind) = event.kind {
                    if let AccessKind::Close(mode) = kind {
                        if mode == AccessMode::Write {
                            let source = event.paths[0].as_path();
                            mover(&target_dir, &source.to_str().unwrap().to_string())
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn mover(target_dir: &String, source: &String) {
    let target_dir = Utc::now().format(target_dir).to_string();
    fs::create_dir_all(&target_dir).unwrap();

    let count = WalkDir::new(&format!("{}/..", target_dir))
        .into_iter()
        .filter(|x| x.as_ref().unwrap().path().is_file())
        .count();
    let target_dir = format!("{}/{}.png", target_dir, count + 1);
    fs::copy(source, &target_dir).unwrap();
    fs::remove_file(source).unwrap();
    info!("{} >>> {}", source, target_dir)
}
