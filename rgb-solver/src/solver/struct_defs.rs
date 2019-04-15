use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default)]
pub struct Color {
    pub label: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,

    pub color_index: usize
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default)]
pub struct Tile {
    pub label: String,
    pub tile_index: usize
}

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition)]
pub struct CellData {
    pub tile: Tile,
    pub color: Option<Color>,

    pub used_mask: u8,

    pub row_index: usize,
    pub col_index: usize,
}

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Default)]
pub struct UniverseData {
    pub width: usize,
    pub height: usize,

    pub cells: Vec<Option<CellData>>,
}

#[wasm_bindgen()]
pub struct Universe {
    pub(crate) data: UniverseData
}

