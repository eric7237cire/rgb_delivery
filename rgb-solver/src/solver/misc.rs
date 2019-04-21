use crate::solver::struct_defs::*;

use crate::solver::struct_defs::TileEnum::{TileWarehouse, TileRoad, TileBridge};

//use crate::solver::public_func::build_color_list;

//use crate::solver::utils::VAN_LABEL;



impl Directions {
    pub(crate) fn opposite(&self) -> Directions {
        match self {
            NORTH => SOUTH,
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST
        }
    }
}

pub(crate) fn opposite_dir_index(dir_index: usize) -> usize {
        match dir_index {
            0 => 2,
            1 => 3,
            2 => 0,
            3 => 1,
            _ => panic!("Not a valid dir index")
        }
    }


use crate::solver::struct_defs::Directions::*;

use crate::solver::struct_defs::Warehouse;


pub (crate) const ALL_DIRECTIONS: [Directions; 4] = [NORTH, EAST, SOUTH, WEST];


impl TileEnum {
    pub(crate) fn mut_warehouse(&mut self) -> &mut Warehouse {
        match self {
            TileWarehouse(inner) => {
                return inner;
            }
            _ => panic!()
        }
    }
}


pub (crate) fn get_adjacent_index(square_index: CellIndex, grid_height: usize, grid_width: usize,  dir: Directions) -> Option<usize> {

    let (cell_row_index, cell_col_index) = square_index.to_row_col(grid_width);

    match dir {
        NORTH => {
            if cell_row_index == 0 {
                None
            } else {
                Some(square_index.0 - grid_width)
            }
        }
        SOUTH => {
            if cell_row_index >= grid_height - 1 {
                None
            } else {
                Some(square_index.0 + grid_width)
            }
        }
        EAST => {
            if cell_col_index >= grid_width - 1 {
                None
            } else {
                Some(square_index.0 + 1)
            }
        }
        WEST => {
            if cell_col_index == 0 {
                None
            } else {
                Some(square_index.0 - 1)
            }
        }
    }
}

pub (crate) fn is_tile_navigable(tile: &TileEnum) -> bool {
    match tile {
        TileRoad(_) => true,
        TileBridge(_) => true,
        _ => false
    }
}