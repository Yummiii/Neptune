fn main() {
    println!("cargo:rerun-if-changed=clibs/nepnep.c");
    println!("cargo:rerun-if-changed=build.rs");

    let mut builder = cc::Build::new();
    builder.file("clibs/nepnep.c");
    builder.cpp(true);

    // for (key, value) in std::env::vars_os() {
    //     println!("cargo:warning={key:?}: {value:?}");
    // }
    
    if let Some(include) = std::env::var_os("DEP_LIBADWAITA_1_INCLUDE") {
        for dir in include.to_str().unwrap().split(":") {
            builder.include(dir);
        }
    }

    //println!("cargo:rustc-link-lib=nepnep");
    builder.compile("nepnep");
}
