use clap::{Clap, AppSettings};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Options {
    #[clap(short, long)]
    pub password: bool,
    #[clap(short, long)]
    pub uuid: bool,
    #[clap(short, long)]
    pub timestamp: bool,
    #[clap(short, long)]
    pub escrever: bool
}

impl Options {
    pub fn build() -> Self {
        Options::parse()
    }
}