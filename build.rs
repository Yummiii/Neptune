fn main() {
    println!("cargo:rerun-if-changed=src/gui_manager/clibs/nepnep.c");
    println!("cargo:rerun-if-changed=build.rs");

    let mut builder = cc::Build::new();
    builder.file("src/gui_manager/clibs/nepnep.c");
    builder.compiler("cc");
    builder.cpp(true);

    let libs = pkg_config::Config::new().probe("libadwaita-1").expect("libadwaita-1 not found");
    libs.include_paths.iter().for_each(|x| {
        builder.include(x);
    });

    builder.shared_flag(true);
    builder.static_flag(true);

    builder.compile("nepnep");
}
