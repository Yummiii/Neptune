use std::time::{SystemTime, UNIX_EPOCH};
use clipboard::{ClipboardContext, ClipboardProvider};
use passwords::PasswordGenerator;
use umiko::hotkeys::{HotKeys, KeyModifiers};
use enigo::{Enigo, KeyboardControllable};
use uuid::Uuid;

fn main() {    
    let mut hk = HotKeys::new();
    hk.add(KeyModifiers::MOD_CONTROL, 'p', || {
        if umiko::keys::is_locked(0x91) {
            let senha = PasswordGenerator::new()
                .length(50)
                .lowercase_letters(true)
                .uppercase_letters(true)
                .symbols(true)
                .numbers(true)
                .generate_one()
                .unwrap();
            escrever(senha);
        }
    });

    hk.add(KeyModifiers::MOD_CONTROL, 'g', || {
        if umiko::keys::is_locked(0x91) {
            escrever(Uuid::new_v4().to_string());
        };
    });

    hk.add(KeyModifiers::MOD_CONTROL, 'o', || {
        if umiko::keys::is_locked(0x91) {
            escrever(SystemTime::now().duration_since(UNIX_EPOCH).expect("error").as_millis().to_string());
        };
    });

    hk.handle();
}

fn escrever(texto: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(texto).expect("fudeu baia");

    let mut enigo = Enigo::new();
    enigo.key_sequence_parse("{+CTRL}v{-CTRL}");
}