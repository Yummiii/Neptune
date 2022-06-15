use run_script::ScriptOptions;

pub fn get_keyboard_num() -> i32 {
    let (code, output, _) = run_script::run_script!(r#"grep -E 'Handlers|EV=' /proc/bus/input/devices | grep -B1 'EV=120013' | grep -Eo 'event[0-9]+' | grep -Eo '[0-9]+' | tr -d '\n'"#, &vec![], &ScriptOptions::new()).unwrap();
    if code == 0 {
        output.parse::<i32>().unwrap()
    } else {
        panic!("não achei o numero do teclado")
    }
}

pub fn get_mouse_num() -> i32 {
    let (code, output, _) = run_script::run_script!(r#"grep -E 'Handlers|EV=' /proc/bus/input/devices | grep -B1 'EV=17' | grep -Eo 'event[0-9]+' | grep -Eo '[0-9]+' | tr -d '\n'"#, &vec![], &ScriptOptions::new()).unwrap();
    if code == 0 {
        output.parse::<i32>().unwrap()
    } else {
        panic!("não achei o numero do mouse")
    }
}
