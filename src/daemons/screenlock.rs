use std::{env::current_exe, process::Command, sync::Arc};
use rand::prelude::SliceRandom;
use tokio::sync::{
    mpsc::{self, Sender},
    Mutex,
};

lazy_static::lazy_static! {
    static ref PROCESS_LIST: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    pub static ref TX: Mutex<Option<Arc<Sender<(u8, Option<String>, Option<bool>)>>>> = Mutex::new(None);
    static ref GRAB_INPUT: Mutex<bool> = Mutex::new(false);
    static ref IMGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub async fn init(grab_input: bool) {
    *GRAB_INPUT.lock().await = grab_input;

    let (tx, mut rx) = mpsc::channel(1);
    *TX.lock().await = Some(Arc::new(tx));
    let mut active = false;

    while let Some(cmd) = rx.recv().await {
        println!("Command received: {}", cmd.0);

        if cmd.0 == 1 && !active {
            active = true;
            info!("Starting screen block");

            let mut img = cmd.1.clone();
            println!("{:?}", img);
            if cmd.1.is_none() && IMGS.lock().await.len() >= 1 {
                let img_list = IMGS.lock().await;
                img = Some(img_list.choose(&mut rand::thread_rng()).unwrap().to_string());
            }
            block_screen(img, cmd.2.unwrap_or(*GRAB_INPUT.lock().await)).await;
        } else if cmd.0 == 2 {
            active = false;
            info!("Killing screen block");
            kill_screen_block().await;
        }
    }
}

pub async fn add_img(img: String) {
    info!("Added image: [{}]", img);
    IMGS.lock().await.push(img);
}

async fn block_screen(img: Option<String>, grab_input: bool) {
    let mut cmd = Command::new(current_exe().unwrap());

    cmd.arg("gui");
    if !grab_input {
        cmd.arg("-s");
    }
    if let Some(img) = img {
        cmd.arg("-i");
        cmd.arg(img);
    }
    let gui = cmd.spawn().unwrap();
    PROCESS_LIST.lock().await.push(gui.id());
}

async fn kill_screen_block() {
    PROCESS_LIST.lock().await.iter().for_each(|id| {
        run_script::spawn_script!(format!("kill $(pstree -p {id} | grep -o '[0-9]*')")).unwrap();
    });
    PROCESS_LIST.lock().await.clear();
}

pub async fn send_message(cmd: u8, img: Option<String>, grab_input: Option<bool>) {
    if let Some(tx) = TX.lock().await.as_ref() {
        let tx = Arc::clone(&tx);

        // não faça perguntas
        tx.send((cmd, img, grab_input)).await.unwrap();
        println!("pedro de alcantra");
        tx.send((0, None, None)).await.unwrap();
    }
}