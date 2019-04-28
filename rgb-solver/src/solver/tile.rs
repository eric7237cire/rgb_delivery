use crate::solver::struct_defs::{TileEnum, RoadConnection, Road, Bridge, Directions};
use crate::solver::struct_defs::TileEnum::{TileRoad, TileBridge};
use crate::solver::struct_defs::Directions::{NORTH, EAST, WEST, SOUTH};

impl Tile {
    pub(crate) fn get_road_connection(&self) -> Option<RoadConnection> {
        match self {
            TileRoad(Road { connections, .. }) => Some(*connections),
            TileBridge(Bridge { connections, .. }) => Some(*connections),
            _ => None
        }
    }
}

impl RoadConnection {
    fn is_ok(&self, dir: Directions) -> bool {
        match self {
            AllDirections => true,
            NorthSouth=> dir == NORTH || dir == SOUTH,
            EastWest => dir == EAST || dir == WEST
        }
    }
}