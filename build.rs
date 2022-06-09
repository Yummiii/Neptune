use std::{fs::{self, File}, path::Path};
use whoami::username;

fn main() {
    println!("cargo:rerun-if-changed=src/gui_manager/clibs/nepnep.c");
    println!("cargo:rerun-if-changed=build.rs");

    if !Path::new(&format!("/home/{}/.config/systemd/user/neptune.service", username())).exists() {
        run_script::spawn_script!("systemctl link --user ./neptune.service").unwrap();
    }

    let configs_dir = format!("/home/{}/.config/neptune/", username());
    let configs_file = format!("{}/neptune.toml", configs_dir);
    fs::create_dir_all(&configs_dir).unwrap();
    if !Path::new(&configs_file).exists() {
        File::create(&configs_file).unwrap();
    }

    let mut builder = cc::Build::new();
    builder.file("src/gui_manager/clibs/nepnep.c");
    builder.compiler("cc");
    builder.cpp(true);

    let libs = pkg_config::Config::new().probe("libadwaita-1").expect("libadwaita-1 not found");
    libs.include_paths.iter().for_each(|x| {
        builder.include(x);
    });

    builder.compile("nepnep");
}
