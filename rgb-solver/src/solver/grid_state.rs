use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;
use crate::solver::struct_defs::{CellData, Warehouse, ColorIndex};
use crate::solver::van::Van;
use crate::solver::struct_defs::TileEnum::TileWarehouse;

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Default, Hash, Eq, PartialEq)]
pub struct GridState {
    pub width: usize,
    pub height: usize,

    pub cells: Vec<CellData>,

    //Js=>Rust will ignore this
    #[serde(skip_deserializing)]
    pub(crate) tick: usize,

    #[serde(skip_deserializing)]
    pub(crate) vans: Vec<Van>,
    #[serde(skip)]
    pub(crate) current_van_index: usize,
}

pub enum CanDropOff {
    NoFail,
    NoOk,
    YesOK,
}

impl GridState {
    pub(crate) fn increment_current_van_index(&mut self) {
        if self.current_van_index == self.vans.len() - 1 {
            log_trace!("Advancing a tick {} => {}", self.tick, self.tick + 1);

            self.tick += 1;
            self.current_van_index = 0;
        } else {
            self.current_van_index += 1;
        }
    }

    pub(crate) fn check_success(&self) -> bool {
        self.cells.iter().all(|cell| {
            match cell.tile {
                TileWarehouse(Warehouse { is_filled, .. }) => {
                    is_filled
                }
                _ => true
            }
        })
    }

    pub(crate) fn current_cell_index(&self) -> usize {
        self.vans[self.current_van_index].cell_index
    }
    pub(crate) fn current_cell_mut(&mut self) -> &mut CellData {
        &mut self.cells[ self.vans[self.current_van_index].cell_index ]
    }
    pub(crate) fn current_cell(&self) -> &CellData {
        &self.cells[ self.vans[self.current_van_index].cell_index ]
    }

    fn get_current_block_on_road(&self) -> Option<ColorIndex> {
        self.current_cell().tile.road().block
    }

    pub(crate) fn pick_up_block_if_exists(&mut self) {


        log_trace!("Processing van at {}, {}.  Van: {:?}",
             cur_row_index, cur_col_index,
             cur_road.van);

        //pick up a block if it exists.  Note a van can pick up a box of any color
        if let Some(block_color) = self.get_current_block_on_road() {
            log_trace!("Rolled on a block of color {:?}", block);

            if let Some(i) = self.vans[self.current_van_index].get_empty_slot() {
                log_trace!("Van picked up a block of color {:?}", block);
                self.vans[self.current_van_index].boxes[i] = Some(block_color);
                self.current_cell_mut().tile.mut_road().block = None;
            }
        }
    }

    ///
    fn empty_warehouse_color(&self) -> Option<ColorIndex> {
        let current_cell_index = self.current_cell_index();

        //on first row
        if current_cell_index < self.width {
            return None;
        }

        let north_tile = &self.cells[current_cell_index - self.width].tile;
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
                    if let Some(color) = self.vans[self.current_van_index].get_top_box() {
                        color
                    } else {
                        //warehouse would be unfillable
                        return Err(CanDropOff::NoFail);
                    }
                };

            if top_block_color_index == warehouse_color_index && (
                self.vans[self.current_van_index].color.is_white()
                    || self.vans[self.current_van_index].color == warehouse_color_index)
            {

                //pop the box
                self.vans[self.current_van_index].clear_top_box();


                //set warehouse to filled
                {
                    let north_index = self.current_cell_index() - self.width;
                    self.cells[ north_index ].tile.mut_warehouse().is_filled = true;
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
}