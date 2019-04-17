use wasm_bindgen::prelude::*;
use crate::solver::struct_defs::*;
use super::utils;
use crate::solver::struct_defs::TileEnum::{TileRoad, TileWarehouse,Empty};
use std::collections::HashSet;
use std::collections::vec_deque::VecDeque;
//use crate::solver::public_func::build_color_list;

//use crate::solver::utils::VAN_LABEL;

#[derive(Clone, Copy, Debug)]
enum Directions {
    NORTH = 1,
    EAST = 2,
    SOUTH = 4,
    WEST = 8

}

impl Directions {
    fn opposite(&self) -> Directions {
        match self {
            NORTH => SOUTH,
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST
        }
    }
}

//use Directions::*;
use crate::solver::universe_impl::Directions::*;
use crate::solver::struct_defs;
use crate::solver::struct_defs::Warehouse;

const ALL_DIRECTIONS : [Directions;4] = [NORTH, EAST, SOUTH, WEST];

impl Van {
    fn get_top_box(&self) -> &Option<Color> {
        for i in (0..=2).rev() {
            if !self.boxes[i].is_none() {
                return &self.boxes[i];
            }
        }
        return &None;
    }

    fn clear_top_box(&mut self) {
        for i in (0..=2).rev() {
            if !self.boxes[i].is_none() {
                self.boxes[i] = None;
            }
        }
    }

    fn get_empty_slot(&self) -> Option<usize> {
        for i in (0..=2).rev() {
            if self.boxes[i].is_none() {
                return Some(i);
            }
        }
        return None;
    }
}
impl TileEnum {

    fn mut_warehouse(&mut self) -> &mut Warehouse {
        match self {
            TileWarehouse(inner) => {
                return inner;
            },
            _ => panic!()
        }

    }
    fn mut_road(&mut self) -> &mut Road {
        match self {
            TileRoad(inner_road) => {
                return inner_road;
            },
            _ => panic!()
        }

    }
    fn road(&self) -> &Road {
        match self {
            TileRoad(inner_road) => {
                return inner_road;
            },
            _ => panic!()
        }

    }
}

impl Universe {

    fn get_adjacent_square_indexes(&self, cell_index: usize,
                                   used_dir_mask: u8) -> Vec<(Directions,usize)> {

        let cell_row_index: usize = cell_index / self.data.width;
        let cell_col_index : usize = cell_index % self.data.width;

        ALL_DIRECTIONS.iter().filter_map( |dir| {

            //first check the mask
            if used_dir_mask & *dir as u8 > 0 {
                return None;
            }

            let adj_index : Option<usize> = match dir {
                NORTH => {
                    if cell_row_index == 0 {
                        None
                    } else {
                        Some( cell_index - self.data.width)
                    }
                },
                SOUTH => {
                    if cell_row_index >= self.data.height {
                        None
                    } else {
                        Some(cell_index + self.data.width)
                    }
                },
                EAST => {
                    if cell_col_index >= self.data.width {
                        None
                    } else {
                        Some(cell_index + 1)
                    }
                },
                WEST => {
                    if cell_col_index == 0 {
                        None
                    } else {
                        Some(cell_index - 1)
                    }
                }
            };

            if let Some( adj_index )= adj_index {
                Some( (*dir, adj_index) )
            } else {
                None
            }

        }).collect()
    }

    ///
    /// Returns the color_index
    fn empty_warehouse_color(&self, cur_row_index:usize, cur_col_index: usize, cur_state: &UniverseData) -> Option<usize> {
        if cur_row_index ==0 {
            return None ;
        }

        let north_tile = &cur_state.cells[ (cur_row_index-1) * self.data.width + cur_col_index ].tile;
        if let TileWarehouse( Warehouse {color: warehouse_color, is_filled} ) = north_tile {
            if *is_filled {
                return None;
            } else {
                return Some(warehouse_color.color_index);
            }
        }

        None

    }

    fn private_calculate(&self) -> Option<UniverseData> {

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        let mut iter_count = 0;

        queue.push_back(self.data.clone());

        'main_queue_loop:
        while let Some(mut cur_state) = queue.pop_front() {

            //Have we seen this node yet?
            if seen.contains(&cur_state) {
                continue;
            }

            seen.insert(cur_state.clone());

            iter_count += 1;

            log!("Loop count: {}  Queue Length: {}", iter_count, queue.len());

            //check success, where all warehouses are filled
            if cur_state.cells.iter().all( |cell| {
               match cell.tile {
                   TileWarehouse(Warehouse {is_filled, ..}) => {
                       is_filled
                   },
                   _ =>  true
               }
            }) {
                log!("Success!");
                return Some(cur_state);
            }

            let mut any_moved = false;

            //find all the car indices
            let van_cell_indices = cur_state.cells.iter().enumerate().filter_map(| (cell_index,cell)| {

                let cell_index_check = cell.row_index * self.data.width + cell.col_index;

                assert_eq!(cell_index, cell_index_check);

                if let TileRoad(road) = &cell.tile {
                    if let Some(van) = &road.van {

                        if van.is_done {
                            None
                        } else if van.tick > cur_state.tick {
                            None
                        } else {
                            Some(cell_index)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }).collect::<Vec<_>>();

            for van_cell_index in van_cell_indices {






                //let van = cur_road.van.as_ref().unwrap();

                {
                    let cur_cell = &cur_state.cells[van_cell_index];
                    log!("Found a van at {}, {}.  Van: {:?}", cur_cell.row_index, cur_cell.col_index, cur_cell.tile.road().van);
                }

                let ( cur_row_index, cur_col_index) = {
                        let c = &cur_state.cells[van_cell_index];

                        (c.row_index, c.col_index)
                    };

                {
                    let cur_road = cur_state.cells[van_cell_index].tile.mut_road();

                    //pick up a block if it exists
                    if let Some(block) = &cur_road.block {
                        log!("Rolled on a block of color {:?}", block);
                        if let Some(i) = cur_road.van.as_ref().unwrap().get_empty_slot() {
                            log!("Rolled on a block of color {:?}", block);
                            cur_road.van.as_mut().unwrap().boxes[i] = Some(block.clone());
                            cur_road.block = None;
                        }
                    }
                }

                //check if we can drop a block off
                if let Some(warehouse_color_index) = self.empty_warehouse_color(cur_row_index, cur_col_index,&cur_state)  {

                    let mut able_to_drop_off = false;


                    //drop off block at warehouse
                    let top_block_color_index =
                        {
                            let cur_road = cur_state.cells[van_cell_index].tile.road();
                            if let Some( color) = cur_road.van.as_ref().unwrap().get_top_box() {

                                color.color_index
                            } else {
                                100
                            }
                        };

                    if top_block_color_index == warehouse_color_index {

                        //pop the box
                        {
                            let cur_road = cur_state.cells[van_cell_index].tile.mut_road();
                            cur_road.van.as_mut().unwrap().clear_top_box();
                        }
                        //set warehouse to filled
                        {
                            cur_state.cells[van_cell_index - self.data.width ].tile.mut_warehouse().is_filled = true;
                        }


                        //test what happens if we stop
                        let mut if_van_stops_state = cur_state.clone();
                        if_van_stops_state.cells[van_cell_index].tile.mut_road().van.as_mut().unwrap().is_done = true;
                        queue.push_back(if_van_stops_state);

                        able_to_drop_off = true;
                    }


                    //we failed
                    if !able_to_drop_off {
                        continue 'main_queue_loop;
                    }

                }


                let cur_road = cur_state.cells[van_cell_index].tile.road();

                //now attempt to move
                let adj_square_indexes = self.get_adjacent_square_indexes(
                    van_cell_index, cur_road.used_mask);

                log!("Adj squares: {:?}", adj_square_indexes);

                for adj_square_index in adj_square_indexes.iter().enumerate().filter_map(
                    | (adj_square_index,& (dir,adj_cell_index)) | {
                    if let TileRoad(road) = &cur_state.cells[adj_cell_index].tile {

                        if road.van.is_none() {
                            Some( adj_square_index )
                        } else {
                            None
                        }
                    } else {
                        //not a road
                        None
                    }
                }) {

                    //now we have checked it is a road without a van in it, the mask is ok, etc.

                    //make the move
                    let mut next_state = cur_state.clone();

                    let adj_info = &adj_square_indexes[adj_square_index];

                    //remove van & set used mask
                    next_state.cells[van_cell_index].tile.mut_road().van = None;
                    next_state.cells[van_cell_index].tile.mut_road().used_mask |= adj_info.0 as  u8;


                    //add van to next square
                    next_state.cells[adj_info.1].tile.mut_road().van = cur_state.cells[van_cell_index].tile.road().van.clone();
                    next_state.cells[adj_info.1].tile.mut_road().van.as_mut().unwrap().tick += 1;
                    //we cant do a U turn
                    next_state.cells[adj_info.1].tile.mut_road().used_mask |= adj_info.0.opposite() as u8;


                    queue.push_back(next_state);
                    any_moved = true;
                }





            }


            //if nothing has moved, lets move  time forward
            if !any_moved {
                let mut next_state = cur_state.clone();
                next_state.tick += 1;
                queue.push_back(next_state);
            }

            if iter_count > 600 {
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
/*
        cells[0] =  CellData{row_index: 0, col_index: 0, tile: TileRoad(
            Road {used_mask: 45, block: None, van: None})
        };*/
        /*cells[1] =  CellData{row_index: 0, col_index: 0, tile: Warehouse {color: cl[2].clone(), is_filled: true},
            van: Some( Van{ boxes: [None, Some(cl[0].clone()), Some(cl[3].clone())] } ) } );*/

        Universe {
            data: UniverseData { width,
            height,
            cells,  ..Default::default() }
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