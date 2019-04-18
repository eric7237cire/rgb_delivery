use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;
use std::collections::HashSet;
use std::collections::vec_deque::VecDeque;

use super::van::Van;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default, Hash)]
pub struct Color {
    pub label: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,

    pub color_index: usize,
}


#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Road {
    pub used_mask: u8,

    #[serde(default)]
    pub used_van_index: [Option<usize>; 4],

    #[serde(default)]
    pub used_tick: [Option<usize>; 4],

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "block")]
    pub block: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub van: Option<Van>,
}

#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Warehouse {
    pub color: Color,
    pub is_filled: bool
}

#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum TileEnum {
    TileRoad(Road),
    TileWarehouse (Warehouse),
    Empty,
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
    pub van_index: usize,
    pub direction_index: usize,

    pub tick: Option<usize>
}

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Default, Hash, Eq, PartialEq)]
pub struct UniverseData {
    pub width: usize,
    pub height: usize,

    pub cells: Vec<CellData>,

    //Js=>Rust will ignore this
    #[serde(skip_deserializing)]
    pub(crate) tick: usize,

    #[serde(skip_deserializing)]
    pub(crate) vans: Vec<Van>,
    #[serde(skip)]
    pub(crate) current_van_index: usize

}

#[cfg_attr( not(target_arch = "x86_64"), wasm_bindgen())]
#[derive(Default)]
pub struct Universe {
    pub(crate) data: UniverseData,

    pub(crate) choice_override_list: Vec<ChoiceOverride>,

    //below are used for calculating
    pub(crate) seen:  HashSet<UniverseData>,
    pub(crate) queue: VecDeque<UniverseData>,

    pub(crate) success: Option<UniverseData>,
    pub(crate) current_calc_state: Option<UniverseData>,
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
    pub(crate) cell_index: usize,
    pub(crate) direction_index: usize
}
