# rust-py-sample
Wasm module in Rust embedding Python via pyo3

This is an experimental, work-in-progress project. Code from here may disappear when it's no longer necessary.

Relevant parts will be moved to [https://github.com/vmware-labs/webassembly-language-runtimes/tree/main/python/examples](https://github.com/vmware-labs/webassembly-language-runtimes/tree/main/python/examples).

# Sample run

Requires

 - `bash`
 - `cargo` with the `wasm32-wasi` toolchain
 - `wasmtime`


```
$$ PYO3_NO_PYTHON=1 cargo build --target=wasm32-wasi

   Compiling ...
    Finished dev [unoptimized + debuginfo] target(s) in 4.67s


$$ wasmtime \
    --mapdir /data::data/ \
    --mapdir /usr::target/wasm32-wasi/wasi-deps/usr \
    target/wasm32-wasi/debug/rust-py-sample.wasm

Transformed /data/input.jpg to tensor: [39, 79, 123, 63, 39, 108, 134, 63, 218, 60, 88, 63, 218, 60, 88, 63, 223, 97, 145, 63, 116, 126, 169, 63, 225, 185, 160, 63, 179, 179, 70, 63, 225, 185, 160, 63, 44, 116, 180, 63, 39, 108, 134, 63, 24, 143, 0, 63, 188, 136, 158, 63, 151, 87, 156, 63, 67, 200, 48, 63, 67, 200, 48, 63, 228, 7, 145, 63, 230, 192, 151, 63, 13, 195, 112, 63, 100, 62, 117, 63, 234, 50, 165, 63, 232, 121, 158, 63, 228, 7, 145, 63, 248, 136, 45, 63, 154, 226, 180, 63, 63, 245, 162, 63, 79, 4, 50, 63, 16, 195, 176, 62, 154, 226, 180, 63, 232, 121, 158, 63, 157, 155, 27, 63, 166, 127, 54, 63, 107, 94, 168, 63, 138, 153, 170, 63, 159, 231, 134, 63, 251, 152, 141, 63, 127, 114, 188, 63, 22, 167, 101, 63, 137, 224, 16, 63, 4, 221, 58, 62, 147, 134, 208, 63, 249, 165, 43, 63, 249, 214, 52, 191, 7, 161, 95, 190, 14, 115, 217, 63, 206, 9, 115, 63, 25, 43, 23, 62, 115, 146, 52, 63]
```

Mapping `data` to `/data` is to make the `data/input.jpg` image visible to the Wasm module.

Mapping `target/wasm32-wasi/wasi-deps/usr` to `/usr` is to make the Python builtin modules visible to the Wasm module (`python.wasm` will fail otherwise).
