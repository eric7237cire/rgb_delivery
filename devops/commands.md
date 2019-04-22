
Cargo watch wasm-pack:
https://github.com/rustwasm/wasm-pack/issues/457

cargo install cargo-watch
# in rgb-solver
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build" -w ../wasm-typescript-definition -w .

# Runs wasm + webworker
cargo watch -i .gitignore -i "pkg/*" -s "E:\git\rgb_delivery\build-wasm.bat" -w ../wasm-typescript-definition -w .

# Running tests
cargo test --target x86_64-pc-windows-msvc  -- --nocapture

## tests in watch mode
cargo watch -x "test --target x86_64-pc-windows-msvc --lib -- --nocapture"

# watch just one test
cargo watch -x "test test_london_h10 --target x86_64-pc-windows-msvc --lib -- --nocapture"


## Cog with WSL
python3 -m cogapp -r "/mnt/e/git/rgb_delivery/rgb-solver/src/solver/tests.rs"

## Publishing to npm 
https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/packaging-and-publishing.html

```
wasm-pack login
cd pkg
npm publish --access=public
```