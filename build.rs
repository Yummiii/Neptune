use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    run_script::spawn_script!(format!("clang clibs/nepnep.c $(pkg-config --cflags --libs libadwaita-1) -o {}/../../../nepnep", out_dir)).unwrap();
}
