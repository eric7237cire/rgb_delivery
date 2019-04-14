mod utils;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;

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
    alert("Hello, 433 rgb-solver!");
}


//example to generate typings
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export type Coords = { "latitude": number, "longitude": number, };

"#;

//#[wasm_bindgen]
//https://github.com/tcr/wasm-typescript-definition
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default)]
struct Color {
    pub label: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,

    pub color_index: usize
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default)]
struct Tile {
    pub label: String,
    pub tile_index: usize
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    color_list: [Color; 4],
    tile_list: [Tile; 4]
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }


}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { "uud◻" } else { "a◼" };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }*/
        write!(f,"Nothing for now\n");

        Ok(())
    }
}

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


fn build_color_list() -> [Color; 4] {
    let mut color_list = [
                Color{ label: "White".to_string(), red: 230, green: 230, blue: 230, ..Default::default()},
                Color{ label: "Red".to_string(), red: 255, green: 0, blue: 0, ..Default::default()},
                Color{ label: "Yellow".to_string(), red: 255, green: 255, blue: 0, ..Default::default()},
                Color{ label: "Blue".to_string(), red: 50, green: 50, blue: 255, ..Default::default()},

/*
 colors = [ new Color("rgb(255,0,0)","Red"), new Color("rgb(255,255,0)","Yellow"),
  new Color("rgb(50,50,255)", "Blue"), new Color("rgb(230,230,230)", "White") ] ;
  */
                ];

    color_list.iter_mut().enumerate().map( | (idx, c) | {
        c.color_index = idx;
    });

    color_list
}

fn build_tile_list() -> [Tile; 4] {
    let mut tile_list = [
                Tile { label: "Road".to_string(), ..Default::default()},
                Tile { label: "Block".to_string(), ..Default::default()},
                Tile { label: "Warehouse".to_string(), ..Default::default()},
                Tile { label: "Van".to_string(), ..Default::default()},

                ];

    for (idx, t) in tile_list.iter_mut().enumerate() {
        t.tile_index = idx;
    }


        log!(
                    "Built tile list {:?} ",
                    tile_list
                );

    tile_list
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    // ...

    pub fn new(w: u32, h: u32) -> Universe {

        log!(
                    "Building a new Grid.  [{}, {}] ",
                    w,
                    h
                );

        utils::set_panic_hook();

        let width = w;
        let height = h;

        /*
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();*/

        Universe {
            width,
            height,
            color_list: build_color_list(),
            tile_list: build_tile_list()
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn get_colors(&self) -> JsValue {
        JsValue::from_serde(&self.color_list).unwrap()
    }

    pub fn get_tiles(&self) -> JsValue {
        JsValue::from_serde(&self.tile_list).unwrap()
    }


    pub fn set_square(&self, row: u8, col: u8, color_val: &JsValue, tile_val: &JsValue ) {

        let color: Color = color_val.into_serde().unwrap();
        let tile: Tile = tile_val.into_serde().unwrap();

        log!(
                    "Received [{}, {}] Color {:?} Tile {:?}",
                    row,
                    col,
            color,
            tile
                );

    }
}