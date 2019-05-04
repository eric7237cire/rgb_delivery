use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;
use crate::solver::structs::{Warehouse, ColorIndex, TileEnum, Bridge, Road, Button, ChoiceOverride, AdjSquareInfo, CellIndex, Van, ALL_DIRECTIONS, get_adjacent_index, GridConnections, GridConnectionsStaticInfo, VanIndex};
use crate::solver::structs::TileEnum::{TileWarehouse, TileRoad, TileBridge};

use std::collections::HashMap;
use crate::solver::func_public::{NUM_COLORS, WHITE_COLOR_INDEX};
use crate::solver::disjointset::DisjointSet;
use crate::solver::grid_state::ComponentMapIdx::*;

#[derive(Default)]
pub struct GridAnalysis {
    //important because if there are none, then a van can only pick up its color
    pub has_poppers: bool,

    pub forced_choices: Vec<ChoiceOverride>,
}


#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Default)]
pub struct GridState {
    pub width: usize,
    pub height: usize,

    pub tiles: Vec<TileEnum>,

    #[serde(skip)]
    pub graph: GridConnections,


    #[serde(skip_deserializing)]
    pub(crate) vans: Vec<Van>,

    //What the bridges will be after the current time tick
    #[serde(skip_deserializing)]
    pub(crate) bridges: Vec<Bridge>,
    pub(crate) buttons: Vec<Button>,

    #[serde(skip_deserializing)]
    pub(crate) warehouses_remaining: usize,


}

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct GridStateKey(Vec<u8>, Vec<(CellIndex, bool, [Option<ColorIndex>; 3])>);

pub enum CanDropOff {
    NoFail,
    NoOk,
    YesOK,
}

impl GridState {

    pub(crate) fn check_bridges_and_buttons(&self) {
        for b in self.bridges.iter() {
            if let TileBridge(check_b) = &self.tiles[b.cell_index.0] {
                assert_eq!(b.color, check_b.color);
                assert_eq!(b.is_up, check_b.is_up);
            }
        }

        for b in self.buttons.iter() {
            if let TileRoad(Road { button_snapshot: Some(check_b), .. }) = &self.tiles[b.cell_index.0] {
                assert_eq!(check_b.color, b.color);
                assert_eq!(check_b.is_pressed, b.is_pressed);
            }
        }
    }

    pub(crate) fn toggle_bridges_and_buttons(&mut self) -> bool {
        log_trace!("Toggling bridges & buttons");

        let pressed_buttons: Vec<ColorIndex> = self.buttons.iter().filter_map(
            |b| if !b.was_pressed_this_tick { None } else { Some(b.color) }).collect();

        for color_to_toggle in pressed_buttons.into_iter() {
            let button_cells: Vec<CellIndex> = self.buttons.iter_mut().filter_map(
                |b| {
                    if b.color != color_to_toggle {
                        None
                    } else {
                        b.was_pressed_this_tick = false;
                        b.is_pressed = !b.is_pressed;
                        Some(b.cell_index)
                    }
                }).collect();

            for bc in button_cells {
                if let TileRoad(r) = &mut self.tiles[bc.0] {
                    r.button_snapshot.as_mut().unwrap().is_pressed = !r.button_snapshot.as_mut().unwrap().is_pressed;
                } else {
                    panic!("Inconsistent");
                }
            }

            let bridge_cells: Vec<CellIndex> = self.bridges.iter_mut().filter_map(
                |b| {
                    if b.color != color_to_toggle {
                        None
                    } else {
                        b.is_up = !b.is_up;
                        Some(b.cell_index)
                    }
                }).collect();

            //also update bridge in tile (use reference, hmmm)
            for bc in bridge_cells.iter() {
                if let TileBridge(tb) = &mut self.tiles[bc.0] {
                    tb.is_up = !tb.is_up;
                } else {
                    panic!("Inconsistent");
                }
            }

            //if any vans are on down(passable) bridges, we fail
            for bc in bridge_cells {
                let has_van = self.vans.iter().any(|v|
                    v.cell_index == bc);

                if has_van {
                    return false;
                }
                //may be worth checking a van wasn't on an open/up bridge...
            }
        }

        true
    }

    pub(crate) fn check_success(&self) -> bool {
        self.warehouses_remaining == 0

        /*self.tiles.iter().all(|tile| {
            match tile {
                TileWarehouse(Warehouse { is_filled, .. }) => {
                    *is_filled
                }
                _ => true
            }
        })*/
    }





    pub(crate) fn pick_up_block_if_exists(&mut self,
                                          current_van_index: usize,
                                          analysis: &GridAnalysis)  {
        log_trace!("pick_up_block_if_exists");

        let current_cell_index = self.vans[current_van_index].cell_index;

        //pick up a block if it exists.  Note a van can pick up a box of any color
        if let TileRoad( Road{ block, ..}) = &mut self.tiles[current_cell_index.0] {

            if let Some(block_color) = block {
                log_trace!("Rolled on a block of color {:?}", block_color);

                if let Some(i) = self.vans[current_van_index].get_empty_slot() {
                    log_trace!("Van picked up a block of color {:?}", block_color);
                    self.vans[current_van_index].boxes[i] = Some(*block_color);
                }

                *block = None;
            }
        };


    }

    pub(crate) fn press_button_if_exists(&mut self, current_van_index: usize) {
        let current_cell_index = self.vans[current_van_index].cell_index;

        let btn_opt = self.buttons.iter_mut().find(|b| b.cell_index ==
            current_cell_index);

        if let Some(btn) = btn_opt {
            log_trace!("Van on a button {:?}", btn);

            if btn.is_pressed {
                return;
            }

            log_trace!("Van pressing button {:?}", btn);

            assert!(!btn.was_pressed_this_tick);
            btn.was_pressed_this_tick = true;
        }
    }

    ///
    pub(crate) fn empty_warehouse_color(&self, current_van_index: usize) -> Option<ColorIndex> {

        let current_cell_index = self.vans[current_van_index].cell_index;

        //on first row
        if current_cell_index.0 < self.width {
            return None;
        }

        let north_tile = &self.tiles[current_cell_index.0 - self.width];
        if let TileWarehouse(Warehouse { color: warehouse_color, is_filled }) = north_tile {
            if *is_filled {
                return None;
            } else {
                return Some(*warehouse_color);
            }
        }

        None
    }

    pub(crate) fn is_below_warehouse(&self, current_van_index: usize) -> bool {

        let current_cell_index = self.vans[current_van_index].cell_index;

        //on first row
        if current_cell_index.0 < self.width {
            return false;
        }

        let north_tile = &self.tiles[current_cell_index.0 - self.width];
        if let TileWarehouse(Warehouse { ..}) = north_tile {
            return true;
        }

        false
    }

    pub(crate) fn can_drop_off_block(&mut self, current_van_index: usize) -> Result<CanDropOff, CanDropOff> {
        if let Some(warehouse_color_index) = self.empty_warehouse_color(current_van_index) {
            let mut able_to_drop_off = false;

            //drop off block at warehouse
            let top_block_color_index =
                {
                    if let Some(color) = self.vans[current_van_index].get_top_box() {
                        color
                    } else {
                        //warehouse would be unfillable
                        return Err(CanDropOff::NoFail);
                    }
                };

            if top_block_color_index == warehouse_color_index && (
                self.vans[current_van_index].color.is_white()
                    || self.vans[current_van_index].color == warehouse_color_index)
            {

                //pop the box
                self.vans[current_van_index].clear_top_box();


                //set warehouse to filled
                {
                    let north_index = self.vans[current_van_index].cell_index.0 - self.width;
                    self.tiles[north_index].mut_warehouse().is_filled = true;
                }

                able_to_drop_off = true;
            }


            //we failed
            if !able_to_drop_off {
                Err(CanDropOff::NoFail)
            } else {
                Ok(CanDropOff::YesOK)
            }
        } else {
            Ok(CanDropOff::NoOk)
        }
    }

    pub(crate) fn filter_map_by_can_have_van<'a>(
        &self,
        current_van_index: usize,
        fixed_choice_opt: &Option<ChoiceOverride>,
        adj_square_info: &'a AdjSquareInfo) -> Option<&'a AdjSquareInfo>

    {
        let direction_index = adj_square_info.direction as usize;
        let adj_cell_index = adj_square_info.cell_index;

        if let Some(ChoiceOverride { direction_index: forced_dir_index, .. }) = fixed_choice_opt {
            if *forced_dir_index != direction_index {
                log_trace!("Not in the forced direction {:?}", direction_index);
                return None;
            }
        }

        //for bridges, also need to check if its up
        if let TileBridge(Bridge { is_up, .. }) = &self.tiles[adj_cell_index.0] {
            if *is_up {
                return None;
            }
        }

        match &self.tiles[adj_cell_index.0] {
            TileRoad(..) | TileBridge(..) => {

                //Check each van that has already moved.  The ones that have yet to move don't need to be checked
                if current_van_index > 0 &&
                    self.vans.iter().take(current_van_index).any(
                        |other_van| adj_cell_index == other_van.cell_index)
                {
                    log_trace!("Another van is there {:?}", direction_index);
                    None
                } else {
                    //no van, so we are good
                    Some(adj_square_info)
                }
            }
            _ => {
                log_trace!("Rejecting direction {:?}, not a road or bridge", direction_index);
                None
            }
        }
    }

    pub(crate) fn handle_move(
        &mut self,
        current_van_index: VanIndex,
        current_tick: usize,
        van_cell_index: CellIndex,
        adj_info: &AdjSquareInfo) {
        //now we have checked it is a road without a van in it, the mask is ok, etc.

        log_trace!("Moving to actual road {:?}.  Row/col: {:?}", adj_info, adj_info.cell_index.to_row_col(self.width));

        let moving_to_cell_index = adj_info.cell_index;

        //must have a connection in the direction we are moving

        //assert!(self.graph.is_connected( van_cell_index, adj_info.direction));
        //assert!(self.graph.is_connected( moving_to_cell_index, adj_info.direction.opposite() ));

        //Now we remove the edge
        //self.graph.set_is_connected( van_cell_index, adj_info.direction, false);
        self.graph.is_connected[van_cell_index.0] &= !(1 << adj_info.direction as u8);
        self.graph.is_connected[moving_to_cell_index.0] &= !(1 << adj_info.direction.opposite() as u8);
        //self.graph.set_is_connected( moving_to_cell_index, adj_info.direction.opposite(), false);

        //assert!(!self.graph.is_connected( van_cell_index, adj_info.direction));
        //Edge is already removed because DRY; we cant do a U turn
        //assert!(!self.graph.is_connected( moving_to_cell_index, adj_info.direction.opposite() ));

        //remove van & set used mask
        self.tiles[van_cell_index.0].set_leaving_van(current_van_index, current_tick, adj_info.direction as usize);

        //add van to next square
        {
            let van = &mut self.vans[current_van_index.0];
            van.cell_index = moving_to_cell_index;
            van.tick += 1;
        }

        self.tiles[moving_to_cell_index.0].set_arriving_van(current_van_index, &self.vans[current_van_index.0], current_tick,
                                                            //opposite direction index
                                                            ALL_DIRECTIONS.iter().position(|d| d == &adj_info.direction.opposite()).unwrap());
    }


    //if did a drop off, returns a grid state to enqueue
    pub fn handle_warehouse_drop_off(&mut self, current_van_index: VanIndex, gc_static_info: &GridConnectionsStaticInfo) -> bool {
        //check if we can drop a block off
        if self.empty_warehouse_color(current_van_index.0).is_some() {
            match self.can_drop_off_block(current_van_index.0) {
                Err(_) => return false,
                Ok(CanDropOff::YesOK) => {
                    assert!(self.empty_warehouse_color(current_van_index.0).is_none());
                    assert!(self.warehouses_remaining > 0);

                    self.warehouses_remaining -= 1;



                }
                _ => {}
            }
        }

        return true;
    }

    pub fn handle_post_move_actions(&mut self, analysis: &GridAnalysis,
                                    gc_static_info: &GridConnectionsStaticInfo,
                                    current_van_index: usize) -> bool {
        if current_van_index == self.vans.len() {
            if !self.toggle_bridges_and_buttons() {
                return false;
            }
        }

        self.press_button_if_exists(current_van_index);

        self.pick_up_block_if_exists(
            current_van_index,
            analysis);

        //check if we can drop a block off
        if !self.handle_warehouse_drop_off(VanIndex(current_van_index),
                                           gc_static_info) {
            return false;
        };

        return true;
    }

    /// On road, toggle-able thing that will pop off the block if active. if we do that, returns that state
    pub fn handle_block_popper(&mut self, current_van_index: VanIndex, current_tick: usize) -> Option<Self> {

        let cell_index = self.vans[current_van_index.0].cell_index.0;
        let on_usable_popper = match &mut self.tiles[cell_index] {
            TileRoad(Road { used_popper_tick, has_popper, .. }) => {
                //need to check it hasn't already been toggled
                let r = *has_popper && used_popper_tick.is_none();

                //in all cases, set to used since we can't use it after this
                *has_popper = false;

                r
            }
            _ => false
        };

        if !on_usable_popper {
            return None;
        }

        //do we have a box to pop?
        if let Some(top_box_color) = self.vans[current_van_index.0].get_top_box() {
            let mut if_popper_active = self.clone();
            if_popper_active.vans[current_van_index.0].clear_top_box();


            if let TileRoad(road) = &mut if_popper_active.tiles[cell_index] {
                assert!(road.block.is_none());

                road.block = Some(top_box_color);
                road.used_popper_tick = Some(current_tick);

                Some(if_popper_active)
            } else {
                panic!("Should be a road");
            }
        } else {
            None
        }
    }




    ///basically in each distinct connected component, we should have the same numbers of blocks and warehouses of each color
    //warning on component_number
    #[allow(unused_variables)]
    pub(crate) fn check_graph_validity(&self) -> bool {
        let mut ds = DisjointSet::new(self.tiles.len());

        for (idx, is_connected_mask) in self.graph.is_connected.iter().enumerate() {

            for (dir_idx, dir) in ALL_DIRECTIONS.iter().enumerate().take(2) {
                if is_connected_mask & (1 << dir_idx) > 0 {
                    let adj_idx = get_adjacent_index(CellIndex(idx), self.height, self.width, *dir).expect("Should not be connected if there is no adj cell");

                    //log_trace!("Merging cells {} and {}", idx, adj_idx);
                    ds.merge_sets(idx, adj_idx.0);
                }
            }
        }

        //for each color, get unfilled warehouse count & block count

        let mut component_to_counts: ComponentCountMap = HashMap::new();

        for (idx, tile) in self.tiles.iter().enumerate() {
            let component_number = ds.get_repr(idx);

            match tile {
                TileRoad(Road { block: Some(block), .. }) => {
                    log_trace!("Block in cell {}, component {}, color {}", idx, component_number, block.0);
                    add_component_to_map(&mut component_to_counts, component_number, *block, BLOCK);
                }
                TileWarehouse(Warehouse { is_filled: false, color }) => {
                    log_trace!("unfilled warehouse in cell {}, component {}, color {}", idx, component_number, color.0);
                    add_component_to_map(&mut component_to_counts, component_number, *color, WAREHOUSE);
                }
                _ => {}
            }

            match tile {
                TileRoad(Road { has_popper: true, .. }) => {
                    log_trace!("Unused popper in cell {}, component {}, color {}", idx, component_number);
                    add_component_to_map(&mut component_to_counts, component_number, ColorIndex(WHITE_COLOR_INDEX), POPPER);
                },
                _ => {}
            }
        }

        for van in self.vans.iter() {
            let component_number = ds.get_repr(van.cell_index.0);

            //they are out of the game
            if van.is_done {
                continue;
            }

            add_component_to_map(&mut component_to_counts, component_number, van.color, VAN);

            for opt_box in van.boxes.iter() {
                if let Some(color) = opt_box {
                    add_component_to_map(&mut component_to_counts, component_number, *color, BLOCK);

                    if !van.color.is_white() && color != van.color && component_to_counts[WHITE_COLOR_INDEX][POPPER] == 0 {
                        log_trace!("No poppers to get rid of block for van {:?}", van);
                        return false;
                    }
                }
            }
        }

        //now we can check for consistency
        for (component_number, color_count) in component_to_counts.iter() {
            for color_index in 0..5 {
                if color_count[color_index][BLOCK as usize] != color_count[color_index][WAREHOUSE as usize] {
                    log_trace!("Inconsistent block / unfilled warehouse in component {} for color # {}-- {:?}", component_number, color_index, color_count[color_index]);
                    return false;
                }

                if color_count[color_index][BLOCK as usize] > 0 && (color_count[WHITE_COLOR_INDEX][VAN as usize] + color_count[color_index][VAN as usize] == 0) {
                    log_trace!("No vans able to do the drop offs for component {} for color # {}-- {:?}", component_number, color_index, color_count[color_index]);
                    return false;
                }

                //we don't need to check for warehouses and vans since we know the block count must == the warehouse count
                //and each block has a van that can handle it

                //a van shouldn't be without blocks, should have set is_done.  Need to skip the first tick though since we haven't yet set that
                if self.tick > 1 &&
                    color_index != WHITE_COLOR_INDEX &&
                    color_count[color_index][BLOCK as usize] == 0 &&
                    color_count[color_index][VAN as usize] > 0 {
                    log_trace!("We have a van but with no blocks to deal with for component {} for color # {}-- {:?}", component_number, color_index, color_count[color_index]);
                    return false;
                }
            }
        }


        return true;
    }


}

#[repr(u8)]
enum ComponentMapIdx {
    BLOCK = 0,
    WAREHOUSE = 1,
    VAN = 2,
    POPPER = 3
}

const NUM_THINGS:usize = 4;

type ComponentCountMap = HashMap<usize, [[u8; NUM_THINGS]; NUM_COLORS]>;

//indexs block 0, warehouse 1, van 2
fn add_component_to_map(component_to_counts: &mut ComponentCountMap, component_number: usize, color: ColorIndex, thing_idx: ComponentMapIdx) {
    let counts = component_to_counts.entry(component_number).or_insert(Default::default());

    counts[color.0][thing_idx as usize] += 1
}