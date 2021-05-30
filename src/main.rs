mod configs;
mod arguments;
mod utils;

use passwords::PasswordGenerator;
use utils::enviar;
use uuid::Uuid;
use crate::{arguments::Options, configs::Configs};

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), GenericError> {
    let args = Options::build();
    let cfgs: Configs = confy::load_path("nepnep.toml")?;

    if args.password {
        let senha = PasswordGenerator::new()
            .length(cfgs.tamanho_senha as usize)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .numbers(true)
            .generate_one()?;
        enviar(senha)?;
    } else if args.uuid {
        enviar(Uuid::new_v4().to_string())?;
    }

    Ok(())
}