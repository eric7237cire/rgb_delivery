
use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;
use crate::solver::structs::van::Van;
use crate::solver::structs::{VanIndex, Button};
use crate::solver::structs::color::ColorIndex;

use super::all_mask;

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Debug, Hash, Eq, PartialEq)]
pub struct NavigableTileStatic {

    #[serde(default = "all_mask")]
    pub connection_mask: u8,

}

impl Default for NavigableTileStatic {
    fn default() -> Self {
        Self { connection_mask: 15 }
    }
}

#[derive(Clone, Serialize, Deserialize, TypescriptDefinition, Debug, Hash, Eq, PartialEq)]
pub struct NavigableTileDynamic {

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "van")]
    pub van_snapshot: Option<Van>,
}


#[derive(Clone, Serialize, Deserialize, Debug, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Road {

    //togglable thing that if activated, will remove the top block and put it back on the street
    #[serde(default)]
    pub has_popper: bool,

    #[serde(default)]
    pub used_van_index: [Option<VanIndex>; 4],

    #[serde(default)]
    pub used_tick: [Option<usize>; 4],

    #[serde(default)]
    pub used_popper_tick: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "block")]
    pub block: Option<ColorIndex>,



    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "button")]
    pub button_snapshot: Option<Button>,

    #[serde(flatten)]
    pub static_attrs: NavigableTileStatic,
    #[serde(flatten)]
    pub dynamic_attrs: NavigableTileDynamic
}
