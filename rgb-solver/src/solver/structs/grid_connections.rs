use bitvec::{BigEndian, BitVec};
use crate::solver::structs::{Direction, CellIndex, AdjSquareInfo, ALL_DIRECTIONS, get_adjacent_index};
use crate::solver::structs::Direction::*;

type NumRowCol = (usize, usize);
//type RowCol = (usize,usize);

pub struct GridConnections {
    is_connected: BitVec<BigEndian, u8>
}

impl GridConnections {

    pub (crate) fn get_adjacent_square_indexes(&self, row_cols: NumRowCol, cell_index: CellIndex,
                                   ) -> impl Iterator<Item = AdjSquareInfo> + '_
    {
        ALL_DIRECTIONS.iter().filter_map(
            move | dir |
                {
                    if let Some(ei) = GridConnections::get_edge_index(row_cols, cell_index, *dir) {
                        if self.is_connected[ei] {
                            Some(*dir)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }).map( move | dir |
            {

            let adj_index = get_adjacent_index(cell_index, row_cols.0, row_cols.1, dir).unwrap();


            AdjSquareInfo { direction: dir, cell_index: adj_index }

        })
    }

    fn get_edge_index(row_cols: NumRowCol, cell_index: CellIndex, dir: Direction) -> Option<usize>
    {
        let (cell_row_index, cell_col_index) = cell_index.to_row_col(row_cols.1);
        let edges_per_row = 2 * row_cols.1 - 1;

        match dir {
            NORTH => {
                if cell_row_index == 0 {
                    None
                } else {
                    Some((row_cols.1 - 1) + cell_col_index + edges_per_row * (cell_row_index - 1))
                }
            }
            SOUTH => {
                if cell_row_index >= row_cols.0 - 1 {
                    None
                } else {
                    Some((row_cols.1 - 1) + cell_col_index + edges_per_row * (cell_row_index))
                }
            }
            EAST => {
                if cell_col_index >= row_cols.1 - 1 {
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
    use crate::solver::structs::{get_adjacent_index, CellIndex, ALL_DIRECTIONS};

    #[test]
    fn test_get_index() {
        let dims = (3, 3);
        let mut row = 0;
        let mut col = 0;
        let mut index = CellIndex(row * dims.1 + col);
        assert_eq!(GridConnections::get_edge_index(dims, index, NORTH), None);
        assert_eq!(GridConnections::get_edge_index(dims, index, WEST), None);
        assert_eq!(GridConnections::get_edge_index(dims, index, EAST), Some(0));
        assert_eq!(GridConnections::get_edge_index(dims, index, SOUTH), Some(2));

        col = 1;
        index = CellIndex(row * dims.1 + col);
        assert_eq!(GridConnections::get_edge_index(dims, index, NORTH), None);
        assert_eq!(GridConnections::get_edge_index(dims, index, WEST), Some(0));
        assert_eq!(GridConnections::get_edge_index(dims, index, EAST), Some(1));
        assert_eq!(GridConnections::get_edge_index(dims, index, SOUTH), Some(3));

        row = 1;
        index = CellIndex(row * dims.1 + col);
        assert_eq!(GridConnections::get_edge_index(dims, index, NORTH), Some(3));
        assert_eq!(GridConnections::get_edge_index(dims, index, WEST), Some(5));
        assert_eq!(GridConnections::get_edge_index(dims, index, EAST), Some(6));
        assert_eq!(GridConnections::get_edge_index(dims, index, SOUTH), Some(8));

        row = 2;
        col = 2;
        index = CellIndex(row * dims.1 + col);
        assert_eq!(GridConnections::get_edge_index(dims, index, NORTH), Some(9));
        assert_eq!(GridConnections::get_edge_index(dims, index, WEST), Some(11));
        assert_eq!(GridConnections::get_edge_index(dims, index, EAST), None);
        assert_eq!(GridConnections::get_edge_index(dims, index, SOUTH), None);


        //assert_eq!(bv.as_slice(), &[0b0101_0000, 0b1111_0000]);
    }

    #[test]
    fn test_get_index_adj() {
        for num_rows in 1..=11 {
            for num_cols in 1..=11 {
                for index in 0..num_cols * num_rows {
                    let cell_index = CellIndex(index);

                    for dir in ALL_DIRECTIONS.iter() {

                        let adj = get_adjacent_index(cell_index, num_rows, num_cols, *dir);

                        let ei = GridConnections::get_edge_index((num_rows, num_cols), cell_index,  *dir);

                        assert_eq!(adj.is_none(), ei.is_none());

                        if let Some(adj) = adj {
                            let ei2 = GridConnections::get_edge_index((num_rows, num_cols), adj,  dir.opposite()).unwrap();
                        }
                    }
                }

                //test max
                assert_eq!(GridConnections::get_edge_index((num_rows, num_cols), CellIndex(num_rows*num_cols-1), WEST), Some(num_rows * (num_cols-1) + num_cols * (num_rows-1)));
            }
        }
    }
}