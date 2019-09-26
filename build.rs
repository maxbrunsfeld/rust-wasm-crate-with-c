extern crate cc;

fn main() {
    let mut config = cc::Build::new();
    config
        .file("src/lib.c")
        .compile("wasm-lib-with-c");
}
