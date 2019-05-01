
pub mod color;
pub mod tile;

mod van;
mod road;
mod bridge;
mod grid_connections;
mod direction;

use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;

pub use self::van::Van;
use crate::solver::grid_state::GridState;
pub use self::color::ColorIndex;
pub use self::tile::TileEnum;

pub use self::road::Road;
pub use self::bridge::Bridge;

pub (crate) use self::direction::{Direction, ALL_DIRECTIONS,get_adjacent_index};

pub use self::road::{NavigableTileStatic, NavigableTileDynamic};

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

    //set by the init calc
    #[serde(skip)]
    pub cell_index: CellIndex,

    #[serde(skip)]
    pub(crate) was_pressed_this_tick: bool
}

pub fn all_mask() -> u8 {
    15
}


#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Warehouse {
    pub color: ColorIndex,
    pub is_filled: bool
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

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct ChoiceOverride {
    pub row_index: usize,
    pub col_index: usize,
    pub van_index: Option<VanIndex>,
    pub direction_index: usize,

    pub tick: Option<usize>
}


//internal helper
#[derive(Debug)]
pub(crate) struct AdjSquareInfo {
    pub(crate) direction: Direction,
    pub(crate) cell_index: CellIndex,
}

#[derive(Default, Serialize, Deserialize, TypescriptDefinition)]
pub struct CalculationResponse {
    pub error_message: Option<String>,
    pub grid_state: Option<GridState>,
    pub iteration_count: usize,
    pub success: bool
}