use std::time::{SystemTime, UNIX_EPOCH};
use clipboard::{ClipboardContext, ClipboardProvider};
use passwords::PasswordGenerator;
use umiko::{common::Keys, hotkeys::{HotKeys, KeyModifiers}};
use enigo::{Enigo, KeyboardControllable};
use uuid::Uuid;

fn main() {   
    let mut hkhandler  = HotKeys::new();
    registrar(&mut hkhandler);
    hkhandler.handle();
}

fn registrar(handler: &mut HotKeys) {
    let mut hotkeys = Vec::new();

    hotkeys.push(handler.add(KeyModifiers::MOD_CONTROL, Keys::P, |_| {
        let senha = PasswordGenerator::new()
            .length(50)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .numbers(true)
            .generate_one()
            .unwrap();
        escrever(senha);        
    }));

    hotkeys.push(handler.add(KeyModifiers::MOD_CONTROL, Keys::G, |_| {
        escrever(Uuid::new_v4().to_string());
    }));

    hotkeys.push(handler.add(KeyModifiers::MOD_CONTROL, Keys::O, |_| {
        escrever(SystemTime::now().duration_since(UNIX_EPOCH).expect("error").as_millis().to_string());
    }));

    handler.add(KeyModifiers::NONE_MODIFIES, Keys::SCROLL_LOCK, move |h| {
        if umiko::keys::is_locked(Keys::SCROLL_LOCK) {
            registrar(h);
            println!("registradas");
        } else {
            for hk in hotkeys.iter() {
                h.remove(*hk);
            }
            println!("Desregistradas");
        }
    });
}

fn escrever(texto: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(texto).expect("fudeu baia");

    let mut enigo = Enigo::new();
    enigo.key_sequence_parse("{+CTRL}v{-CTRL}");
}