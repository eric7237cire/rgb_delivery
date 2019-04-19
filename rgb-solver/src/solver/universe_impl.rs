use crate::solver::struct_defs::*;

use crate::solver::struct_defs::TileEnum::{TileRoad, TileWarehouse};

//use crate::solver::public_func::build_color_list;

//use crate::solver::utils::VAN_LABEL;



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


use crate::solver::struct_defs::Directions::*;

use crate::solver::struct_defs::Warehouse;
use crate::solver::van::Van;
use crate::solver::grid_state::{GridState, CanDropOff};

const ALL_DIRECTIONS: [Directions; 4] = [NORTH, EAST, SOUTH, WEST];


impl TileEnum {
    pub(crate) fn mut_warehouse(&mut self) -> &mut Warehouse {
        match self {
            TileWarehouse(inner) => {
                return inner;
            }
            _ => panic!()
        }
    }
    pub(crate) fn mut_road(&mut self) -> &mut Road {
        match self {
            TileRoad(inner_road) => {
                return inner_road;
            }
            _ => panic!()
        }
    }
    pub(crate) fn road(&self) -> &Road {
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
                                   used_dir_mask: u8) -> Vec<AdjSquareInfo> {
        let cell_row_index: usize = cell_index / self.data.width;
        let cell_col_index: usize = cell_index % self.data.width;

        ALL_DIRECTIONS.iter().enumerate().filter_map(|(dir_index,dir)| {

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
                    if cell_row_index >= self.data.height-1 {
                        None
                    } else {
                        Some(cell_index + self.data.width)
                    }
                }
                EAST => {
                    if cell_col_index >= self.data.width-1 {
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
                Some(AdjSquareInfo{direction:*dir, cell_index: adj_index, direction_index: dir_index})
            } else {
                None
            }
        }).collect()
    }



    pub(crate) fn initial_van_list(&self) -> Vec<Van> {


        self.data.tiles.iter().enumerate().filter_map(|(cell_index, tile)| {
           
            if let TileRoad(road) = &tile {
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


    pub(crate) fn process_queue_item(&mut self) -> &Option<GridState> {

        if self.success.is_some() {
            return &self.success;
        }
        while let Some(mut cur_state) = self.queue.pop_front() {

            self.current_calc_state = Some(cur_state.clone());

            //check success, where all warehouses are filled
            if cur_state.check_success() {
                log!("Success!");
                self.success = Some(cur_state);
                return &self.success;
            }

            //change current_van_index in one place
            match cur_state.increment_current_van_index() {
                Err(_) => continue,
                _ => ()
            };

            self.iter_count += 1;

            log_trace!("\n\nLoop count: {}  Queue Length: {} Cur van index: {:?}", self.iter_count, self.queue.len(), cur_state.current_van_index);

            if self.iter_count % 500 == 0 {
                 log!("\n\nLoop count: {}  Queue Length: {} Cur van index: {:?}", self.iter_count, self.queue.len(), cur_state.current_van_index);
            }

            if self.iter_count > 300000 {
                self.queue.clear();
                break;
            }

            let van_cell_index = cur_state.vans[cur_state.current_van_index.0].cell_index;

            let (cur_row_index, cur_col_index) = ( van_cell_index / self.data.width, van_cell_index % self.data.width );

            cur_state.pick_up_block_if_exists();

            //check if we can drop a block off
            if cur_state.empty_warehouse_color().is_some() {
                match cur_state.can_drop_off_block() {
                    Err(_) => continue,
                    Ok(CanDropOff::YesOK) => {

                        assert!(cur_state.empty_warehouse_color().is_none());
                        assert!(cur_state.warehouses_remaining > 0);

                        cur_state.warehouses_remaining -= 1;

                        //test what happens if we stop
                        let mut if_van_stops_state = cur_state.clone();
                        if_van_stops_state.current_van_mut().is_done = true;
                        self.queue.push_back(if_van_stops_state);
                    },
                    _ => ()
                };
            }

            let cur_road = cur_state.tiles[van_cell_index].road();

            //now attempt to move 

            //Where could we move?  (looks at mask & grid)
            let adj_square_indexes = self.get_adjacent_square_indexes(
                van_cell_index, cur_road.used_mask);

            log_trace!("Adj squares: {:?}", adj_square_indexes);
            let mut any_moved = false;

            let fixed_choice_opt = self.choice_override_list.iter().find( |co| {

                if let Some(forced_tick) = co.tick {
                    if forced_tick != cur_state.tick {
                        return false;
                    }
                }

                co.van_index == cur_state.current_van_index
                    && cur_row_index == co.row_index
                    && cur_col_index == co.col_index
                }
            );

            for adj_square_index in adj_square_indexes.iter().enumerate().filter_map(
                |(adj_square_index, &AdjSquareInfo{cell_index: adj_cell_index, direction_index,..})| {

                    if let Some( ChoiceOverride{ direction_index:forced_dir_index, ..}) = fixed_choice_opt {
                        if *forced_dir_index != direction_index {
                             log_trace!("Not in the forced direction {:?}", direction);
                            return None;
                        }
                    }

                    if let TileRoad(..) = &cur_state.tiles[adj_cell_index] {

                        //Check each van that has already moved.  The ones that have yet to move don't need to be checked
                        if cur_state.current_van_index.0 > 0 &&
                            cur_state.vans.iter().take(cur_state.current_van_index.0-1).any(
                                |other_van| adj_cell_index == other_van.cell_index)
                        {
                            log_trace!("Another van is there {:?}", direction);
                            None
                        } else {
                            //no van
                            Some(adj_square_index)
                        }
                    } else {
                        log_trace!("Rejecting direction {:?}, not a road", direction);
                        None
                    }
                }) {



                //now we have checked it is a road without a van in it, the mask is ok, etc.



                //make the move
                let mut next_state = cur_state.clone();

                let adj_info = &adj_square_indexes[adj_square_index];

                log_trace!("Moving to actual road {:?}", adj_info);

                //remove van & set used mask
                {
                    let current_tile_road = next_state.tiles[van_cell_index].mut_road();
                    current_tile_road.van = None;
                    current_tile_road.used_mask |= adj_info.direction as u8;
                    current_tile_road.used_tick[adj_info.direction_index] = Some(cur_state.tick);
                    current_tile_road.used_van_index[adj_info.direction_index] = Some(cur_state.current_van_index);

                }


                //add van to next square

                let moving_to_cell_index =adj_info.cell_index;

                {
                        let van = next_state.current_van_mut();
                    van.cell_index = moving_to_cell_index;
                    van.tick += 1;
                    
                }
                {
                    //keep a history
                    let next_road =next_state.tiles[moving_to_cell_index].mut_road();
                    next_road.van = Some(next_state.vans[next_state.current_van_index.0].clone());

                    //we cant do a U turn
                    next_road.used_mask |= adj_info.direction.opposite() as u8;

                    let opp_dir_index = ALL_DIRECTIONS.iter().position(|d| d == &adj_info.direction.opposite()).unwrap();

                    next_road.used_van_index[opp_dir_index] = Some(cur_state.current_van_index);
                    next_road.used_tick[opp_dir_index] = Some( cur_state.tick );
                }



                self.queue.push_back(next_state);
                any_moved = true;
            }

            //we are stuck, nothing else will be queued at this point
            if !any_moved {
                 log_trace!("NO MOVES {}, {}.  Van: {:?}",
                     cur_row_index, cur_col_index,
                     cur_road.van);
                continue;
            }


            return &self.current_calc_state;

        }

        return &None;
    }
}
