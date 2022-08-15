#!/bin/bash

cargo build --bin ctviewer --target wasm32-unknown-unknown
mkdir ./target/debug/web/
cp ./web/* ./target/debug/web/
wasm-bindgen --out-name ctviewer \
  --out-dir target/debug/web/build \
  --target web target/wasm32-unknown-unknown/debug/ctviewer.wasm
basic-http-server ./target/debug/web