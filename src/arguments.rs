use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Yummi")]
pub struct LaunchOptions {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    GUI {
        #[clap(value_parser, short, long)]
        image: Option<String>,
        #[clap(value_parser, short, long, default_value_t = false)]
        show_cursor: bool
    },
    Daemons {
        #[clap(value_parser, short, long, default_value = "a")]
        config_file: String
    }
}

impl LaunchOptions {
    pub fn build() -> Self {
        LaunchOptions::parse()
    }
}