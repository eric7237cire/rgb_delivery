
use wasm_bindgen::prelude::*;

use crate::solver::struct_defs::*;

#[wasm_bindgen]
    pub fn get_colors() -> JsValue {
        JsValue::from_serde( &build_color_list()).unwrap()
    }


pub(crate) const NUM_COLORS:usize = 9;
pub(crate) const WHITE_COLOR_INDEX:usize = 0;

pub(crate) fn build_color_list() -> Vec<Color> {
    let mut color_list = vec![
        Color{ label: "White".to_string(), red: 230, green: 230, blue: 230, ..Default::default()},
        Color{ label: "Red".to_string(), red: 255, green: 0, blue: 0, ..Default::default()},
        Color{ label: "Yellow".to_string(), red: 255, green: 255, blue: 0, ..Default::default()},
        Color{ label: "Blue".to_string(), red: 50, green: 50, blue: 255, ..Default::default()},
        Color{ label: "Green".to_string(), red: 0, green: 255, blue: 0, ..Default::default()},
        Color{ label: "Purple".to_string(), red: 167, green: 152, blue: 253, ..Default::default()},

        Color{ label: "PinkPurple".to_string(), red: 185, green: 95, blue: 167, ..Default::default()},
        Color{ label: "Teal".to_string(), red: 169, green: 162, blue: 107, ..Default::default()},
        Color{ label: "Orange".to_string(), red: 219, green: 103, blue: 0, ..Default::default()},

/*
 colors = [ new Color("rgb(255,0,0)","Red"), new Color("rgb(255,255,0)","Yellow"),
  new Color("rgb(50,50,255)", "Blue"), new Color("rgb(230,230,230)", "White") ] ;
  */
                ];

    for (idx, c) in color_list.iter_mut().enumerate() {
        c.color_index = ColorIndex(idx);
    }

    assert_eq!(color_list.len(), NUM_COLORS);

    color_list
}


