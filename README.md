# herbst
Bevy jam game about falling leaves


## Run locally
```
cargo run --features bevy/dynamic_linking
```

## Build for the web

### Prerequisites

Install `wasm-bindgen-cli`
```
cargo install wasm-bindgen-cli
```

Copy assets to `out/` directory:
```
cp -r assets out/assets
```

### Building
```
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "herbst" ./target/wasm32-unknown-unknown/release/herbst.wasm
```

### Run locally
```
npm install --global serve
serve out
```