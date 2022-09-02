#[macro_use]
extern crate log;
use clap::{Parser, Subcommand};
use std::{env::current_exe, fs, path::Path};

mod daemon;
mod gui;

#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Yummi")]
struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GUI {
        #[clap(value_parser, short, long)]
        image: Option<String>,
        #[clap(value_parser, short = 'H', long, default_value_t = false)]
        hide_cursor: bool,
        #[clap(value_parser, short, long, default_value_t = false)]
        windowed: bool,
        #[clap(value_parser, short, long)]
        title: Option<String>,
    },
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
        Commands::GUI {
            image,
            hide_cursor,
            windowed,
            title
        } => gui::open_gui(image, hide_cursor, windowed, title),
        Commands::DAEMON { config_file } => daemon::start_daemon(config_file).await,
        Commands::ENABLE => enable(),
    }
}

//isso ta muito feio
fn enable() {
    run_script::run_script!("systemctl --user stop neptune").unwrap();
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
    run_script::run_script!("systemctl --user enable --now neptune").unwrap();
    println!("Serviço ligado e habilitado na inicialização do sistema :)")
}
