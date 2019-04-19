use wasm_bindgen::prelude::*;

use crate::solver::struct_defs::*;
use std::collections::vec_deque::VecDeque;
use super::utils;
use crate::solver::struct_defs::TileEnum::{TileRoad,TileWarehouse};
use crate::solver::utils::set_panic_hook;
use crate::solver::grid_state::GridState;

#[wasm_bindgen]
    pub fn get_colors() -> JsValue {
        JsValue::from_serde( &build_color_list()).unwrap()
    }




pub(crate) fn build_color_list() -> Vec<Color> {
    let mut color_list = vec![
        Color{ label: "White".to_string(), red: 230, green: 230, blue: 230, ..Default::default()},
        Color{ label: "Red".to_string(), red: 255, green: 0, blue: 0, ..Default::default()},
        Color{ label: "Yellow".to_string(), red: 255, green: 255, blue: 0, ..Default::default()},
        Color{ label: "Blue".to_string(), red: 50, green: 50, blue: 255, ..Default::default()},
        Color{ label: "Green".to_string(), red: 0, green: 255, blue: 0, ..Default::default()},
        Color{ label: "Purple".to_string(), red: 167, green: 152, blue: 253, ..Default::default()},

/*
 colors = [ new Color("rgb(255,0,0)","Red"), new Color("rgb(255,255,0)","Yellow"),
  new Color("rgb(50,50,255)", "Blue"), new Color("rgb(230,230,230)", "White") ] ;
  */
                ];

    for (idx, c) in color_list.iter_mut().enumerate() {
        c.color_index = ColorIndex(idx);
    }

    color_list
}



#[wasm_bindgen()]
impl Universe {
    // ...

    pub fn new(h: usize, w: usize) -> Universe {
        log!(
            "Building a new Grid.  [{}, {}] ",
            w,
            h
        );

        utils::set_panic_hook();

        let width = w;
        let height = h;


        let tiles: Vec<TileEnum> = (0..width * height)
            .map(|_| {
                TileEnum::Empty
            })
            .collect();

        //let cl = build_color_list();
        /*
                cells[0] =  CellData{row_index: 0, col_index: 0, tile: TileRoad(
                    Road {used_mask: 45, block: None, van: None})
                };*/
        /*cells[1] =  CellData{row_index: 0, col_index: 0, tile: Warehouse {color: cl[2].clone(), is_filled: true},
            van: Some( Van{ boxes: [None, Some(cl[0].clone()), Some(cl[3].clone())] } ) } );*/

        Universe {
            data: GridState {
                width,
                height,
                tiles,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }


    pub fn get_data(&self) -> JsValue {
        JsValue::from_serde(&self.data).unwrap()
    }


    pub fn set_overrides(&mut self, choice_override_list: &JsValue) {
        let lo : Vec<ChoiceOverride> = choice_override_list.into_serde().unwrap();

        self.choice_override_list = lo.clone();

        log!("Set override list {:?}", lo);
    }

    pub fn init_calculate(&mut self) {

        set_panic_hook();


        self.queue = VecDeque::new();

        self.iter_count = 0;

        self.success = None;

        self.data.vans = self.initial_van_list();

        self.data.warehouses_remaining = self.data.tiles.iter().filter( |t| {
            if let TileWarehouse(_) = t {
                true
            } else {
                false 
            }
        }).count();

        //we increment on pop, so...
        self.data.current_van_index = VanIndex( self.initial_van_list().len() - 1 );


        //reset road history
        for road in self.data.tiles.iter_mut().filter_map( |t| {
            if let TileRoad(road) = t {
                Some(road)
            } else {
                None
            }
        }) {
            road.used_van_index = Default::default();
            road.used_mask = Default::default();
        }

        self.queue.push_back(self.data.clone());
    }

    pub fn next_calculate(&mut self) -> JsValue {
        let v = self.process_queue_item();
        JsValue::from_serde(&v).unwrap()
    }

    pub fn next_batch_calculate(&mut self, repeat_count: usize) -> JsValue {

        log!("Batch calculate, repeat count: {}", repeat_count);

        if repeat_count > 350000 {
            return JsValue::from_serde(&self.data).unwrap();
        }

        for _i in 0..repeat_count {
            self.process_queue_item();
            if self.success.is_some() {
                return JsValue::from_serde(&self.success).unwrap()
            }
        }

        return self.next_calculate();

    }


    pub fn set_square(&mut self, tile_val: &JsValue) {
        let cell: CellData = tile_val.into_serde().unwrap();

        let idx: usize = cell.row_index * self.data.width + cell.col_index;

        /*log!(
            "Received Row/Col [{}, {}] = idx [{}].  Tile {:?}",
            tile.row_index,
            tile.col_index,
            idx,
            tile
        );*/

        if idx < self.data.tiles.len() {
            self.data.tiles[idx] = cell.tile;
        } else {
            log!(
                "Out of bounds, ignoring"
            );
        }
    }
}