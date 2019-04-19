use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;
use crate::solver::struct_defs::{Warehouse, ColorIndex, VanIndex, TileEnum, Bridge, Road, Button, ChoiceOverride, AdjSquareInfo, CellIndex};
use crate::solver::van::Van;
use crate::solver::struct_defs::TileEnum::{TileWarehouse, TileRoad, TileBridge};
use crate::solver::universe_impl::ALL_DIRECTIONS;

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Default, Hash, Eq, PartialEq)]
pub struct GridState {
    pub width: usize,
    pub height: usize,

    pub tiles: Vec<TileEnum>,

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

pub enum CanDropOff {
    NoFail,
    NoOk,
    YesOK,
}

impl GridState {
    pub(crate) fn increment_current_van_index(&mut self) -> Result<(),()> {

        for _ in 0..self.vans.len() {
            //increment
            if self.current_van_index.0 == self.vans.len() - 1 {
                log_trace!("Advancing a tick {} => {}", self.tick, self.tick + 1);

                self.tick += 1;
                self.current_van_index = 0usize.into();
            } else {
                self.current_van_index.0 += 1;
            }

            if !self.current_van().is_done {
                return Ok(());
            } else {
                log_trace!("Van #{:?}: {:?} is done, skipping", self.current_van_index, self.vans[self.current_van_index.0]);
            }
        }

        Err(())
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

    fn get_current_block_on_road(&self) -> Option<ColorIndex> {
        self.current_cell().road().block
    }

    pub(crate) fn pick_up_block_if_exists(&mut self) {


        log_trace!("pick_up_block_if_exists");

        //pick up a block if it exists.  Note a van can pick up a box of any color
        if let Some(block_color) = self.get_current_block_on_road() {
            log_trace!("Rolled on a block of color {:?}", block_color);

            if let Some(i) = self.vans[self.current_van_index.0].get_empty_slot() {
                log_trace!("Van picked up a block of color {:?}", block_color);
                self.vans[self.current_van_index.0].boxes[i] = Some(block_color);
                self.current_cell_mut().mut_road().block = None;
            }
        }
    }

    pub(crate) fn press_button_if_exists(&mut self) {

        if let Road{ button_snapshot: Some(button), ..} = self.current_cell_mut().mut_road()
        {
            log_trace!("Van on a button {:?}", button);
            if !button.is_pressed  {                

                assert!(!button.was_pressed_this_tick);

                button.was_pressed_this_tick = true;
            }
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

    pub (crate) fn get_cur_used_mask(&self) -> u8 {
        match &self.tiles[self.current_cell_index().0] {
            TileRoad( Road{ used_mask, van_snapshot, .. } ) => {
                assert!(van_snapshot.is_some());
                *used_mask
            },
            TileBridge( Bridge{ used_van_index, .. } ) => if used_van_index.is_some() { (1 << 4) - 1} else {0},
            _ => panic!("Van not on road or bridge")
        }
    }

    pub (crate) fn filter_map_by_can_have_van<'a>(
        &self,
        fixed_choice_opt: &Option<ChoiceOverride>,
        adj_square_info: &'a AdjSquareInfo) -> Option<&'a AdjSquareInfo> 
        
    {

        let direction_index = adj_square_info.direction_index;
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

        //remove van & set used mask
        match &mut self.tiles[van_cell_index.0] {
            TileRoad( current_tile_road ) => 
            {
                assert!(current_tile_road.van_snapshot.is_some());
                assert_eq!(current_tile_road.used_mask & adj_info.direction as u8, 0);

                current_tile_road.van_snapshot = None;
                current_tile_road.used_mask |= adj_info.direction as u8;
                current_tile_road.used_tick[adj_info.direction_index] = Some(self.tick);
                current_tile_road.used_van_index[adj_info.direction_index] = Some(self.current_van_index);

            },
            TileBridge(current_tile_bridge) => 
            {
                assert!(!current_tile_bridge.is_up);
                
                //These are set when moved to
                assert_eq!(current_tile_bridge.used_van_index, Some(self.current_van_index));
                assert_eq!(current_tile_bridge.used_tick, Some(self.tick - 1));

                assert!(current_tile_bridge.van_snapshot.is_some());

                //current_tile_bridge.used_van_index = Some(self.current_van_index);
                //current_tile_bridge.used_tick = Some(self.tick);
                current_tile_bridge.van_snapshot = None;
            },
            _ => {
                panic!("Not a road or bridge");
            }
        }


        //add van to next square

        let moving_to_cell_index =adj_info.cell_index;

        {
            let van = self.current_van_mut();
            van.cell_index = moving_to_cell_index;
            van.tick += 1;
        }

        match &mut self.tiles[moving_to_cell_index.0] {
            TileRoad( next_road ) => 
            {
                //keep a history
                next_road.van_snapshot = Some(self.vans[self.current_van_index.0].clone());

                //we cant do a U turn
                next_road.used_mask |= adj_info.direction.opposite() as u8;

                let opp_dir_index = ALL_DIRECTIONS.iter().position(|d| d == &adj_info.direction.opposite()).unwrap();

                next_road.used_van_index[opp_dir_index] = Some(self.current_van_index);
                next_road.used_tick[opp_dir_index] = Some( self.tick );
            },
            TileBridge( next_bridge) => 
            {
                assert!(next_bridge.van_snapshot.is_none());
                assert!(next_bridge.used_van_index.is_none());
                assert!(next_bridge.used_tick.is_none());

                next_bridge.van_snapshot = Some(self.vans[self.current_van_index.0].clone());

                //we cant do a U turn
                next_bridge.used_van_index = Some(self.current_van_index);

                next_bridge.used_tick = Some( self.tick );
            },
            _ => panic!("not a road or bridge")
        }
    }





}