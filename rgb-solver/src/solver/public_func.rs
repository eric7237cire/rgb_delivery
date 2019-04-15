use wasm_bindgen::prelude::*;



use crate::solver::struct_defs::*;
use crate::solver::utils::*;


#[wasm_bindgen]
    pub fn get_colors() -> JsValue {
        JsValue::from_serde( &build_color_list()).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_tiles() -> JsValue {
        JsValue::from_serde(&build_tile_list()).unwrap()
    }


fn build_tile_list() -> [Tile; 4] {
    let mut tile_list = [
                Tile { label: "Road".to_string(), ..Default::default()},
               /* Tile { label: "Road Used NE".to_string(), ..Default::default()},
                Tile { label: "Road Used NW".to_string(), ..Default::default()},
        Tile { label: "Road Used SW".to_string(), ..Default::default()},
        Tile { label: "Road Used SE".to_string(), ..Default::default()},
        Tile { label: "Road Used NS".to_string(), ..Default::default()},
        Tile { label: "Road Used EW".to_string(), ..Default::default()},*/
                Tile { label: "Block".to_string(), ..Default::default()},
                Tile { label: "Warehouse".to_string(), ..Default::default()},
                Tile { label: VAN_LABEL.to_string(), ..Default::default()},

                ];

    for (idx, t) in tile_list.iter_mut().enumerate() {
        t.tile_index = idx;
    }


        log!(
                    "Built tile list {:?} ",
                    tile_list
                );

    tile_list
}


fn build_color_list() -> [Color; 4] {
    let mut color_list = [
                Color{ label: "White".to_string(), red: 230, green: 230, blue: 230, ..Default::default()},
                Color{ label: "Red".to_string(), red: 255, green: 0, blue: 0, ..Default::default()},
                Color{ label: "Yellow".to_string(), red: 255, green: 255, blue: 0, ..Default::default()},
                Color{ label: "Blue".to_string(), red: 50, green: 50, blue: 255, ..Default::default()},

/*
 colors = [ new Color("rgb(255,0,0)","Red"), new Color("rgb(255,255,0)","Yellow"),
  new Color("rgb(50,50,255)", "Blue"), new Color("rgb(230,230,230)", "White") ] ;
  */
                ];

    for (idx, c) in color_list.iter_mut().enumerate() {
        c.color_index = idx;
    }

    color_list
}