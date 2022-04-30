use tokio::sync::mpsc::Receiver;

pub async fn init(mut rx: Receiver<i32>) {

    while let Some(msg) = rx.recv().await {
        println!("{}", msg);

        if msg == 1 {
            run_script::spawn_script!("/home/yummi/Taiga/CodigosFodas/Neptune/telas_legais/build/src/./nepnep").unwrap();
        } else if msg == 2 {
            run_script::spawn_script!("killall -9 nepnep").unwrap();
        }
    }
}
