use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;
use super::structs::{Warehouse, ColorIndex, VanIndex, TileEnum, Bridge, Road, Button, ChoiceOverride, AdjSquareInfo, CellIndex,ALL_DIRECTIONS};

use super::structs::TileEnum::{TileWarehouse, TileRoad, TileBridge};

use crate::solver::func_public::{NUM_COLORS, WHITE_COLOR_INDEX};
use crate::solver::disjointset::DisjointSet;
use crate::solver::grid_state::ComponentMapIdx::*;
use crate::solver::structs::{Van, GridConnections, GridConnectionsStaticInfo, get_adjacent_index};

#[derive(Default)]
pub struct GridAnalysis {

    //important because if there are none, then a van can only pick up its color
    pub has_poppers: bool,

    pub forced_choices: Vec<ChoiceOverride>
}


#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Default)]
pub struct GridState {
    pub width: usize,
    pub height: usize,

    pub tiles: Vec<TileEnum>,

    #[serde(skip)]
    pub graph: GridConnections,

    //Js=>Rust will ignore this
    #[serde(skip_deserializing)]
    pub(crate) tick: usize,

    #[serde(skip_deserializing)]
    pub(crate) vans: Vec<Van>,

    //What the bridges will be after the current time tick
    #[serde(skip_deserializing)]
    pub(crate) bridges: Vec<Bridge>,
    pub(crate) buttons: Vec<Button>,

    #[serde(skip_deserializing)]
    pub(crate) warehouses_remaining: usize,

    #[serde(skip)]
    pub(crate) current_van_index: VanIndex,


}

#[derive(Hash, PartialEq, Eq)]
pub (crate) struct GridStateKey(Vec<u8>, Vec<(CellIndex,bool,[Option<ColorIndex>;3])>);

pub enum CanDropOff {
    NoFail,
    NoOk,
    YesOK,
}

impl GridState {
    pub(crate) fn increment_current_van_index(&mut self) -> Result<bool,()> {

        let mut incremented_tick = false;

        for _ in 0..self.vans.len() {
            //increment
            if self.current_van_index.0 == self.vans.len() - 1 {
                log_trace!("Advancing a tick {} => {}", self.tick, self.tick + 1);

                self.tick += 1;
                self.current_van_index = 0usize.into();
                incremented_tick=true;
            } else {
                self.current_van_index.0 += 1;
            }

            if !self.current_van().is_done {
                return Ok(incremented_tick);
            } else {
                log_trace!("Van #{:?}: {:?} is done, skipping", self.current_van_index, self.vans[self.current_van_index.0]);
            }
        }

        Err(())
    }


    pub(crate) fn check_bridges_and_buttons(&self) {
        for b in self.bridges.iter() {
            if let TileBridge(check_b) = &self.tiles[b.cell_index.0] {
                assert_eq!(b.color, check_b.color);
                assert_eq!(b.is_up, check_b.is_up);
            }
        }

        for b in self.buttons.iter() {
            if let TileRoad( Road {button_snapshot:Some(check_b),..}) = &self.tiles[b.cell_index.0] {
                assert_eq!(check_b.color, b.color);
                assert_eq!(check_b.is_pressed, b.is_pressed);
            }
        }
    }

    pub(crate) fn toggle_bridges_and_buttons(&mut self) -> Result<(),()> {

        log_trace!("Toggling bridges & buttons");

        let pressed_buttons:Vec<ColorIndex> = self.buttons.iter().filter_map(
            |b| if !b.was_pressed_this_tick { None } else {Some(b.color)}).collect();

        for color_to_toggle in pressed_buttons.into_iter() {


            let button_cells: Vec<CellIndex> = self.buttons.iter_mut().filter_map(
                |b| {
                    if b.color != color_to_toggle {
                        None
                    } else {
                        b.was_pressed_this_tick=false;
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

            let bridge_cells : Vec<CellIndex> = self.bridges.iter_mut().filter_map(
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

                let has_van = self.vans.iter().any( |v|
                    v.cell_index == bc );

                if has_van {
                    return Err(());
                }
                //may be worth checking a van wasn't on an open/up bridge...
            }
        }

        Ok(())
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

    pub(crate) fn current_cell_index(&self) -> CellIndex {
        self.vans[self.current_van_index.0].cell_index
    }
    pub(crate) fn current_cell_mut(&mut self) -> &mut TileEnum {
        &mut self.tiles[ self.vans[self.current_van_index.0].cell_index.0 ]
    }
    pub(crate) fn current_cell(&self) -> &TileEnum {
        &self.tiles[ self.vans[self.current_van_index.0].cell_index.0 ]
    }

    pub(crate) fn current_van(&self) -> &Van {
        let i:usize = self.current_van_index.into();
        &self.vans[ i ]
    }
    pub(crate) fn current_van_mut(&mut self) -> &mut Van {
        let i:usize = self.current_van_index.into();
        &mut self.vans[ i ]
    }

    pub(crate) fn pick_up_block_if_exists(&mut self, analysis: &GridAnalysis ) -> Result<(),()> {


        log_trace!("pick_up_block_if_exists");

        let opt = match self.current_cell() {
            TileRoad(road) => {

                if Some(self.tick) == road.used_popper_tick {
                    //don't pick up the block we just set down
                    None
                } else {
                    road.block
                }
            },
            _ => None
        };

        //pick up a block if it exists.  Note a van can pick up a box of any color
        if let Some(block_color) = opt {
            log_trace!("Rolled on a block of color {:?}", block_color);

            if let Some(i) = self.vans[self.current_van_index.0].get_empty_slot() {

                if !analysis.has_poppers
                    && !self.vans[self.current_van_index.0].color.is_white()
                    && self.vans[self.current_van_index.0].color != block_color{
                    log_trace!("No way of dropping block off");
                    return Err(());
                }

                log_trace!("Van picked up a block of color {:?}", block_color);
                self.vans[self.current_van_index.0].boxes[i] = Some(block_color);
                
                if let TileRoad( road ) = self.current_cell_mut() {
                    road.block = None;
                }
            }
        }

        Ok(())
    }

    pub(crate) fn press_button_if_exists(&mut self) {

        let current_cell_index = self.vans[self.current_van_index.0].cell_index;

        let btn_opt = self.buttons.iter_mut().find( |b| b.cell_index ==
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
    pub(crate) fn empty_warehouse_color(&self) -> Option<ColorIndex> {
        let current_cell_index = self.current_cell_index();

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

    pub(crate) fn can_drop_off_block(&mut self) -> Result<CanDropOff, CanDropOff> {
        if let Some(warehouse_color_index) = self.empty_warehouse_color() {
            let mut able_to_drop_off = false;

            //drop off block at warehouse
            let top_block_color_index =
                {
                    if let Some(color) = self.vans[self.current_van_index.0].get_top_box() {
                        color
                    } else {
                        //warehouse would be unfillable
                        return Err(CanDropOff::NoFail);
                    }
                };

            if top_block_color_index == warehouse_color_index && (
                self.current_van().color.is_white()
                    || self.current_van().color == warehouse_color_index)
            {

                //pop the box
                self.vans[self.current_van_index.0].clear_top_box();


                //set warehouse to filled
                {
                    let north_index = self.current_cell_index().0 - self.width;
                    self.tiles[ north_index ].mut_warehouse().is_filled = true;
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

    pub (crate) fn filter_map_by_can_have_van<'a>(
        &self,
        fixed_choice_opt: &Option<ChoiceOverride>,
        adj_square_info: &'a AdjSquareInfo) -> Option<&'a AdjSquareInfo>
        
    {

        let direction_index = adj_square_info.direction as usize;
        let adj_cell_index = adj_square_info.cell_index;

        if let Some( ChoiceOverride{ direction_index:forced_dir_index, ..}) = fixed_choice_opt {
            if *forced_dir_index != direction_index {
                 log_trace!("Not in the forced direction {:?}", direction_index);
                return None;
            }
        }

        //for bridges, also need to check if its up
        if let TileBridge( Bridge {is_up, ..}) = &self.tiles[adj_cell_index.0] {
            if *is_up {
                return None;
            }
        }

        match &self.tiles[adj_cell_index.0] {
            TileRoad(..) | TileBridge(..) => {

                //Check each van that has already moved.  The ones that have yet to move don't need to be checked
                if self.current_van_index.0 > 0 &&
                    self.vans.iter().take(self.current_van_index.0).any(
                        |other_van| adj_cell_index == other_van.cell_index)
                {
                    log_trace!("Another van is there {:?}", direction_index);
                    None
                } else {
                    //no van, so we are good
                    Some(adj_square_info)
                }
            },
            _ => {
                log_trace!("Rejecting direction {:?}, not a road or bridge", direction_index);
                None
            }
        }
    }

    pub(crate) fn handle_move(
        &mut self,
        van_cell_index: CellIndex,
        adj_info: &AdjSquareInfo) {
        //now we have checked it is a road without a van in it, the mask is ok, etc.



        log_trace!("Moving to actual road {:?}.  Row/col: {:?}", adj_info, adj_info.cell_index.to_row_col(self.width));

        let moving_to_cell_index =adj_info.cell_index;

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
        self.tiles[van_cell_index.0].set_leaving_van(self.current_van_index, self.tick, adj_info.direction as usize);

        //add van to next square
        {
            let van = self.current_van_mut();
            van.cell_index = moving_to_cell_index;
            van.tick += 1;
        }

        self.tiles[moving_to_cell_index.0].set_arriving_van(self.current_van_index, &self.vans[self.current_van_index.0], self.tick,
            //opposite direction index
            ALL_DIRECTIONS.iter().position(|d| d == &adj_info.direction.opposite()).unwrap());
           
    }


    //if did a drop off, returns a grid state to enqueue
    pub fn handle_warehouse_drop_off(&mut self, gc_static_info: &GridConnectionsStaticInfo) -> Result<Option<Self>, ()> {
        //check if we can drop a block off
        if self.empty_warehouse_color().is_some() {
            match self.can_drop_off_block() {
                Err(_) => Err(()),
                Ok(CanDropOff::YesOK) => {

                    assert!(self.empty_warehouse_color().is_none());
                    assert!(self.warehouses_remaining > 0);

                    self.warehouses_remaining -= 1;

                    //test what happens if we stop
                    if self.can_current_van_stop() {
                        let mut if_van_stops_state = self.clone();
                        if_van_stops_state.current_van_mut().is_done = true;

                        let stopped_cell_index = if_van_stops_state.vans[if_van_stops_state.current_van_index.0].cell_index;

                        assert!(if_van_stops_state.tiles[stopped_cell_index.0].get_van().is_some());

                        //disconnect this square and everything adjacent to it
                        if_van_stops_state.graph.is_connected[ stopped_cell_index.0] = 0;

                        for adj in gc_static_info.adj_info[stopped_cell_index.0].iter().filter_map( |a| a.as_ref()) {
                            if_van_stops_state.graph.is_connected[ adj.cell_index.0] &= !(1 << adj.direction.opposite() as u8);
                        }

                        Ok(Some(if_van_stops_state))
                    } else {
                        Ok(None)
                    }
                    
                },
                _ => Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// On road, toggle-able thing that will pop off the block if active. if we do that, returns that state
    pub fn handle_block_popper(&mut self) -> Result<Option<Self>, ()> {


        let cell_index = self.current_cell_index().0;
        let on_usable_popper = match &mut self.tiles[cell_index] {
            TileRoad( Road{ used_popper_tick,has_popper,.. }) => {
                //need to check it hasn't already been toggled
                let r = *has_popper && used_popper_tick.is_none();

                //in all cases, set to used since we can't use it after this
                *has_popper = false;

                r
            },
            _ => false
        };

        if !on_usable_popper {
            return Ok(None);
        }

        //do we have a box to pop?
        if let Some(top_box_color) = self.vans[self.current_van_index.0].get_top_box() {

            let mut if_popper_active = self.clone();
            if_popper_active.vans[if_popper_active.current_van_index.0].clear_top_box();

            let cell_index = if_popper_active.current_cell_index().0;
            if let TileRoad(road) = &mut if_popper_active.tiles[cell_index] {
                assert!(road.block.is_none());

                road.block = Some(top_box_color);
                road.used_popper_tick = Some(self.tick);

                Ok(Some(if_popper_active))
            } else {
                panic!("Should be a road");
            }
        } else {

            Ok(None)
        }
        //create the state

    }


    pub fn can_current_van_stop(&self) -> bool {
        let any_non_stopped_white_vans = self.vans.iter().any(  |v| v.color.is_white() && !v.is_done );

        if any_non_stopped_white_vans {
            return true;
        }

        //Is there an empty warehouse of this vans color?
        let empty_wh_count = self.tiles.iter().filter(|tile| {
            match tile {
                TileWarehouse(Warehouse { is_filled,color, .. }) => {
                    !is_filled && color == &self.vans[self.current_van_index.0].color
                },
                _ => false
            }
        }).count();

        if empty_wh_count == 0 {
            return true;
        }

        //can another van handle it?
        let other_van = self.vans.iter().enumerate().any( | (v_idx,v)|
                                                              //not current van
                                                              v_idx != self.current_van_index.0 &&
            //not wrong color
                                                                  !v.is_done && v.color == self.vans[self.current_van_index.0].color);

        //another van could in theory handle it
        other_van
    }

    ///basically in each distinct connected component, we should have the same numbers of blocks and warehouses of each color    
    //warning on component_number
    #[allow(unused_variables)]
    pub(crate) fn check_graph_validity(&self, gs_static_info: &GridConnectionsStaticInfo) -> bool {

        let mut ds = DisjointSet::new(self.tiles.len());

        for (idx, is_connected_mask) in self.graph.is_connected.iter().enumerate() {
            for (dir_idx, dir) in ALL_DIRECTIONS.iter().enumerate() {
                //log_trace!("Merging cells {} and {}", idx, adj_idx);
                if is_connected_mask & (1 << dir_idx) > 0 {
                    let adj_idx = get_adjacent_index(CellIndex(idx), self.height, self.width, *dir).expect("Should not be connected if there is no adj cell");
                    //log_trace!("Merging cells {} and {}", idx, adj_idx);
                    ds.merge_sets(idx, adj_idx.0);
                }
            }
        }

        //for each color, get unfilled warehouse count & block count

        let mut component_to_counts: Vec< [ [usize;3]; NUM_COLORS ]> = vec![ Default::default(); ds.number_of_elems() ];
        //let index_to_comp_num
        //let mut component_to_has_popper: HashMap<usize, bool> = HashMap::new();

        for (idx, tile) in self.tiles.iter().enumerate() {

            let component_number = ds.get_repr(idx);

            match tile {
                TileRoad(Road { block: Some(block), .. }) => {
                    log_trace!("Block in cell {}, component {}, color {}", idx, component_number, block.0);
                    add_component_to_map(&mut component_to_counts, component_number, *block, BLOCK);

                },
                TileWarehouse( Warehouse{is_filled: false, color}) => {
                    log_trace!("unfilled warehouse in cell {}, component {}, color {}", idx, component_number, color.0);
                    add_component_to_map(&mut component_to_counts, component_number, *color, WAREHOUSE);
                }
                _ => {}
            }

            /*
            match tile {
                TileRoad(Road { has_popper:true, .. }) => {
                    log_trace!("in cell {}, component {}, has popper {}", idx, component_number, true);

                    *component_to_has_popper.entry(component_number).or_insert(false) = true;
                },
                _ => {}
            }*/
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
                }
            }
        }

        //now we can check for consistency
        for (component_number, color_count) in component_to_counts.iter().enumerate() {

            for color_index in 0..NUM_COLORS {
                if color_count[color_index][BLOCK as usize] != color_count[color_index][WAREHOUSE as usize] {
                    log_trace!("Inconsistent block / unfilled warehouse in component {} for color # {}-- {:?}", component_number, color_index, color_count[color_index]);
                    return false;
                }

                if color_count[color_index][BLOCK as usize] > 0 && (color_count[WHITE_COLOR_INDEX][VAN as usize] + color_count[color_index][VAN as usize] == 0) {
                    log_trace!("No vans able to do the drop offs for component {} for color # {}-- {:?}", component_number, color_index, color_count[color_index]);
                    return false;
                }

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

        //do we have a van that has a box of a different color
        /*
        for van in self.vans.iter().filter(|v| !v.color.is_white()) {
            let component_number = ds.get_repr(van.cell_index.0);

            let has_box_other_color = van.boxes.iter().any(|c| {
                if let Some(color) = c {
                    if *color != van.color {
                        return true;
                    }
                }
                return false;
            });

            if has_box_other_color && *component_to_has_popper.entry(component_number).or_insert(false) == false {
                log_trace!("No popper available!");
                return false;
            }

        }*/


        return true;
    }



}

enum ComponentMapIdx {
    BLOCK = 0,
    WAREHOUSE = 1,
    VAN = 2
}

//indexs block 0, warehouse 1, van 2
fn add_component_to_map(component_to_counts: &mut Vec< [ [usize;3]; NUM_COLORS ]>, component_number: usize, color: ColorIndex, thing_idx: ComponentMapIdx) {

    component_to_counts[component_number][color.0][thing_idx as usize] += 1

}