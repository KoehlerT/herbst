# herbst
Bevy jam game about falling leaves


## Run locally
```
cargo run --features bevy/dynamic_linking
```

## Build for the web

```
cargo install wasm-bindgen-cli

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "herbst" ./target/wasm32-unknown-unknown/release/herbst.wasm

cp -r assets out/assets
```