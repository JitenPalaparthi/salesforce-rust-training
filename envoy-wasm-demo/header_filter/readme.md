rustup target add wasm32-wasip1
cd header_filter
cargo build --release --target wasm32-wasip1


mkdir -p envoy

cp header_filter/target/wasm32-wasip1/release/header_filter.wasm envoy/

touch envoy/envoy.yaml

docker compose up -d

docker compose logs -f envoy

curl -i localhost:10000/

-- directly call upstream
curl -i localhost:5678/