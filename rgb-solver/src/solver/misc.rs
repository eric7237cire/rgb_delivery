use crate::solver::struct_defs::*;

use crate::solver::struct_defs::TileEnum::{ TileWarehouse, };

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

