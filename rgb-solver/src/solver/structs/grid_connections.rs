use crate::solver::structs::{Direction, CellIndex, ALL_DIRECTIONS};
//use crate::solver::structs::Direction::*;
use crate::solver::structs::direction::get_adjacent_index;

#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct GridConnections {
    pub is_connected: Vec<u8>,
    num_rows: usize,
    num_cols: usize,
}

#[derive(Default)]
pub struct GridConnectionsStaticInfo {
    pub adj_info: Vec<[Option<AdjSquareInfo>; 4]>,

}

#[derive(Debug,Clone)]
pub struct AdjSquareInfo {
    pub direction: Direction,
    pub cell_index: CellIndex
}


impl GridConnections {
    pub fn build_static_info(&self) -> GridConnectionsStaticInfo {
        GridConnectionsStaticInfo {
            adj_info: (0..self.num_rows * self.num_cols).map(|idx| {
                let mut v = ALL_DIRECTIONS.iter().map(|d|

                    if let Some(adj_idx) = get_adjacent_index(CellIndex(idx), self.num_rows, self.num_cols, *d) {
                        Some(AdjSquareInfo { cell_index: adj_idx, direction: *d })
                    } else { None }
                );
                let array: [Option<AdjSquareInfo>; 4] = [v.next().unwrap(),v.next().unwrap(),v.next().unwrap(),v.next().unwrap()];
                array
            }).collect(),
        }
    }

    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        GridConnections {
            is_connected: vec![0;
                num_rows * num_cols],
            num_rows,
            num_cols,
        }
    }

    //#[inline(always)]
    pub fn get_adjacent_square_indexes<'a>(&'a self,
                                           static_info: &'a GridConnectionsStaticInfo,
                                           cell_index: CellIndex,
    ) -> impl Iterator<Item=&'a AdjSquareInfo>
    {
        ALL_DIRECTIONS.iter().filter_map(move |dir|
            {
                if self.is_connected(cell_index, *dir) {
                      static_info.adj_info[cell_index.0][*dir as usize].as_ref()
                } else {
                    None
                }

            })
    }

    //#[inline(always)]
    pub fn is_connected(&self, cell_index: CellIndex, dir: Direction) -> bool
    {
        self.is_connected[cell_index.0] & (1 << dir as u8) > 0
    }

    //#[inline(always)]
    pub fn set_is_connected(&mut self, cell_index: CellIndex, dir: Direction, connected: bool)
    {
        if connected {
            self.is_connected[cell_index.0] |= 1 << dir as u8;
        } else {
            self.is_connected[cell_index.0] &= !(1 << dir as u8);
        }

    }



}

#[cfg(test)]
mod tests {

}