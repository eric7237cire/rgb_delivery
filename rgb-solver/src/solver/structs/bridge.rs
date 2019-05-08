use crate::solver::structs::color::ColorIndex;
use crate::solver::structs::van::Van;
use crate::solver::structs::{CellIndex, NavigableTileDynamic, NavigableTileStatic, VanIndex};
use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;

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

    //set by init calc
    #[serde(skip)]
    pub cell_index: CellIndex,

    #[serde(flatten)]
    pub static_attrs: NavigableTileStatic,

    #[serde(flatten)]
    pub dynamic_attrs: NavigableTileDynamic,
}
