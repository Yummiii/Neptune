use clap::{Clap, AppSettings};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Options {
    #[clap(subcommand)]
    pub subcmd: SubCommands
}

#[derive(Clap, Debug)]
pub enum SubCommands {
    Enviar(Enviar),
    Geradores(Geradores)
}

#[derive(Clap, Debug)]
pub struct Enviar {
    #[clap(short, long)]
    pub nome: String
}

#[derive(Clap, Debug)]
pub struct Geradores {
    #[clap(short, long)]
    pub tipo: i32
}

impl Options {
    pub fn build() -> Self {
        Options::parse()
    }
}