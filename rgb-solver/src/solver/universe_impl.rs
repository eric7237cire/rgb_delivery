use wasm_bindgen::prelude::*;
use crate::solver::struct_defs::*;
use super::utils;
use crate::solver::struct_defs::TileEnum::{Road, Warehouse,Empty};
use std::collections::HashSet;
use std::collections::vec_deque::VecDeque;
//use crate::solver::public_func::build_color_list;

//use crate::solver::utils::VAN_LABEL;

enum Directions {
    NORTH = 0,
    EAST = 1,
    SOUTH = 2,
    WEST = 4

}

//use Directions::*;
use crate::solver::universe_impl::Directions::*;
use crate::solver::struct_defs;

const ALL_DIRECTIONS : [Directions;4] = [NORTH, EAST, SOUTH, WEST];

impl Van {
    fn get_top_box(&self) -> &Option<Color> {
        for i in (0..2).rev() {
            if !self.boxes[i].is_none() {
                return &self.boxes[i];
            }
        }
        return &None;
    }

    fn clear_top_box(&mut self) {
        for i in (0..2).rev() {
            if !self.boxes[i].is_none() {
                self.boxes[i] = None;
            }
        }
    }

    fn get_empty_slot(&self) -> Option<usize> {
        for i in (0..2).rev() {
            if self.boxes[i].is_none() {
                return Some(i);
            }
        }
        return None;
    }
}


impl Universe {
    fn private_calculate(&self) -> Option<UniverseData> {

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        let mut iter_count = 0;

        queue.push_back(self.data.clone());

        while let Some(cur_state) = queue.pop_front() {

            //Have we seen this node yet?
            if seen.contains(&cur_state) {
                continue;
            }

            seen.insert(cur_state.clone());

            iter_count += 1;

            log!("Loop count: {}  Queue Length: {}", iter_count, queue.len());

            let mut next_state = cur_state.clone();

            //find all the cars
            for cell in cur_state.cells.iter() {

                let cell_index = cell.row_index * self.data.width + cell.col_index;

                match &cell.tile {
                    Road{
                        van: Some(van),
                        block: block_opt,
                        used_mask
                    }
                    => {

                        log!("Found a van at {}, {}.  Van: {:?}", cell.row_index, cell.col_index, van);

                        if van.is_done {
                            continue;
                        }

                        //pick up a block if it exists
                        if let Some(block) = block_opt {
                            if let Some(i) = van.get_empty_slot() {
                                match &mut next_state.cells[cell_index].tile {
                                    Road{ van: Some(next_van) ,..} => {
                                        next_van.boxes[i] = Some(block.clone());
                                    },
                                    _ => panic!()
                                }
                            }
                        }


                        if cell.row_index > 0 {
                            let north_tile = &cur_state.cells[ (cell.row_index-1) * self.data.width + cell.col_index ].tile;
                            if let Warehouse {color: warehouse_color, is_filled} = north_tile {
                                if !is_filled {

                                     //drop off block at warehouse
                                    if let Some(top_block) = van.get_top_box() {
                                        if top_block == warehouse_color {
                                            match &mut next_state.cells[cell_index].tile {
                                                Road { van: Some(next_van), .. } => {
                                                    next_van.clear_top_box();
                                                },
                                                _ => panic!()
                                            }
                                        }
                                    }
                                }
                            }
                        }




                    },
                    _ => {}
                }

            }

            if iter_count > 3 {
                return Some(cur_state);
            }
        }

        return None;
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

    pub fn calculate(&self) -> JsValue {
        let v = self.private_calculate();
        JsValue::from_serde(&v).unwrap()
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