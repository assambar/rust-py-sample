#!/usr/bin/env bash

if ! (rustup target list | grep "installed" | grep -q "wasm32-wasi"); then
    echo "Please run 'rustup target add wasm32-wasi' first"
    exit 1
fi

set -e

PYO3_NO_PYTHON=1 cargo build --target=wasm32-wasi
wasmtime \
    --mapdir /data::data/ \
    --mapdir /usr::target/wasm32-wasi/wasi-deps/usr \
    target/wasm32-wasi/debug/rust-py-sample.wasm \
    data/sample_script.py  --image-path data/input.jpg
