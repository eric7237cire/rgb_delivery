use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;

use std::collections::vec_deque::VecDeque;

use super::van::Van;
use crate::solver::grid_state::GridState;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default, Hash)]
pub struct Color {
    pub label: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,

    pub color_index: ColorIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default, Hash)]
pub struct ColorIndex(pub usize);

impl ColorIndex {
    pub fn is_white(&self) -> bool {
        return self.0 == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default, Hash)]
pub struct CellIndex(pub usize);

impl From<usize> for CellIndex {
    fn from(index: usize) -> Self {
        CellIndex(index)
    }
}

impl CellIndex {
    pub(crate) fn to_row_col(&self, width: usize) -> (usize,usize) {
        ( self.0 / width,
            self.0 % width )
    }
}

//If 3 vats, indexes will be 0,1,2
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default, Hash)]
pub struct VanIndex(pub usize);

impl From<usize> for VanIndex {
    fn from(index: usize) -> Self {
        VanIndex(index)
    }
}
impl From<VanIndex> for usize {
    fn from(index: VanIndex) -> Self {
        index.0 
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Button  {
    pub is_pressed: bool,
    pub color: ColorIndex,
    pub cell_index: CellIndex,

    #[serde(skip)]
    pub(crate) was_pressed_this_tick: bool
}

#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Road {
    pub used_mask: u8,

    #[serde(default)]
    pub used_van_index: [Option<VanIndex>; 4],

    #[serde(default)]
    pub used_tick: [Option<usize>; 4],

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "block")]
    pub block: Option<ColorIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "van")]
    pub van_snapshot: Option<Van>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "button")]
    pub button_snapshot: Option<Button>
}

#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Bridge {

    //once van leaves, is set to the van that used this bridge
    #[serde(default)]
    pub used_van_index: Option<VanIndex>,

    #[serde(default)]
    pub used_tick: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub van_snapshot: Option<Van>,

    pub is_up: bool,

    pub color: ColorIndex,

    pub cell_index: CellIndex
}

#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Warehouse {
    pub color: ColorIndex,
    pub is_filled: bool
}

#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum TileEnum {
    TileRoad(Road),
    TileWarehouse (Warehouse),
    TileBridge(Bridge),
    Empty
}


#[derive(Clone, Debug, Serialize, Deserialize, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct CellData {
    pub tile: TileEnum,

    pub row_index: usize,
    pub col_index: usize,
}

impl Default for CellData {
    fn default() -> Self {
        Self { tile: TileEnum::Empty, row_index: 0, col_index: 0 }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct ChoiceOverride {
    pub row_index: usize,
    pub col_index: usize,
    pub van_index: VanIndex,
    pub direction_index: usize,

    pub tick: Option<usize>
}


#[cfg_attr( not(target_arch = "x86_64"), wasm_bindgen())]
#[derive(Default)]
pub struct Universe {
    pub(crate) data: GridState,

    pub(crate) choice_override_list: Vec<ChoiceOverride>,

    //below are used for calculating
    pub(crate) queue: VecDeque<GridState>,

    pub(crate) success: Option<GridState>,
    pub(crate) current_calc_state: Option<GridState>,
    pub(crate) iter_count: usize
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Directions {
    NORTH = 1,
    EAST = 2,
    SOUTH = 4,
    WEST = 8,
}

//internal helper
#[derive(Debug)]
pub(crate) struct AdjSquareInfo {
    pub(crate) direction: Directions,
    pub(crate) cell_index: CellIndex,
    pub(crate) direction_index: usize
}
