#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Direction {
    NORTH = 1,
    EAST = 2,
    SOUTH = 4,
    WEST = 8,
}

use self::Direction::*;
use crate::solver::structs::CellIndex;

impl Direction {
    pub(crate) fn opposite(&self) -> Direction {
        match self {
            NORTH => SOUTH,
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST
        }
    }


}

pub fn opposite_dir_index(dir_index: usize) -> usize {
        match dir_index {
            0 => 2,
            1 => 3,
            2 => 0,
            3 => 1,
            _ => panic!("Not a valid dir index")
        }
    }

pub fn get_adjacent_index(square_index: CellIndex, grid_height: usize, grid_width: usize, dir: Direction) -> Option<CellIndex> {

    let (cell_row_index, cell_col_index) = square_index.to_row_col(grid_width);

    match dir {
        NORTH => {
            if cell_row_index == 0 {
                None
            } else {
                Some(CellIndex(square_index.0 - grid_width))
            }
        }
        SOUTH => {
            if cell_row_index >= grid_height - 1 {
                None
            } else {
                Some(CellIndex(square_index.0 + grid_width))
            }
        }
        EAST => {
            if cell_col_index >= grid_width - 1 {
                None
            } else {
                Some(CellIndex(square_index.0 + 1))
            }
        }
        WEST => {
            if cell_col_index == 0 {
                None
            } else {
                Some(CellIndex(square_index.0 - 1))
            }
        }
    }
}

pub const ALL_DIRECTIONS: [Direction; 4] = [NORTH, EAST, SOUTH, WEST];
