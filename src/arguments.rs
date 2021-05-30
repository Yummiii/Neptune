use clap::{Clap, AppSettings};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Options {
    #[clap(short, long)]
    pub password: bool
}

impl Options {
    pub fn build() -> Self {
        Options::parse()
    }
}