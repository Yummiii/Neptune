use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Comandos
}

#[derive(Clap, Debug)]
pub enum Comandos {
    #[clap(about = "Tira um print")]
    Print(Print),
    #[clap(about = "Gera uma senha")]
    Senha(Senha),
    #[clap(about = "Gera um uuid")]
    Uuid,
    #[clap(about = "Pega o unix time atual")]
    Timestamp
}

#[derive(Clap, Debug)]
pub struct Print {
    #[clap(short, long, about = "Lugar em que o print vai ser salvo compativel com https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html")]
    pub path: String
}

#[derive(Clap, Debug)]
pub struct Senha {
    #[clap(short, long, about = "Tamanho da senha", default_value = "50")]
    pub tamanho: usize
}