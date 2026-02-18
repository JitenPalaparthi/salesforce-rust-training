rustup target list --installed
rustup target add wasm32-wasip1

cargo build --release --target wasm32-wasip1

wasmtime run target/wasm32-wasip1/release/hello_wasm_runtime.wasm --foo bar