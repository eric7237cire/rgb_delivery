use wasm_bindgen::prelude::*;
use crate::solver::struct_defs::*;
use super::utils;
use crate::solver::struct_defs::TileEnum::{Road, Warehouse,Empty};
//use crate::solver::public_func::build_color_list;

//use crate::solver::utils::VAN_LABEL;

/*
#[wasm_bindgen()]
impl Universe {
    pub fn calculate(&self) {

        let mut queue = Vec::new();

        queue.push(self.data.clone());

        while let Some(next_state) = queue.pop() {

            //find all the cars
            for cell_option in next_state.cells.iter() {


            }
        }
    }
}
*/

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


        let cells: Vec<CellData> = (0..width * height)
            .map(|idx| {
                CellData{row_index: idx / width, col_index: idx % width,
                     ..Default::default()}
            })
            .collect();

        //let cl = build_color_list();

        //cells[0] =  CellData{row_index: 0, col_index: 0, tile: Road {used_mask: 45, _box: None}, van: None});
        /*cells[1] =  CellData{row_index: 0, col_index: 0, tile: Warehouse {color: cl[2].clone(), is_filled: true},
            van: Some( Van{ boxes: [None, Some(cl[0].clone()), Some(cl[3].clone())] } ) } );*/

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



    pub fn set_square(&mut self, tile_val: &JsValue ) {

        let tile: CellData = tile_val.into_serde().unwrap();


        let idx: usize = tile.row_index * self.data.width + tile.col_index;

        log!(
                    "Received Row/Col [{}, {}] = idx [{}].  Tile {:?}",
                    tile.row_index,
                    tile.col_index,
            idx,

            tile
                );

        if idx < self.data.cells.len() {
            self.data.cells[idx] = tile;
        } else {
            log!(
                    "Out of bounds, ignoring"
                );
        }
    }
}