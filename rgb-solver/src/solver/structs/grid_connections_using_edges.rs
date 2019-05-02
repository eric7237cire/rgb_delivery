use bitvec::{BigEndian, BitVec};
use crate::solver::structs::{Direction, CellIndex, ALL_DIRECTIONS};
use crate::solver::structs::Direction::*;
use crate::solver::structs::direction::get_adjacent_index;

#[derive(Clone, Default)]
pub struct GridConnections {
    is_connected: BitVec<BigEndian, u8>,
    num_rows: usize,
    num_cols: usize,
}

#[derive(Default)]
pub struct GridConnectionsStaticInfo {
    pub edge_indexes: Vec<[Option<usize>; 4]>,
    pub adj_info: Vec<[Option<AdjSquareInfo>; 4]>,

}

#[derive(Debug)]
pub struct AdjSquareInfo {
    pub direction: Direction,
    pub cell_index: CellIndex,
}


impl GridConnections {
    pub fn build_static_info(&self) -> GridConnectionsStaticInfo {
        GridConnectionsStaticInfo {
            edge_indexes: (0..self.num_rows * self.num_cols).map(|idx| {
                let v: Vec<_> = ALL_DIRECTIONS.iter().map(|d|
                    self.get_edge_index(CellIndex(idx), *d)).collect();

                let mut array: [Option<usize>; 4] = [None;4];
                array.copy_from_slice(&v[0..4]);
                array
            }).collect(),
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
            is_connected: bitvec![0;
                num_rows * (num_cols - 1) + num_cols * (num_rows - 1)],
            num_rows,
            num_cols,
        }
    }

    #[inline(always)]
    pub fn get_adjacent_square_indexes<'a>(&'a self,
                                           static_info: &'a GridConnectionsStaticInfo,
                                           cell_index: CellIndex,
    ) -> impl Iterator<Item=&'a AdjSquareInfo>
    {
        static_info.edge_indexes[cell_index.0].iter().enumerate().filter_map(move |(dir_idx, ei_opt)|
            {
                if let Some(ei) = ei_opt {
                    if self.is_connected[*ei] {
                        static_info.adj_info[cell_index.0][dir_idx].as_ref()
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }

    #[inline(always)]
    pub fn is_connected(&self, static_info: &GridConnectionsStaticInfo, cell_index: CellIndex, dir: Direction) -> bool
    {
        if let Some(ei) = static_info.edge_indexes[cell_index.0][dir as usize] {
            self.is_connected[ei]
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn set_is_connected(&mut self, static_info: &GridConnectionsStaticInfo, cell_index: CellIndex, dir: Direction, connected: bool)
    {
        if let Some(ei) = static_info.edge_indexes[cell_index.0][dir as usize] {
            self.is_connected.set(ei, connected);
        } else {
            //do nothing
        }
    }

    fn get_edge_index(&self, cell_index: CellIndex, dir: Direction) -> Option<usize>
    {
        let (cell_row_index, cell_col_index) = cell_index.to_row_col(self.num_cols);
        let edges_per_row = 2 * self.num_cols - 1;

        match dir {
            NORTH => {
                if cell_row_index == 0 {
                    None
                } else {
                    Some((self.num_cols - 1) + cell_col_index + edges_per_row * (cell_row_index - 1))
                }
            }
            SOUTH => {
                if cell_row_index >= self.num_rows - 1 {
                    None
                } else {
                    Some((self.num_cols - 1) + cell_col_index + edges_per_row * (cell_row_index))
                }
            }
            EAST => {
                if cell_col_index >= self.num_cols - 1 {
                    None
                } else {
                    Some(cell_col_index + edges_per_row * cell_row_index)
                }
            }
            WEST => {
                if cell_col_index == 0 {
                    None
                } else {
                    Some(cell_col_index - 1 + edges_per_row * cell_row_index)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::solver::structs::direction::Direction::*;
    use crate::solver::structs::{CellIndex, ALL_DIRECTIONS};

    #[test]
    fn test_get_index() {
        let dims = (3, 3);
        let gc = GridConnections::new(3, 3);
        let mut row = 0;
        let mut col = 0;
        let mut index = CellIndex(row * dims.1 + col);
        assert_eq!(gc.get_edge_index(index, NORTH), None);
        assert_eq!(gc.get_edge_index(index, WEST), None);
        assert_eq!(gc.get_edge_index(index, EAST), Some(0));
        assert_eq!(gc.get_edge_index(index, SOUTH), Some(2));

        col = 1;
        index = CellIndex(row * dims.1 + col);
        assert_eq!(gc.get_edge_index(index, NORTH), None);
        assert_eq!(gc.get_edge_index(index, WEST), Some(0));
        assert_eq!(gc.get_edge_index(index, EAST), Some(1));
        assert_eq!(gc.get_edge_index(index, SOUTH), Some(3));

        row = 1;
        index = CellIndex(row * dims.1 + col);
        assert_eq!(gc.get_edge_index(index, NORTH), Some(3));
        assert_eq!(gc.get_edge_index(index, WEST), Some(5));
        assert_eq!(gc.get_edge_index(index, EAST), Some(6));
        assert_eq!(gc.get_edge_index(index, SOUTH), Some(8));

        row = 2;
        col = 2;
        index = CellIndex(row * dims.1 + col);
        assert_eq!(gc.get_edge_index(index, NORTH), Some(9));
        assert_eq!(gc.get_edge_index(index, WEST), Some(11));
        assert_eq!(gc.get_edge_index(index, EAST), None);
        assert_eq!(gc.get_edge_index(index, SOUTH), None);


        //assert_eq!(bv.as_slice(), &[0b0101_0000, 0b1111_0000]);
    }

    #[test]
    fn test_get_index_adj() {
        for num_rows in 1..=11 {
            for num_cols in 1..=11 {
                let gc = GridConnections::new(num_rows, num_cols);
                for index in 0..num_cols * num_rows {
                    let cell_index = CellIndex(index);

                    for dir in ALL_DIRECTIONS.iter() {
                        let adj = get_adjacent_index(cell_index, num_rows, num_cols, *dir);

                        let ei = gc.get_edge_index(cell_index, *dir);

                        assert_eq!(adj.is_none(), ei.is_none());

                        if let Some(adj) = adj {
                            let ei2 = gc.get_edge_index(adj, dir.opposite());

                            assert_eq!(ei, ei2);
                        }
                    }
                }

                //test max
                if num_cols > 1 {
                    assert_eq!(gc.get_edge_index(CellIndex(num_rows * num_cols - 1), WEST), Some(num_rows * (num_cols - 1) + num_cols * (num_rows - 1) - 1));
                }
            }
        }
    }
}