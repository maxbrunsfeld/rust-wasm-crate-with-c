extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/lib.c")
        .compile("wasm-c-component");
}
