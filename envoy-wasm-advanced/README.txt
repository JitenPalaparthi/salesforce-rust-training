
BUILD WASM:
cd wasm-auth-filter
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1

Copy wasm:
cp target/wasm32-wasip1/release/wasm_auth_filter.wasm ../envoy/auth_filter.wasm

RUN:
docker compose up --build

TEST:
curl localhost:10000/
curl -H "Authorization: Bearer deny" localhost:10000/
curl -H "Authorization: Bearer allow" localhost:10000/
