
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
cargo watch -x "test grid_connections --target x86_64-pc-windows-msvc --lib -- --nocapture"
SET RUST_BACKTRACE=1
cargo watch --ignore *.tree -x "test tree --release --target x86_64-pc-windows-msvc  --lib -- --nocapture"


## Cog with WSL
python3 -m cogapp -r "/mnt/e/git/rgb_delivery/rgb-solver/src/solver/tests.rs"
d:\Miniconda3\envs\scripts\python -m cogapp -r D:\git\rgb_delivery\rgb-solver\src\solver\tests.rs
d:\Miniconda3\envs\scripts\python -m cogapp -r D:\git\rgb_delivery\web_worker\worker\index.ts

## Publishing to npm 
https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/packaging-and-publishing.html

```
wasm-pack login
cd pkg
npm publish --access=public
```

https://github.com/eric7237cire/rgb_delivery/compare/0529801910e8f1ea9d8d3a4274ef976f6353a47e..2c57c7f19cdf639b48cd58fd25a41cb923bc37a5

docker build . -t eric7237cire/rgb
docker login --username=eric7237cire
docker push eric7237cire/rgb

docker run --rm  -it eric7237cire/rgb bash

#San Diego R4
#46,922,652 iterations
#9 secs: 43.90