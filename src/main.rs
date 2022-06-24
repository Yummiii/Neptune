extern crate pretty_env_logger;
#[macro_use] extern crate log;

use arguments::{Commands, LaunchOptions};

mod daemon;
mod arguments;
mod gui_manager;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = LaunchOptions::build();    
    trace!("Received arguments: {:?}", args);
    match args.command {
        Commands::GUI { image, show_cursor } => gui_manager::open_block_gui(image, show_cursor),
        Commands::DAEMON { config_file } => daemon::start(config_file)
    }
}
