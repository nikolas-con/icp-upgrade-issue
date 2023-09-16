
# build child wasm
cargo build --target wasm32-unknown-unknown --package  backend --release
ic-wasm target/wasm32-unknown-unknown/release/backend.wasm -o target/wasm32-unknown-unknown/release/backend_opt.wasm shrink

# copy to build folder
cp target/wasm32-unknown-unknown/release/backend_opt.wasm  build/backend_v2.wasm

