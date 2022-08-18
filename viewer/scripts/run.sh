#!/bin/bash

mkdir ./target/release/web/
cp -r ./web/* ./target/release/web/
cargo build --bin ctviewer --target wasm32-unknown-unknown --release
wasm-bindgen --out-name ctviewer \
  --out-dir target/release/web/build \
  --target web target/wasm32-unknown-unknown/release/ctviewer.wasm \
  && basic-http-server ./target/release/web