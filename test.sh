#!/bin/bash

# Must be LLVM version >= 8
export PATH=/opt/local/llvm/bin:${PATH}

# Compile the Rust/C into JS/Wasm
AR=llvm-ar cargo build --target=wasm32-unknown-unknown
wasm-bindgen \
  target/wasm32-unknown-unknown/debug/wasm_lib_with_c.wasm \
  --target web \
  --out-dir pkg

# Convert the generated `.js` file into a node-compatible `.mjs` file.
cat > pkg/wasm_lib_with_c.mjs <<JS
import encoding from 'text-encoding';
import fs from 'fs';

const {TextDecoder} = encoding;

async function fetch(path) {
  return {
    async arrayBuffer() {
      return fs.readFileSync(path);
    }
  }
}
JS
cat pkg/wasm_lib_with_c.js >> pkg/wasm_lib_with_c.mjs

# Run the script with Node.
node --experimental-modules test.mjs
