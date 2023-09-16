
 cargo build --target wasm32-unknown-unknown --release
 wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/rust_game_huarong_dao.wasm