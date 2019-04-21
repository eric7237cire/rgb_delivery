[![Build Status](https://travis-ci.org/eric7237cire/poker.svg?branch=master)](https://travis-ci.org/eric7237cire/poker)

# A grid solver for [RGB Express](http://rgbexpress.com/) ![Rust](http://rust-lang.org/logos/rust-logo-32x32.png) 
<img src="https://camo.githubusercontent.com/dd1c37a7b0d62d08e89b887ac73a4c4ca4940f97/68747470733a2f2f692e696d6775722e636f6d2f6d58626c5233392e706e67" alt="drawing" width="32"/>
<img src="https://github.com/remojansen/logo.ts/raw/master/ts.png" alt="TypeScript Logo" width=32/>

## Components

### grid-editor

Angular front end, grid editor.

![Screenshot](./readme_images/grid_editor.png)

Language: TypeScript

### web_worker_build

Packages the WASM using a stand alone webpack as a Web worker.  Too much of a PITA to get angular cli
to play nicely.  Copies the output directly to angulars assets.

Language: TypeScript

### rgb-solver

WASM Source.  Unit tests run in x86 (or linux on travis)

Language: Rust