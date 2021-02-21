use std::env;
fn main() {
    let verbose = match env::var("SEALPIR_VERBOSE") {
        Ok(v) => if v == "1" {
            "VERBOSE"
        } else {
            "NONVERBOSE"
        },
        Err(_) => "NONVERBOSE"
    };

    cc::Build::new()
        .file("sealpir/pir.cpp")
        .file("sealpir/pir_server.cpp")
        .file("sealpir/pir_client.cpp")
        .file("sealpir-bindings/pir_rust.cpp")
        .include("sealpir-bindings/")
        .include("sealpir/")
        .flag("-Wno-unknown-pragmas")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        .flag("-std=c++17")
        .flag("-fopenmp")
        .flag("-O3")
        .pic(true)
        .cpp(true)
        .define(verbose, None)
        .compile("libsealpir.a");

    println!("cargo:rerun-if-env-changed=SEALPIR_VERBOSE");
    println!("cargo:rustc-link-search=/usr/local/lib/");
    println!("cargo:rustc-link-lib=static=seal");
}
