cargo build --target wasm32-unknown-unknown
cp ../target/wasm32-unknown-unknown/debug/client-bin.wasm .
basic-http-server .