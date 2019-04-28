use crate::solver::struct_defs::{TileEnum, Road, Bridge};
use crate::solver::struct_defs::TileEnum::{TileRoad, TileBridge};
//use crate::solver::struct_defs::Directions::{NORTH, EAST, WEST, SOUTH};

impl TileEnum {
    pub(crate) fn get_connection_mask(&self) -> Option<u8> {
        match self {
            TileRoad(Road { connection_mask, .. }) => Some(*connection_mask),
            TileBridge(Bridge { connection_mask, .. }) => Some(*connection_mask),
            _ => None
        }
    }
}
