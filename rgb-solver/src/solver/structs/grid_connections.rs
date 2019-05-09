use crate::solver::grid_state::GridState;
use crate::solver::structs::direction::get_adjacent_index;
use crate::solver::structs::tile::TileEnum::TileWarehouse;
use crate::solver::structs::Direction::*;
use crate::solver::structs::{CellIndex, Direction, ALL_DIRECTIONS};

#[derive(Clone, Default)]
pub struct GridConnections {
    pub is_connected: Vec<u8>,
    pub num_rows: usize,
    pub num_cols: usize,
}

#[derive(Default)]
pub struct GridConnectionsStaticInfo {
    pub adj_info: Vec<[Option<AdjSquareInfo>; 4]>,
}

#[derive(Debug, Clone, Default)]
pub struct AdjSquareInfo {
    pub direction: Direction,
    pub cell_index: CellIndex,
    pub edge_index: u8,
}

impl GridConnections {
    pub fn build_static_info(&self) -> GridConnectionsStaticInfo {
        GridConnectionsStaticInfo {
            adj_info: (0..self.num_rows * self.num_cols)
                .map(|idx| {
                    let mut v = ALL_DIRECTIONS.iter().map(|d| {
                        if let Some(adj_idx) =
                            get_adjacent_index(CellIndex(idx), self.num_rows, self.num_cols, *d)
                        {
                            Some(AdjSquareInfo {
                                cell_index: adj_idx,
                                direction: *d,
                                ..Default::default()
                            })
                        } else {
                            None
                        }
                    });
                    let array: [Option<AdjSquareInfo>; 4] = [
                        v.next().unwrap(),
                        v.next().unwrap(),
                        v.next().unwrap(),
                        v.next().unwrap(),
                    ];
                    array
                })
                .collect(),
        }
    }

    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        GridConnections {
            is_connected: vec![0; num_rows * num_cols],
            num_rows,
            num_cols,
        }
    }

    //#[inline(always)]
    pub fn get_adjacent_square_indexes<'a>(
        &'a self,
        static_info: &'a GridConnectionsStaticInfo,
        cell_index: CellIndex,
    ) -> impl Iterator<Item = &'a AdjSquareInfo> {
        ALL_DIRECTIONS.iter().filter_map(move |dir| {
            if self.is_connected(cell_index, *dir) {
                static_info.adj_info[cell_index.0][*dir as usize].as_ref()
            } else {
                None
            }
        })
    }

    //#[inline(always)]
    pub fn is_connected(&self, cell_index: CellIndex, dir: Direction) -> bool {
        self.is_connected[cell_index.0] & (1 << dir as u8) > 0
    }

    //#[inline(always)]
    pub fn set_is_connected(&mut self, cell_index: CellIndex, dir: Direction, connected: bool) {
        if connected {
            self.is_connected[cell_index.0] |= 1 << dir as u8;
        } else {
            self.is_connected[cell_index.0] &= !(1 << dir as u8);
        }
    }
}

pub fn build_graph(data: &GridState) -> (GridConnections, GridConnectionsStaticInfo) {
    let mut gc = GridConnections::new(data.height, data.width);

    let so = gc.build_static_info();

    for (cur_square_index, tile) in data.tiles.iter().enumerate() {
        if let Some(connection_mask) = tile.get_connection_mask() {
            let cell_index = CellIndex(cur_square_index);

            for adj_dir in ALL_DIRECTIONS.iter() {
                let adj_square_index = get_adjacent_index(
                    CellIndex(cur_square_index),
                    data.height,
                    data.width,
                    *adj_dir,
                );

                if let Some(adj_square_index) = adj_square_index {
                    if let Some(adj_connection_mask) =
                        data.tiles[adj_square_index.0].get_connection_mask()
                    {
                        if (connection_mask & (1 << *adj_dir as u8)) > 0
                            && (adj_connection_mask & (1 << adj_dir.opposite() as u8) > 0)
                        {
                            gc.set_is_connected(cell_index, *adj_dir, true);
                        }
                    } else if adj_dir == &NORTH {
                        if let TileWarehouse(_) = &data.tiles[adj_square_index.0] {
                            //special case that we want warehouses to be connected to the cell to their south
                            gc.set_is_connected(cell_index, *adj_dir, true);
                        }
                    }
                }
            }
        } else {
            continue;
        }
    }

    (gc, so)
}

#[cfg(test)]
mod tests {}
