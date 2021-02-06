fn main() {
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
        .pic(true)
        .cpp(true)
        .compile("libsealpir.a");

    println!("cargo:rustc-link-search=/usr/local/lib/");
    println!("cargo:rustc-link-lib=static=seal");
}
