use crate::solver::struct_defs::*;

use crate::solver::struct_defs::TileEnum::{TileRoad, TileWarehouse};

//use crate::solver::public_func::build_color_list;

//use crate::solver::utils::VAN_LABEL;

#[derive(Clone, Copy, Debug)]
enum Directions {
    NORTH = 1,
    EAST = 2,
    SOUTH = 4,
    WEST = 8,
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

use crate::solver::struct_defs::Warehouse;
use crate::solver::van::Van;

const ALL_DIRECTIONS: [Directions; 4] = [NORTH, EAST, SOUTH, WEST];


impl TileEnum {
    fn mut_warehouse(&mut self) -> &mut Warehouse {
        match self {
            TileWarehouse(inner) => {
                return inner;
            }
            _ => panic!()
        }
    }
    fn mut_road(&mut self) -> &mut Road {
        match self {
            TileRoad(inner_road) => {
                return inner_road;
            }
            _ => panic!()
        }
    }
    fn road(&self) -> &Road {
        match self {
            TileRoad(inner_road) => {
                return inner_road;
            }
            _ => panic!()
        }
    }
}

impl Universe {
    fn get_adjacent_square_indexes(&self, cell_index: usize,
                                   used_dir_mask: u8) -> Vec<(Directions, usize)> {
        let cell_row_index: usize = cell_index / self.data.width;
        let cell_col_index: usize = cell_index % self.data.width;

        ALL_DIRECTIONS.iter().filter_map(|dir| {

            //first check the mask
            if used_dir_mask & *dir as u8 > 0 {
                return None;
            }

            let adj_index: Option<usize> = match dir {
                NORTH => {
                    if cell_row_index == 0 {
                        None
                    } else {
                        Some(cell_index - self.data.width)
                    }
                }
                SOUTH => {
                    if cell_row_index >= self.data.height {
                        None
                    } else {
                        Some(cell_index + self.data.width)
                    }
                }
                EAST => {
                    if cell_col_index >= self.data.width {
                        None
                    } else {
                        Some(cell_index + 1)
                    }
                }
                WEST => {
                    if cell_col_index == 0 {
                        None
                    } else {
                        Some(cell_index - 1)
                    }
                }
            };

            if let Some(adj_index) = adj_index {
                Some((*dir, adj_index))
            } else {
                None
            }
        }).collect()
    }

    ///
    /// Returns the color_index
    fn empty_warehouse_color(&self, cur_row_index: usize, cur_col_index: usize, cur_state: &UniverseData) -> Option<usize> {
        if cur_row_index == 0 {
            return None;
        }

        let north_tile = &cur_state.cells[(cur_row_index - 1) * self.data.width + cur_col_index].tile;
        if let TileWarehouse(Warehouse { color: warehouse_color, is_filled }) = north_tile {
            if *is_filled {
                return None;
            } else {
                return Some(warehouse_color.color_index);
            }
        }

        None
    }

    pub(crate) fn initial_van_list(&self) -> Vec<Van> {


        self.data.cells.iter().enumerate().filter_map(|(cell_index, cell)| {
            let cell_index_check = cell.row_index * self.data.width + cell.col_index;

            assert_eq!(cell_index, cell_index_check);

            if let TileRoad(road) = &cell.tile {
                if let Some(van) = &road.van {

                    //found a van
                    let mut m_van = van.clone();
                    m_van.tick = 0;
                    m_van.is_done = false;
                    m_van.cell_index = cell_index;
                    Some(m_van)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }


    pub(crate) fn process_queue_item(&mut self) -> Option<UniverseData> {
        'main_queue_loop:
            while let Some(mut cur_state) = self.queue.pop_front() {

            //Have we seen this node yet?  Probably not needed
            /*if self.seen.contains(&cur_state) {
                log!("Already seen");
                continue;
            }

            self.seen.insert(cur_state.clone());*/

            //change current_van_index in one place
            if cur_state.current_van_index == cur_state.vans.len() - 1 {
                log_trace!("Advancing a tick {} => {}", cur_state.tick, cur_state.tick + 1);
                //let mut next_state = cur_state.clone();
                cur_state.tick += 1;
                cur_state.current_van_index = 0;
            } else {
                cur_state.current_van_index += 1;
            }

            self.iter_count += 1;

            log_trace!("\n\nLoop count: {}  Queue Length: {}", self.iter_count, self.queue.len());


            //check success, where all warehouses are filled
            if cur_state.cells.iter().all(|cell| {
                match cell.tile {
                    TileWarehouse(Warehouse { is_filled, .. }) => {
                        is_filled
                    }
                    _ => true
                }
            }) {
                log!("Success!");
                return Some(cur_state);
            }


            if cur_state.vans[cur_state.current_van_index].is_done {
                log_trace!("Van #{}: {:?} is done, skipping", cur_state.current_van_index,
                cur_state.vans[cur_state.current_van_index]);
                continue;
            }

            let van_cell_index = cur_state.vans[cur_state.current_van_index].cell_index;

            let (cur_row_index, cur_col_index) = {
                let c = &cur_state.cells[van_cell_index];

                (c.row_index, c.col_index)
            };

            {
                let cur_road = cur_state.cells[van_cell_index].tile.mut_road();

                log_trace!("Processing van at {}, {}.  Van: {:?}",
                     cur_row_index, cur_col_index,
                     cur_road.van);

                //pick up a block if it exists.  Note a van can pick up a box of any color
                if let Some(block) = &cur_road.block {
                    log_trace!("Rolled on a block of color {:?}", block);
                    if let Some(i) = cur_state.vans[cur_state.current_van_index].get_empty_slot() {
                        log_trace!("Rolled on a block of color {:?}", block);
                        cur_state.vans[cur_state.current_van_index].boxes[i] = Some(block.clone());
                        cur_road.block = None;
                    }
                }
            }

            //check if we can drop a block off
            if let Some(warehouse_color_index) = self.empty_warehouse_color(cur_row_index, cur_col_index, &cur_state) {
                let mut able_to_drop_off = false;


                //drop off block at warehouse
                let top_block_color_index =
                    {
                        let cur_road = cur_state.cells[van_cell_index].tile.road();
                        if let Some(color) = cur_road.van.as_ref().unwrap().get_top_box() {
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
                        cur_state.cells[van_cell_index - self.data.width].tile.mut_warehouse().is_filled = true;
                    }


                    //test what happens if we stop
                    let mut if_van_stops_state = cur_state.clone();
                    if_van_stops_state.cells[van_cell_index].tile.mut_road().van.as_mut().unwrap().is_done = true;
                    self.queue.push_back(if_van_stops_state);

                    able_to_drop_off = true;
                }


                //we failed
                if !able_to_drop_off {
                    continue 'main_queue_loop;
                }
            }


            let cur_road = cur_state.cells[van_cell_index].tile.road();

            //now attempt to move

            //Where could we move?  (looks at mask & grid)
            let adj_square_indexes = self.get_adjacent_square_indexes(
                van_cell_index, cur_road.used_mask);

            log_trace!("Adj squares: {:?}", adj_square_indexes);
            let mut any_moved = false;

            for adj_square_index in adj_square_indexes.iter().enumerate().filter_map(
                |(adj_square_index, &(_dir, adj_cell_index))| {
                    if let TileRoad(..) = &cur_state.cells[adj_cell_index].tile {

                        //Check each van that has already moved.  The ones that have yet to move don't need to be checked
                        if cur_state.current_van_index > 0 &&
                            cur_state.vans.iter().take(cur_state.current_van_index-1).any(
                                |other_van| adj_cell_index == other_van.cell_index)
                        {
                            None
                        } else {
                            //no van
                            Some(adj_square_index)
                        }
                    } else {
                        //not a road
                        None
                    }
                }) {

                //now we have checked it is a road without a van in it, the mask is ok, etc.

                log_trace!("Moving to actual road {:?}", adj_square_index);

                //make the move
                let mut next_state = cur_state.clone();

                let adj_info = &adj_square_indexes[adj_square_index];

                //remove van & set used mask
                next_state.cells[van_cell_index].tile.mut_road().van = None;
                next_state.cells[van_cell_index].tile.mut_road().used_mask |= adj_info.0 as u8;


                //add van to next square

                let moving_to_cell_index =adj_info.1;

                {
                    let van = &mut next_state.vans[next_state.current_van_index];
                    van.cell_index = moving_to_cell_index;
                    van.tick += 1;

                    //keep a history
                    next_state.cells[moving_to_cell_index].tile.mut_road().van = Some(van.clone());
                }

                //we cant do a U turn
                next_state.cells[moving_to_cell_index].tile.mut_road().used_mask |= adj_info.0.opposite() as u8;

                self.queue.push_back(next_state);
                any_moved = true;
            }

            //we are stuck, nothing else will be queued at this point
            if !any_moved {
                continue;
            }

            return Some(cur_state);

        }

        return None;
    }
}
