#![feature(uniform_paths)]

//Serialize/Deserialize traints
#[macro_use]
extern crate serde_derive;

extern crate web_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//Calling a JS function
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rgb-solver!");
}

//example to generate typings, perpended to the rest
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

"#;

mod solver;

pub use solver::*;

use std::fmt;
use crate::solver::struct_defs::Universe;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { "uud◻" } else { "a◼" };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }*/
        write!(f,"Nothing for now\n")?;

        Ok(())
    }
}
