#[macro_use]
extern crate log;
use clap::{Parser, Subcommand};
use std::{env::current_exe, fs, path::Path};

mod daemon;
#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Yummi")]
struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    DAEMON {
        #[clap(value_parser, short, long, value_name = "FILE")]
        config_file: Option<String>,
    },
    ENABLE,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let args = Args::parse();
    match args.command {
        Commands::DAEMON { config_file } => daemon::start_daemon(config_file).await,
        Commands::ENABLE => enable(),
    }
}

//isso ta muito feio
fn enable() {
    let dir = format!(
        "{}/.config/systemd/user",
        home::home_dir().unwrap().to_str().unwrap()
    );
    fs::create_dir_all(&dir).unwrap();
    let dir = format!("{}/neptune.service", dir);
    if Path::exists(Path::new(&dir)) {
        fs::remove_file(&dir).unwrap();
    }
    let service = include_str!("../neptune.service");
    let service = service.replace(
        "{path}",
        &format!("{} daemon", current_exe().unwrap().to_str().unwrap()),
    );
    fs::write(dir, service).unwrap();
    println!(":)")
}
