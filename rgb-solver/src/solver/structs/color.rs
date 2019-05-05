
use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default, Hash)]
pub struct Color {
    pub label: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,

    pub color_index: ColorIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, TypescriptDefinition, Default, Hash, Ord, PartialOrd)]
pub struct ColorIndex(pub usize);

impl ColorIndex {
    pub fn is_white(&self) -> bool {
        return self.0 == 0
    }
}