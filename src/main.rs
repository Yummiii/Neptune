mod arguments;
mod utils;

use std::time::{SystemTime, UNIX_EPOCH};
use arguments::{Options, SubCommands};
use passwords::PasswordGenerator;
use utils::enviar;
use uuid::Uuid;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), GenericError> {
    let args = Options::build();    
    match args.subcmd {
        SubCommands::Geradores(gerador) => {
            if gerador.tipo == 1 {
                let senha = PasswordGenerator::new()
                    .length(50)
                    .lowercase_letters(true)
                    .uppercase_letters(true)
                    .symbols(true)
                    .numbers(true)
                    .generate_one()?;
                enviar(senha)?;
            } else if gerador.tipo == 2 {
                enviar(Uuid::new_v4().to_string())?;
            } else if gerador.tipo == 3 {
                enviar(SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_millis()
                    .to_string())?;
            }
        },
        SubCommands::Enviar(..) => {
            println!("Enviar")
        }
    }

    Ok(())
}