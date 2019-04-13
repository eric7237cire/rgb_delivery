mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rgb-solver!");
}


#[no_mangle]
pub fn add(first: i32, second:i32) -> i32 {
  first + second
}
#[no_mangle]
pub fn subtract(first: i32, second:i32) -> i32 {
  first - second
}
#[no_mangle]
pub fn multiply(first: i32, second:i32) -> i32 {
  first * second
}