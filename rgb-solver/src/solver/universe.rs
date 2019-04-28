use crate::solver::grid_state::{GridState, GridAnalysis, GridGraph};
use crate::solver::struct_defs::{ChoiceOverride, CellIndex, AdjSquareInfo, Bridge, Button, CellData, VanIndex, TileEnum, Road, CalculationResponse};
use std::collections::vec_deque::VecDeque;
use crate::solver::misc::{ALL_DIRECTIONS, get_adjacent_index, GraphBridge};
use crate::solver::struct_defs::TileEnum::{TileRoad, TileBridge, TileWarehouse};
use crate::solver::van::Van;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use crate::solver::utils;
use crate::solver::utils::set_panic_hook;
use crate::solver::struct_defs::Directions::NORTH;

#[cfg_attr(not(target_arch = "x86_64"), wasm_bindgen())]
#[derive(Default)]
pub struct Universe {
    pub(crate) data: GridState,

    pub(crate) choice_override_list: Vec<ChoiceOverride>,

    //below are used for calculating
    pub(crate) queue: VecDeque<GridState>,

    pub(crate) success_state: Option<GridState>,

    pub(crate) iter_count: usize,

    pub(crate) analysis: GridAnalysis,
}

//private
impl Universe {
    /// Gets adj indexes, checking grid limits
    fn get_adjacent_square_indexes(&self, cell_index: CellIndex,
                                   is_connected_mask: u8) -> Vec<AdjSquareInfo>
    {
        ALL_DIRECTIONS.iter().enumerate().filter_map(|(dir_index, dir)| {

            //first check the mask
            if is_connected_mask & *dir as u8 == 0 {
                return None;
            }

            let adj_index = get_adjacent_index(cell_index, self.data.height, self.data.width, *dir);

            if let Some(adj_index) = adj_index {
                Some(AdjSquareInfo { direction: *dir, cell_index: adj_index, direction_index: dir_index })
            } else {
                None
            }
        }).collect()
    }

    /// If we provided a choice for the row/col and perhaps tick (as a van can go through the
    /// same cell 2x
    fn get_fixed_choice(&self, cur_state: &GridState) -> Option<ChoiceOverride> {
        let (cur_row_index, cur_col_index) = cur_state.current_cell_index().to_row_col(cur_state.width);

        let o = self.choice_override_list.iter()
            //also any system generated forced choices
            .chain( self.analysis.forced_choices.iter())
            .find(|co| {
            if let Some(forced_tick) = co.tick {
                if forced_tick != cur_state.tick {
                    return false;
                }
            }

            if let Some(van_index) = co.van_index {
                if van_index != cur_state.current_van_index {
                    return false;
                }
            }

            
            cur_row_index == co.row_index
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
        let is_connected_list = self.data.tiles.iter().enumerate().map(|(cur_square_index, tile)| {


            if let Some(connection_mask) = tile.get_connection_mask() {

                let mut is_connected: u8 = 0;

                for (dir_idx, adj_dir) in ALL_DIRECTIONS.iter().enumerate() {
                    let adj_square_index = get_adjacent_index(CellIndex(cur_square_index), self.data.height, self.data.width, *adj_dir);

                    if let Some(adj_square_index) = adj_square_index {
                        if let Some(adj_connection_mask) = self.data.tiles[adj_square_index.0].get_connection_mask()
                        {
                            if (connection_mask & (*adj_dir as u8)) > 0 && (adj_connection_mask & (adj_dir.opposite() as u8) > 0) {
                                assert_eq!(*adj_dir as u8, 1 << dir_idx);
                                is_connected |= 1 << dir_idx;
                            }
                        } else if adj_dir == &NORTH {
                            if let TileWarehouse(_) = &self.data.tiles[adj_square_index.0] {
                                //special case that we want warehouses to be connected to the cell to their south
                                assert_eq!(*adj_dir as u8, 1 << dir_idx);
                                is_connected |= 1 << dir_idx;
                            }
                        }
                    }
                }

                is_connected
            } else {
                0
            }

        }).collect();

        GridGraph { is_connected: is_connected_list }
    }

    pub(crate) fn initial_graph_analysis(&self) -> GridAnalysis {

        //for the moment, not useful since this basically finds warehouses and dead ends
        //later might be interesting to use to make sure if a van & warehouse are on one side of a bridge
        //and the block on another, we can eliminate a gridstate candidate
        let mut b: GraphBridge = Default::default();
        log_trace!("Finding bridges");
        b.do_it(&self.data.graph, self.data.height, self.data.width);


        let mut ga = GridAnalysis {
            has_poppers: self.data.tiles.iter().any(|t| if let TileRoad(r) = t {
                return r.has_popper;
            } else { false }),
            ..Default::default()
        };

        //if a van starts on a square that has 2 options, see if there is a block on that path (no warehouse), if so, we must go towards the block

        for (van_index,van) in self.data.vans.iter().enumerate() {

            let van_index = VanIndex(van_index);

            if  self.data.graph.is_connected[van.cell_index.0].count_ones() != 2 {
                continue;
            }

            let van_adj_cells = self.get_adjacent_square_indexes(van.cell_index, self.data.graph.is_connected[van.cell_index.0]);

            assert_eq!(2, van_adj_cells.len());

            let forced_adj_cell = van_adj_cells.iter().find( | potential_force_cell | {
                let mut last_cell_index:CellIndex = van.cell_index;
                let mut cur_cell_index:CellIndex = potential_force_cell.cell_index;
                loop {
                    let next_cell_index = self.get_adjacent_square_indexes(cur_cell_index, self.data.graph.is_connected[cur_cell_index.0])
                       .into_iter().filter(|ai| ai.cell_index != last_cell_index).collect::<Vec<AdjSquareInfo>>();

                    if next_cell_index.len() > 1 {
                        break;
                    }

                    last_cell_index = cur_cell_index;
                    cur_cell_index = next_cell_index[0].cell_index;

                    //don't neet to check if warehouse to north because there would be a connection

                    if let TileRoad( Road{ block: Some(_block),..}) = &self.data.tiles[cur_cell_index.0] {

                        return true;
                    }
                }

                false 

            });

            if let Some( forced_adj_cell ) = forced_adj_cell {
                //found a force choice
                let rc = van.cell_index.to_row_col(self.data.width);
                
                ga.forced_choices.push(ChoiceOverride {
                    row_index: rc.0,
                    col_index: rc.1,
                    van_index: Some(van_index),
                    direction_index: forced_adj_cell.direction_index,
                    ..Default::default()
                });

                log!("Found a forced choice: {:?}", ga.forced_choices);
                break;
            }
        }

        ga
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
        if self.success_state.is_some() {
            return self.success_state.as_ref();
        }
        while let Some(mut cur_state) = self.queue.pop_front() {
            self.iter_count += 1;


            //self.current_calc_state = Some(cur_state.clone());

            //check success, where all warehouses are filled
            if cur_state.check_success() {
                log!("Success!");
                self.success_state = Some(cur_state);
                return self.success_state.as_ref();
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
                log_trace!("Adding state where van does not move for van index: {}", cur_state.current_van_index.0);
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
                    }
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
                }
                Err(_) => continue,
                _ => ()
            };

            match cur_state.handle_block_popper() {
                Ok(Some(mut next_state)) => {
                    //reset to values as when it was just popped
                    next_state.tick = save_for_toggle.0;
                    next_state.current_van_index = save_for_toggle.1;
                    self.queue.push_front(next_state);
                }
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

            let adj_info_filtered_list: Vec<&AdjSquareInfo> = adj_square_indexes.iter().filter_map(
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

    //return CalculationResponse
    pub(crate) fn next_calculate(&mut self) -> JsValue {
        let iter_count = self.iter_count;
        let q_len = self.queue.len();
        let success = self.success_state.is_some();

        let v = self.process_queue_item();

        let r = if let Some(cur_state) = v {
            log!("next_calculate: Is success?: {} Iter Count: {} Tick: {} \
            Queue Length: {} Cur van index: {:?}  Row/Col: {:?}",
                 success,
                 iter_count,
                 cur_state.tick,
                 q_len, cur_state.current_van_index,
                 cur_state.vans[cur_state.current_van_index.0].cell_index.to_row_col(cur_state.width)
            );
            CalculationResponse{               
                grid_state: Some(cur_state.clone()),
                iteration_count: self.iter_count,
                success,
                ..Default::default()
            }
        } else {            
            CalculationResponse{
                error_message: Some( "No grid state".to_string() ),
                iteration_count: self.iter_count,
                success,
                ..Default::default()
            }
        };

        //false for should not stop
        JsValue::from_serde(&r).unwrap()
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
        let lo: Vec<ChoiceOverride> = choice_override_list.into_serde().unwrap();

        self.choice_override_list = lo.clone();

        //log!("Set override list {:?}", lo);
    }

    pub fn init_calculate(&mut self) {
        set_panic_hook();
        
        self.queue = VecDeque::new();

        self.iter_count = 0;
        self.data.tick = 0;

        self.success_state = None;

        self.data.vans = self.initial_van_list();
        self.data.buttons = self.initial_button_list();
        self.data.bridges = self.initial_bridge_list();
        self.data.graph = self.initial_graph();

        self.data.warehouses_remaining = self.data.tiles.iter().filter(|t| {
            if let TileWarehouse(_) = t {
                true
            } else {
                false
            }
        }).count();

        //we increment on pop, so...
        self.data.current_van_index = VanIndex(self.initial_van_list().len() - 1);


        //reset road history
        for tile in self.data.tiles.iter_mut() {
            match tile {
                TileRoad(road) => {
                    road.used_van_index = Default::default();
                    road.van_snapshot = None;
                }
                TileBridge(bridge) => {
                    bridge.used_tick = None;
                    bridge.used_van_index = None;
                    bridge.van_snapshot = None;
                }
                _ => {}
            }
        }

        //vans should start on roads
        let van_cells: Vec<CellIndex> = self.data.vans.iter().map(|v| v.cell_index).collect();

        for (van_idx, v_cell_index) in van_cells.iter().enumerate() {
            if let TileRoad(road) = &mut self.data.tiles[v_cell_index.0] {
                road.van_snapshot = Some(self.data.vans[van_idx].clone());
            } else {
                panic!("Van is not on a road");
            }
        }

        self.analysis = self.initial_graph_analysis();

        self.queue.push_back(self.data.clone());
    }



    //returns CalculationResponse
    pub fn next_batch_calculate(&mut self, repeat_count: usize) -> JsValue {
        log!("Batch calculate, repeat count: {}", repeat_count);

        if repeat_count > 10_000_000 {
            return JsValue::from_serde(&CalculationResponse {
                error_message: Some(format!("Too many repetitions...{}", repeat_count)),
                ..Default::default()
            }).unwrap();
        }

        let target_iter_count = self.iter_count + repeat_count - 1;

        while self.iter_count < target_iter_count {
            self.process_queue_item();
            if let Some(success_state) = &self.success_state {
                return JsValue::from_serde(&CalculationResponse {
                    grid_state: Some(success_state.clone()),
                    iteration_count: self.iter_count,
                    success: true,
                    ..Default::default()
                }).unwrap();
            }
            if self.queue.is_empty() {
                return JsValue::from_serde(&CalculationResponse {
                    error_message: Some(format!("Queue is empty")),
                    ..Default::default()
                }).unwrap();
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