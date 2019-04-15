mod utils;

//Serialize/Deserialize traints
#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;

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

export type UniverseDataFixed = {
"width" : number , "height" : number , "cells" : Array<CellData> } ;

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

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition)]
struct CellData {
    pub tile: Tile,
    pub color: Option<Color>,

    pub used_mask: u8,

    pub row_index: usize,
    pub col_index: usize,
}

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Default)]
pub struct UniverseData {
    width: usize,
    height: usize,

    cells: Vec<Option<CellData>>,
}

#[wasm_bindgen()]
pub struct Universe {
    data: UniverseData
}

impl Universe {

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
        write!(f,"Nothing for now\n")?;

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

    for (idx, c) in color_list.iter_mut().enumerate() {
        c.color_index = idx;
    }

    color_list
}

const VAN_LABEL: &str = "Van";

fn build_tile_list() -> [Tile; 4] {
    let mut tile_list = [
                Tile { label: "Road".to_string(), ..Default::default()},
               /* Tile { label: "Road Used NE".to_string(), ..Default::default()},
                Tile { label: "Road Used NW".to_string(), ..Default::default()},
        Tile { label: "Road Used SW".to_string(), ..Default::default()},
        Tile { label: "Road Used SE".to_string(), ..Default::default()},
        Tile { label: "Road Used NS".to_string(), ..Default::default()},
        Tile { label: "Road Used EW".to_string(), ..Default::default()},*/
                Tile { label: "Block".to_string(), ..Default::default()},
                Tile { label: "Warehouse".to_string(), ..Default::default()},
                Tile { label: VAN_LABEL.to_string(), ..Default::default()},

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
    pub fn get_colors() -> JsValue {
        JsValue::from_serde( &build_color_list()).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_tiles() -> JsValue {
        JsValue::from_serde(&build_tile_list()).unwrap()
    }


#[wasm_bindgen()]
impl Universe {
    pub fn calculate(&self) {

        let mut queue = Vec::new();

        queue.push(self.data.clone());

        while let Some(next_state) = queue.pop() {

            //find all the cars
            for cell_option in next_state.cells.iter() {

                if let Some(cell) = cell_option {

                    if cell.tile.label != VAN_LABEL {
                        continue;
                    }

                    log!("Found a car @ {:?}", cell.tile);
                }
            }
        }
    }
}


#[wasm_bindgen()]
impl Universe {
    // ...

    pub fn new(h: usize, w: usize) -> Universe {

        log!(
                    "Building a new Grid.  [{}, {}] ",
                    w,
                    h
                );

        utils::set_panic_hook();

        let width = w;
        let height = h;


        let cells = (0..width * height)
            .map(|_i| {
                None
            })
            .collect();

        Universe {
            data: UniverseData { width,
            height,
            cells }
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }



    pub fn get_data(&self) -> JsValue {
        JsValue::from_serde(&self.data).unwrap()
    }



    pub fn set_square(&mut self, row_index: usize, col_index: usize, color_val: &JsValue, tile_val: &JsValue ) {

        let color: Option<Color> = color_val.into_serde().unwrap();
        let tile: Option<Tile> = tile_val.into_serde().unwrap();


        let idx: usize = row_index * self.data.width + col_index;

        log!(
                    "Received Row/Col [{}, {}] = idx [{}].  Color {:?} Tile {:?}",
                    row_index,
                    col_index,
            idx,
            color,
            tile
                );


        self.data.cells[idx] = if let Some(tile) = tile  {
            Some(CellData {
                row_index,
                col_index,
                color,
                tile,
                used_mask: 0
            })
        } else {
            None
        };




    }
}