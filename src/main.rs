#![feature(proc_macro_hygiene)]
extern crate command_macros;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use clap::{Parser, Subcommand};

mod gui;
mod daemons;

#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Yummi")]
struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Opens the GUI
    GUI {
        #[clap(value_parser, short, long)]
        image: Option<String>,
        #[clap(value_parser, short, long, default_value_t = false)]
        show_cursor: bool,
        #[clap(value_parser, short, long, default_value_t = false)]
        windowed: bool,
        /// This does literally nothing
        #[clap(value_parser, short, long, default_value_t = false)]
        nothing: bool
    },
    /// Starts the neptune daemon
    DAEMON {
        #[clap(value_parser, short, long)]
        config_file: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init_custom_env("NEPTUNE_LOG");

    let args = Args::parse();
    match args.command {
        Commands::GUI { image, show_cursor, windowed, nothing: _ } => gui::open_gui(image, show_cursor, windowed),
        Commands::DAEMON { config_file } => daemons::start_daemons(config_file).await,
    }
}
