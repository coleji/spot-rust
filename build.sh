#!/bin/bash
rm ./dist -r
mkdir dist
cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/spot_rust.wasm  --no-modules  --out-dir ./dist
