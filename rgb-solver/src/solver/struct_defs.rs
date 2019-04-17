use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;
//use crate::solver::struct_defs::TileEnum::Empty;
//use crate::solver::utils::VAN_LABEL;

use serde::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default, Hash)]
pub struct Color {
    pub label: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,

    pub color_index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Van {
    pub boxes: [Option<Color>; 3],
    pub color: Color,
    pub is_done: bool,

    #[serde(skip)]
    pub(crate) tick: usize
}

#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Road {
    pub used_mask: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "block")]
    pub block: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub van: Option<Van>,
}


#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum TileEnum {
    TileRoad(Road),
    Warehouse { color: Color, is_filled: bool },
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

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Default, Hash, Eq, PartialEq)]
pub struct UniverseData {
    pub width: usize,
    pub height: usize,

    pub cells: Vec<CellData>,

    #[serde(skip)]
    pub(crate) tick: usize
}

#[wasm_bindgen()]
pub struct Universe {
    pub(crate) data: UniverseData
}

