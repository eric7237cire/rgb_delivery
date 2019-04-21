use crate::solver::grid_state::{GridState, GridAnalysis, GridGraph};
use crate::solver::struct_defs::{ChoiceOverride, CellIndex, AdjSquareInfo, Bridge, Button, CellData, VanIndex, TileEnum};
use std::collections::vec_deque::VecDeque;
use crate::solver::misc::{ALL_DIRECTIONS, get_adjacent_index, is_tile_navigable};
use crate::solver::struct_defs::TileEnum::{TileRoad, TileBridge, TileWarehouse};
use crate::solver::van::Van;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use crate::solver::utils;
use crate::solver::utils::set_panic_hook;
use crate::solver::struct_defs::Directions::NORTH;

#[cfg_attr( not(target_arch = "x86_64"), wasm_bindgen())]
#[derive(Default)]
pub struct Universe {
    pub(crate) data: GridState,

    pub(crate) choice_override_list: Vec<ChoiceOverride>,

    //below are used for calculating
    pub(crate) queue: VecDeque<GridState>,

    pub(crate) success: Option<GridState>,

    pub(crate) iter_count: usize,

    pub(crate) analysis: GridAnalysis
}

//private
impl Universe {

    /// Gets adj indexes, checking grid limits
    fn get_adjacent_square_indexes(&self, cell_index: CellIndex,
                                   is_connected_mask: u8) -> Vec<AdjSquareInfo> {
        
        ALL_DIRECTIONS.iter().enumerate().filter_map(|(dir_index,dir)| {

            //first check the mask
            if is_connected_mask & *dir as u8 == 0 {
                return None;
            }

            let adj_index: Option<usize> = get_adjacent_index(cell_index, self.data.height, self.data.width, *dir);

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



    pub(crate) fn initial_graph(&self) -> GridGraph {

        let is_connected_list = self.data.tiles.iter().enumerate().map( |(cur_square_index, tile)| {

            let mut is_connected: u8 = 0;

            if !is_tile_navigable(tile) {
                return 0;
            }

            for (dir_idx,adj_dir) in ALL_DIRECTIONS.iter().enumerate() {

                let adj_square_index: Option<usize> = get_adjacent_index( CellIndex(cur_square_index), self.data.height, self.data.width, *adj_dir);

                if let Some(adj_square_index) = adj_square_index {
                    if is_tile_navigable(&self.data.tiles[adj_square_index])                         
                    {
                        assert_eq!( *adj_dir as u8, 1 << dir_idx);
                        is_connected |= 1 << dir_idx;
                    } else if adj_dir == &NORTH { 
                        if let TileWarehouse(_) = &self.data.tiles[adj_square_index]  {
                            //special case that we want warehouses to be connected to the cell to their south
                            assert_eq!( *adj_dir as u8, 1 << dir_idx);
                            is_connected |= 1 << dir_idx;
                        }
                    }
                }
            }

            is_connected

        }).collect();

        GridGraph { is_connected: is_connected_list }

    }

    pub(crate) fn initial_bridge_list(&self) -> Vec<Bridge> {
       self.data.tiles.iter().enumerate().filter_map(|(cell_index, tile)| {

            if let TileBridge(bridge) = &tile {
                let mut m_bridge = bridge.clone();
                m_bridge.cell_index = cell_index.into();                
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

            self.iter_count += 1;


            //self.current_calc_state = Some(cur_state.clone());

            //check success, where all warehouses are filled
            if cur_state.check_success() {
                log!("Success!");
                self.success = Some(cur_state);
                return self.success.as_ref();
            }

            let save_for_toggle = (cur_state.tick, cur_state.current_van_index);

            //change current_van_index in one place
            let did_tick_advance = match cur_state.increment_current_van_index() {
                Err(_) => continue,
                Ok(b) => b
            };


            log_trace!("\n\nLoop count: {} Tick: {} \
            Queue Length: {} Cur van index: {:?}  Row/Col: {:?}",
                self.iter_count,
                cur_state.tick,
                self.queue.len(), cur_state.current_van_index,
                cur_state.vans[cur_state.current_van_index.0].cell_index.to_row_col(cur_state.width)
            );

            // Also test if starting vans don't move
            if cur_state.tick == 1 {
                assert!(!cur_state.vans[cur_state.current_van_index.0].is_done);

                if cur_state.can_current_van_stop() {
                    let mut if_van_stops_state = cur_state.clone();
                    if_van_stops_state.current_van_mut().is_done = true;
                    //push back to calculate last
                    self.queue.push_back(if_van_stops_state);
                }
            }

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

            if self.iter_count % 10000 == 0 {
                 log!("\n\nLoop count: {} \
                 Queue Length: {} Current Tick: {} ",
                      self.iter_count, self.queue.len(), cur_state.tick);
            }

            if !cur_state.check_graph_validity() {
                log_trace!("Rejecting state");
                continue;
            }


            let van_cell_index = cur_state.vans[cur_state.current_van_index.0].cell_index;

            //let (cur_row_index, cur_col_index) = van_cell_index.to_row_col(self.data.width);

            match cur_state.pick_up_block_if_exists(&self.analysis) {
                Err(_) => continue,
                _ => ()
            };

            //check if we can drop a block off
            match cur_state.handle_warehouse_drop_off() {
                Ok(Some(next_state)) => {
                    self.queue.push_front(next_state);
                },
                Err(_) => continue,
                _ => ()
            };

            match cur_state.handle_block_popper() {
                Ok(Some(mut next_state)) => {
                    //reset to values as when it was just popped
                    next_state.tick = save_for_toggle.0;
                    next_state.current_van_index = save_for_toggle.1;
                    self.queue.push_front(next_state);
                },
                Err(_) => continue,
                _ => ()
            };


            let cur_is_connected_mask = cur_state.get_cur_is_connected_mask();

            log_trace!("Current used mask: {:#07b}", cur_is_connected_mask);

            //now attempt to move

            //Where could we move?  (looks at mask & grid)
            let adj_square_indexes = self.get_adjacent_square_indexes(
                van_cell_index, cur_is_connected_mask);

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

                //checking tile consistency
                {
                    /*
                    Example: tick 123:
                        Van[0] moves A=>B
                        van 2 moves B=>C

                        tick 124
                        van 0 won't have a tile set in B


                    van_snapshot is just used for visualization so its ok
                    */

                    /*for vi in 0..next_state.vans.len() {
                        if let TileRoad(road) = &next_state.tiles[next_state.vans[vi].cell_index.0] {
                            assert!(road.van_snapshot.is_some(), "van index {} row/col: {:?} does not have a van set in its tile", vi,
                                    next_state.vans[vi].cell_index.to_row_col(next_state.width));
                        }
                    }*/
                }

                next_state.press_button_if_exists();

                self.queue.push_front(next_state);
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

        log!("Queue is empty");
        return None;
    }
}

//public

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


        let tiles: Vec<TileEnum> = (0..width * height)
            .map(|_| {
                TileEnum::Empty
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
            data: GridState {
                width,
                height,
                tiles,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    
    pub fn get_data(&self) -> JsValue {
        JsValue::from_serde(&self.data).unwrap()
    }


    pub fn set_overrides(&mut self, choice_override_list: &JsValue) {
        let lo : Vec<ChoiceOverride> = choice_override_list.into_serde().unwrap();

        self.choice_override_list = lo.clone();

        log!("Set override list {:?}", lo);
    }

    pub fn init_calculate(&mut self) {

        set_panic_hook();


        self.queue = VecDeque::new();        

        self.iter_count = 0;
        self.data.tick = 0;

        self.success = None;

        self.data.vans = self.initial_van_list();
        self.data.buttons = self.initial_button_list();
        self.data.bridges = self.initial_bridge_list();
        self.data.graph = self.initial_graph();

        self.data.warehouses_remaining = self.data.tiles.iter().filter( |t| {
            if let TileWarehouse(_) = t {
                true
            } else {
                false
            }
        }).count();

        //we increment on pop, so...
        self.data.current_van_index = VanIndex( self.initial_van_list().len() - 1 );


        //reset road history
        for tile in self.data.tiles.iter_mut() {
            match tile {
                TileRoad(road) => {
                    road.used_van_index = Default::default();
                    road.van_snapshot = None;
                },
                TileBridge(bridge) => {
                    bridge.used_tick = None;
                    bridge.used_van_index = None;
                    bridge.van_snapshot = None;
                },
                _ => {}
            }
        }

        //vans should start on roads
        let van_cells: Vec<CellIndex> = self.data.vans.iter().map( |v| v.cell_index).collect();

        for (van_idx, v_cell_index) in van_cells.iter().enumerate() {
            if let TileRoad(road) = &mut self.data.tiles[v_cell_index.0] {
                road.van_snapshot = Some(self.data.vans[van_idx].clone());
            } else {
                panic!("Van is not on a road");
            }
        }

        self.analysis = GridAnalysis {
            has_poppers: self.data.tiles.iter().any( |t| if let TileRoad(r) = t {
                return r.has_popper;
            } else {false} )
        };

        self.queue.push_back(self.data.clone());


    }

    pub fn next_calculate(&mut self) -> JsValue {

        let iter_count = self.iter_count;
        let q_len = self.queue.len();
        let success = self.success.is_some();

        let v = self.process_queue_item();

        if let Some(cur_state) = v {
            

            log!("next_calculate: Is success?: {} Iter Count: {} Tick: {} \
            Queue Length: {} Cur van index: {:?}  Row/Col: {:?}",
                success,
                 iter_count,
                cur_state.tick,
                q_len, cur_state.current_van_index,
                cur_state.vans[cur_state.current_van_index.0].cell_index.to_row_col(cur_state.width)
            );
        } else {
            log!("next_calculate: No grid state");
        }

        JsValue::from_serde(&v).unwrap()
    }

    pub fn next_batch_calculate(&mut self, repeat_count: usize) -> JsValue {

        log!("Batch calculate, repeat count: {}", repeat_count);

        if repeat_count > 10_000_000 {
            log!("Too many repetitions...{}", repeat_count);
            return JsValue::from_serde(&self.data).unwrap();
        }

        let target_iter_count = self.iter_count + repeat_count - 1;

        while self.iter_count < target_iter_count {
            self.process_queue_item();
            if self.success.is_some() {
                return JsValue::from_serde(&self.success).unwrap();
            }
            if self.queue.is_empty() {
                log!("Queue is empty");
                return JsValue::from_serde(&self.success).unwrap();
            }
        }

        return self.next_calculate();

    }


    pub fn set_square(&mut self, tile_val: &JsValue) {
        let cell: CellData = tile_val.into_serde().unwrap();

        let idx: usize = cell.row_index * self.data.width + cell.col_index;

        /*log!(
            "Received Row/Col [{}, {}] = idx [{}].  Tile {:?}",
            tile.row_index,
            tile.col_index,
            idx,
            tile
        );*/

        if idx < self.data.tiles.len() {
            self.data.tiles[idx] = cell.tile;
        } else {
            log!(
                "Out of bounds, ignoring"
            );
        }
    }
}