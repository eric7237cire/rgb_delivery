# 🦀 🕸️ `rgb-solver`

Web Assembly written in rust.

Build using --
[`wasm-pack`](https://github.com/rustwasm/wasm-pack)

See main [github repo](https://github.com/eric7237cire/rgb_delivery)

Version History:

# 0.1.0-progress.5 

* Fix bug where a stopped van could be passed through

# 0.1.1

* Refactoring grid connections to be more compact
* Reorg structs to use modules 

00
01
10
11

2 bits per move

8 bits -- stop #

4 bits -- current index

all 1s -- not stopped

