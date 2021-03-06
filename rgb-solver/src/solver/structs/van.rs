use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;
use super::{ColorIndex, CellIndex};

#[derive(Clone, Debug, Serialize, Deserialize, TypescriptDefinition, Hash, Eq, PartialEq)]
pub struct Van {
    pub boxes: [Option<ColorIndex>; 3],
    pub color: ColorIndex,
    pub is_done: bool,

    #[serde(skip)]
    pub(crate) tick: usize,
    #[serde(default)]
    pub(crate) cell_index: CellIndex 
}


impl Van {

    //0 is the bottom of the stack

    pub(crate) fn get_top_box(&self) -> Option<ColorIndex> {
        for i in (0..=2).rev() {
            if !self.boxes[i].is_none() {
                return self.boxes[i];
            }
        }
        return None;
    }

    pub(crate) fn clear_top_box(&mut self) {
        for i in (0..=2).rev() {
            if !self.boxes[i].is_none() {
                self.boxes[i] = None;
                return;
            }
        }

        //did not clear a box
        panic!("Did not clear a box" );
    }

    pub(crate) fn get_empty_slot(&self) -> Option<usize> {
        for i in 0..=2 {
            if self.boxes[i].is_none() {
                return Some(i);
            }
        }
        return None;
    }
}