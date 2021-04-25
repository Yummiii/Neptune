use clipboard::{ClipboardContext, ClipboardProvider};
use passwords::PasswordGenerator;
use umiko::hotkeys::{HotKeys, KeyModifiers};
use enigo::{Enigo, KeyboardControllable};

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

        };
    })

    hk.handle();
}

fn escrever(texto: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(texto).expect("fudeu baia");

    let mut enigo = Enigo::new();
    enigo.key_sequence_parse("{+CTRL}v{-CTRL}");
}