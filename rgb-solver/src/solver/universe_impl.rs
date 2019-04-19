use crate::solver::struct_defs::*;

use crate::solver::struct_defs::TileEnum::{TileRoad, TileWarehouse, TileBridge};

//use crate::solver::public_func::build_color_list;

//use crate::solver::utils::VAN_LABEL;



impl Directions {
    pub(crate) fn opposite(&self) -> Directions {
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

pub (crate) const ALL_DIRECTIONS: [Directions; 4] = [NORTH, EAST, SOUTH, WEST];


impl TileEnum {
    pub(crate) fn mut_warehouse(&mut self) -> &mut Warehouse {
        match self {
            TileWarehouse(inner) => {
                return inner;
            }
            _ => panic!()
        }
    }
}


impl Universe {

    /// Gets adj indexes, checking grid limits
    fn get_adjacent_square_indexes(&self, cell_index: CellIndex,
                                   used_dir_mask: u8) -> Vec<AdjSquareInfo> {
        let cell_row_index: usize = cell_index.0 / self.data.width;
        let cell_col_index: usize = cell_index.0 % self.data.width;

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
                        Some(cell_index.0 - self.data.width)
                    }
                }
                SOUTH => {
                    if cell_row_index >= self.data.height-1 {
                        None
                    } else {
                        Some(cell_index.0 + self.data.width)
                    }
                }
                EAST => {
                    if cell_col_index >= self.data.width-1 {
                        None
                    } else {
                        Some(cell_index.0 + 1)
                    }
                }
                WEST => {
                    if cell_col_index == 0 {
                        None
                    } else {
                        Some(cell_index.0 - 1)
                    }
                }
            };

            if let Some(adj_index) = adj_index {
                Some(AdjSquareInfo{direction:*dir, cell_index: adj_index.into(), direction_index: dir_index})
            } else {
                None
            }
        }).collect()
    }

    /// If we provided a choice for the row/col and perhaps tick (as a van can go through the
    /// same cell 2x
    fn get_fixed_choice(&self, cur_state: &GridState) -> Option<ChoiceOverride> {

        let (cur_row_index, cur_col_index) = cur_state.current_cell_index().to_row_col(cur_state.width);

        let o = self.choice_override_list.iter().find( |co| {

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

        if let Some(o) = o {
            Some(o.clone())
        } else {
            None
        }
    }


    pub(crate) fn initial_van_list(&self) -> Vec<Van> {


        self.data.tiles.iter().enumerate().filter_map(|(cell_index, tile)| {
           
            if let TileRoad(road) = &tile {
                if let Some(van) = &road.van_snapshot {

                    //found a van
                    let mut m_van = van.clone();
                    m_van.tick = 0;
                    m_van.is_done = false;
                    m_van.cell_index = cell_index.into();
                    Some(m_van)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }

    pub(crate) fn initial_bridge_list(&self) -> Vec<Bridge> {
       self.data.tiles.iter().enumerate().filter_map(|(cell_index, tile)| {

            if let TileBridge(bridge) = &tile {
                let mut m_bridge = bridge.clone();
                m_bridge.cell_index = cell_index.into();
                m_bridge.used_mask = 0;
                Some(m_bridge)
            } else {
                None
            }
        }).collect()
    }

    pub(crate) fn initial_button_list(&self) -> Vec<Button> {
       self.data.tiles.iter().enumerate().filter_map(|(cell_index, tile)| {

            if let TileRoad(road) = &tile {
                if let Some(button) = &road.button_snapshot {

                    //found a van
                    let mut m_button = button.clone();
                    m_button.cell_index = cell_index.into();
                    m_button.was_pressed_this_tick = false;
                    Some(m_button)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }


    pub(crate) fn process_queue_item(&mut self) -> Option<&GridState> {

        if self.success.is_some() {
            return self.success.as_ref();
        }
        while let Some(mut cur_state) = self.queue.pop_front() {

            //self.current_calc_state = Some(cur_state.clone());

            //check success, where all warehouses are filled
            if cur_state.check_success() {
                log!("Success!");
                self.success = Some(cur_state);
                return self.success.as_ref();
            }

            //change current_van_index in one place
            let did_tick_advance = match cur_state.increment_current_van_index() {
                Err(_) => continue,
                Ok(b) => b
            };

            cur_state.check_bridges_and_buttons();

            if did_tick_advance {
                match cur_state.toggle_bridges_and_buttons() {
                    Err(_) => {
                        log_trace!("Van caught on open bridge");
                        continue;
                    },
                    Ok(_) => {}
                };
                cur_state.check_bridges_and_buttons();
            } else {
                log_trace!("Tick did not advance");
            }

            self.iter_count += 1;

            log_trace!("\n\nLoop count: {} Tick: {} Queue Length: {} Cur van index: {:?}  Row/Col: {:?}",
                self.iter_count,
                self.data.tick,
                self.queue.len(), cur_state.current_van_index,
                cur_state.vans[cur_state.current_van_index.0].cell_index.to_row_col(cur_state.width)
            );

            if self.iter_count % 500 == 0 {
                 log!("\n\nLoop count: {}  Queue Length: {} Cur van index: {:?}", self.iter_count, self.queue.len(), cur_state.current_van_index);
            }

            if self.iter_count > 300000 {
                self.queue.clear();
                break;
            }

            let van_cell_index = cur_state.vans[cur_state.current_van_index.0].cell_index;

            //let (cur_row_index, cur_col_index) = van_cell_index.to_row_col(self.data.width);

            cur_state.pick_up_block_if_exists();

            //cur_state.press_button_if_exists();

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

                         {
                            if let TileRoad(road) = &if_van_stops_state.tiles[if_van_stops_state.vans[if_van_stops_state.current_van_index.0].cell_index.0] {
                                assert!(road.van_snapshot.is_some());
                            }
                        }
                        self.queue.push_back(if_van_stops_state);
                    },
                    _ => ()
                };
            }


            let cur_used_mask = cur_state.get_cur_used_mask();

            log_trace!("Current used mask: {:#07b}", cur_used_mask);

            //now attempt to move 

            //Where could we move?  (looks at mask & grid)
            let adj_square_indexes = self.get_adjacent_square_indexes(
                van_cell_index, cur_used_mask);

            log_trace!("Adj squares: {:?}", adj_square_indexes);
            let mut any_moved = false;

            let fixed_choice_opt = self.get_fixed_choice(&cur_state);

            let adj_info_filtered_list : Vec<&AdjSquareInfo> = adj_square_indexes.iter().filter_map(
                |a_info| cur_state.filter_map_by_can_have_van(&fixed_choice_opt, a_info)).collect();

            log_trace!("Adj squares info list: {:?}", adj_info_filtered_list);

            for adj_info in adj_info_filtered_list.into_iter() {

                //now we have checked it is a road without a van in it, the mask is ok, etc.

                //make the move
                let mut next_state = cur_state.clone();

                next_state.handle_move(van_cell_index, adj_info);

                {
                    for vi in 0..next_state.vans.len() {
                        if let TileRoad(road) = &next_state.tiles[next_state.vans[vi].cell_index.0] {
                            assert!(road.van_snapshot.is_some(), "Hey bad vi {} row/col: {:?}", vi, next_state.vans[vi].cell_index.to_row_col(next_state.width));
                        }
                    }
                }

                next_state.press_button_if_exists();

                self.queue.push_back(next_state);
                any_moved = true;
            }

            //we are stuck, nothing else will be queued at this point
            if !any_moved {
                 log_trace!("NO MOVES  Van: {:?}",                     
                     cur_state.current_van_index);
                continue;
            }


            if let Some(f) = self.queue.front() {
                return Some(f);
            } else {
                return None;
            }

        }

        return None;
    }
}
