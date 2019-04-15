use wasm_bindgen::prelude::*;
use crate::solver::struct_defs::*;
use super::utils;
use crate::solver::utils::VAN_LABEL;

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