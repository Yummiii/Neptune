#[macro_use] extern crate log;
use clap::{Parser, Subcommand};

mod gui;
mod daemon;

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
    },
    DAEMON {
        #[clap(value_parser, short, long, value_name = "FILE")]
        config_file: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    
    let args = Args::parse();
    match args.command {
        Commands::GUI { image, hide_cursor, windowed} => gui::open_gui(image, hide_cursor, windowed),
        Commands::DAEMON { config_file } =>  daemon::start_daemon(config_file).await
    }
}
