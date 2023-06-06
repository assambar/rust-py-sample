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
    target/wasm32-wasi/debug/rust-py-sample.wasm \
    data/sample_script.py  --image-path data/input.jpg

Calling //data/sample_script.py with args: ['data/sample_script.py', '--image-path', 'data/input.jpg']
Transformed data/input.jpg to tensor[2, 2]: [4, 147, 147, 63, 221, 236, 118, 63, 225, 185, 160, 63, 144, 218, 83, 63, 61, 60, 156, 63, 13, 195, 112, 63, 63, 245, 162, 63, 161, 13, 41, 63, 179, 251, 154, 63, 65, 67, 30, 63, 159, 231, 134, 63, 228, 54, 6, 60]
```

Mapping `data` to `/data` is to make the `data/input.jpg` image visible to the Wasm module.

Mapping `target/wasm32-wasi/wasi-deps/usr` to `/usr` is to make the Python builtin modules visible to the Wasm module (`python.wasm` will fail otherwise).
