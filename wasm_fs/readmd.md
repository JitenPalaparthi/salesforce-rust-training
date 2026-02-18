cargo build --release --target wasm32-wasip1

wasmtime run target/wasm32-wasip1/release/wasm_fs.wasm

wasmtime run --dir . \
  target/wasm32-wasip1/release/wasm_fs.wasm

  WASI uses capability-based security.

  --dir .