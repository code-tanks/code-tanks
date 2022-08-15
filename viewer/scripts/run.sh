#!/bin/bash

mkdir ./target/debug/web/
cp -r ./web/* ./target/debug/web/
cargo build --bin ctviewer --target wasm32-unknown-unknown
wasm-bindgen --out-name ctviewer \
  --out-dir target/debug/web/build \
  --target web target/wasm32-unknown-unknown/debug/ctviewer.wasm
basic-http-server ./target/debug/web