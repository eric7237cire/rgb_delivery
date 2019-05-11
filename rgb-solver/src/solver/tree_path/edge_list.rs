use crate::solver::structs::CellIndex;
use std::collections::HashMap;

pub type EdgeIndex = u8;

type CellIndexTuple = (usize, usize);

#[derive(Default)]
pub struct EdgeList {
    edges: HashMap<CellIndexTuple, EdgeIndex>,
    edge_list: Vec<CellIndexTuple>,
    pub grid_width: usize,
}

impl EdgeList {
    pub fn get_edge_index(&self, cell_index_1: usize, cell_index_2: usize) -> EdgeIndex {
        if cell_index_1 < cell_index_2 {
            self.edges[&(cell_index_1, cell_index_2)]
        } else {
            self.edges[&(cell_index_2, cell_index_1)]
        }
    }

    pub fn get_cell_indexes(&self, edge_index: EdgeIndex) -> &CellIndexTuple {
        &self.edge_list[edge_index as usize]
    }

    pub fn get_edge_str(&self, edge_index: EdgeIndex, edge_index_2: EdgeIndex) -> String {
        let common_cell = EdgeList::find_common_cell_index(
            self.get_cell_indexes(edge_index),
            self.get_cell_indexes(edge_index_2),
        );

        let row_cols = CellIndex(common_cell).to_row_col(self.grid_width);
        format!("(r{:02}, c{:02})", row_cols.0, row_cols.1)
    }

    pub fn get_edge_str_2(&self, current_edge_index: EdgeIndex, prev_edge_index: EdgeIndex) -> String {
        let common_cell = EdgeList::find_next_cell_index(
            self.get_cell_indexes(current_edge_index),
            self.get_cell_indexes(prev_edge_index),
        );

        let row_cols = CellIndex(common_cell).to_row_col(self.grid_width);
        format!("(r{:02}, c{:02})", row_cols.0, row_cols.1)
    }

    pub fn insert_edge_if_needed(&mut self, cell_index_1: usize, cell_index_2: usize) -> EdgeIndex {
        let cur_len = self.edges.len() as EdgeIndex;

        let cell_index_tuple = if cell_index_1 < cell_index_2 {
            (cell_index_1, cell_index_2)
        } else {
            (cell_index_2, cell_index_1)
        };

        let inserted = *self.edges.entry(cell_index_tuple).or_insert(cur_len);

        //a record was added
        if inserted == cur_len {
            assert_eq!(self.edge_list.len(), cur_len as usize);

            self.edge_list.push(cell_index_tuple);
        }

        inserted
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn find_next_cell_index(
        current_cells: &CellIndexTuple,
        prev_cells: &CellIndexTuple,
    ) -> usize {
        if current_cells.0 == prev_cells.0 || current_cells.0 == prev_cells.1 {
            current_cells.1
        } else if current_cells.1 == prev_cells.0 || current_cells.1 == prev_cells.1 {
            current_cells.0
        } else {
            panic!("No common one found")
        }
    }

    fn find_common_cell_index(
        current_cells: &CellIndexTuple,
        prev_cells: &CellIndexTuple,
    ) -> usize {
        if current_cells.0 == prev_cells.0 || current_cells.0 == prev_cells.1 {
            current_cells.0
        } else if current_cells.1 == prev_cells.0 || current_cells.1 == prev_cells.1 {
            current_cells.1
        } else {
            panic!("No common one found")
        }
    }
}
