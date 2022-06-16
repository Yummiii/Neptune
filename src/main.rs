use arguments::{Commands, LaunchOptions};
use gui::GuiConfigs;

mod arguments;
mod daemons;
mod gui;

#[tokio::main]
async fn main() {
    let args = LaunchOptions::build();
    match args.command {
        Commands::GUI { image, show_cursor } => gui::open(GuiConfigs { image, show_cursor }),
        Commands::Daemons { config_file } => daemons::start_daemons(config_file)
    }
}
