# How to compile client

https://macroquad.rs/articles/wasm/

**In the client folder:**
Run `./run.sh`

**Manually:**

- run `cargo build --target wasm32-unknown-unknown`
- copy wasm to www folder: `cp ../target/wasm32-unknown-unknown/debug/client-bin.wasm .`
- `basic-http-server .` (cargo install this if you don't have it)
