### Setup

1. Run `npm install` to install the `TextEncoder` polyfill.
2. Install Rust and the `wasm32-unknown-unknown` target with `rustup`.
3. Install LLVM >= 8.0. ([downloads](http://releases.llvm.org/download.html))
4. Make sure the `PATH` variable in `test.sh` points to LLVM >= 8.0

### Testing

The `test.sh` script will compile this project to Wasm/JS and test it with Node.
