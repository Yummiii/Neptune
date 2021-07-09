mod arguments;
mod cmds;

use crate::{arguments::Comandos, cmds::capturar_print};
use arguments::Opts;
use chrono::Utc;
use clap::Clap;
use cmds::{enviar_clipboard, quantidade_arquivos};
use passwords::PasswordGenerator;
use uuid::Uuid;
use std::{fs, time::{SystemTime, UNIX_EPOCH}};

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), GenericError> {
    let opts = Opts::parse();

    match opts.subcmd {
        Comandos::Print(print) => {
            let mut local = Utc::now().format(&print.path).to_string();
            fs::create_dir_all(&local)?;
            local.push_str(&format!(
                "{}.png",
                quantidade_arquivos(&format!("{}..", &local))? + 1
            ));
            capturar_print(&local)?;
        }
        Comandos::Senha(senha) => {
            let senha = PasswordGenerator::new()
                .length(senha.tamanho)
                .lowercase_letters(true)
                .uppercase_letters(true)
                .symbols(true)
                .numbers(true)
                .generate_one()?;
            enviar_clipboard(senha)?;
        }
        Comandos::Uuid => {
            enviar_clipboard(Uuid::new_v4().to_string())?;
        }
        Comandos::Timestamp => {
            enviar_clipboard(SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string())?;
        }
    }

    Ok(())
}
