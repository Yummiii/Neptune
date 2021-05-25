mod arguments;

use std::{process::{Command}, time::{SystemTime, UNIX_EPOCH}};

use arguments::Options;
use enigo::{Enigo, KeyboardControllable};
use passwords::PasswordGenerator;
use uuid::Uuid;

fn main() {
    let args = Options::build();
    
    if args.password {
        let senha = PasswordGenerator::new()
            .length(50)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .numbers(true)
            .generate_one()
            .unwrap();
        enviar(senha, args.escrever);
    } else if args.timestamp {
        enviar(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("error")
            .as_millis()
            .to_string(), args.escrever);
    } else if args.uuid {
        enviar(Uuid::new_v4().to_string(), args.escrever);
    }
}

fn enviar(conteudo: String, escrever: bool) {
    Command::new("/usr/bin/qdbus")
        .arg("org.kde.klipper")
        .arg("/klipper")
        .arg("setClipboardContents")
        .arg(conteudo)
        .spawn().expect("Erro ao escrever na clipboard");

    if escrever {
        let mut enigo = Enigo::new();
        enigo.key_sequence_parse("{+CTRL}{+SHIFT}v{-CTRL}{-SHIFT}");
    }
}