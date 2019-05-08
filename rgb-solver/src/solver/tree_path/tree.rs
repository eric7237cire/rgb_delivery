use std::collections::HashMap;

struct TernaryTree<T>
{
    //store Ternary tree in 3-ary array
    nodes : Vec<Option<T>>
}

impl<T> TernaryTree<T>
{
    fn new( parent_value: T ) -> Self {
        let mut nodes : Vec<Option<T>> = Default::default();
        nodes.push(Some(parent_value));

        Self {
            nodes
        }
    }

    fn get_children(&self, index:usize) -> &[Option<T>]
    {
        //0
        //1 2 3
        //4 5 6   7 8 9   10 11 12

        //3k+1 3k+2 3k+3

        &self.nodes[3*index+1..3*index+4]
    }

    fn extend_to_size(&mut self, new_size: usize)
    {
        while self.nodes.len() < new_size {
            //let new_nodes: Vec<Option<T>> = vec![None; new_size - self.nodes.len()];
            //self.nodes.extend(new_nodes.iter());
            self.nodes.push(None);
        }
    }

    fn insert_child(&mut self, index:usize, val: T) -> bool {

        self.extend_to_size(3*index+4);

        for child_index in 3*index+1..=3*index+3 {
            if self.nodes[child_index].is_none() {
                self.nodes[child_index] = Some(val);
                return true;
            }
        }

        return false;
    }
}


type EdgeIndex = u8;
type CellIndexTuple = (usize,usize);

#[derive(Default)]
struct EdgeList
{
    edges: HashMap< CellIndexTuple, EdgeIndex>,
    edge_list: Vec< CellIndexTuple >
}

impl EdgeList {

    fn get_edge_index( &self, cell_index_1: usize, cell_index_2: usize ) -> EdgeIndex
    {
        if cell_index_1 < cell_index_2 {
            self.edges[&(cell_index_1, cell_index_2)]
        } else {
            self.edges[&(cell_index_2, cell_index_1)]
        }
    }

    fn get_cell_indexes( &self, edge_index: EdgeIndex ) -> &CellIndexTuple
    {
        &self.edge_list[ edge_index as usize]
    }

    fn insert_edge_if_needed(&mut self, cell_index_1: usize, cell_index_2: usize ) -> EdgeIndex
    {
        let cur_len = self.edges.len() as EdgeIndex;

        let cell_index_tuple = if cell_index_1 < cell_index_2 {
            (cell_index_1, cell_index_2)
        } else {
            (cell_index_2, cell_index_1)
        };

        let inserted = *self.edges.entry(cell_index_tuple).or_insert( cur_len );

        //a record was added
        if inserted == cur_len {
            assert_eq!(edge_list.len(), cur_len);

            edge_list.push( cell_index_tuple);
        }

        inserted

    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    fn find_next_cell_index(tup1: &CellIndexTuple, prev_cells: &CellIndexTuple) -> usize
    {
        if tup1.0 == tup2.0 {
            tup1.1
        } else if tup1.0 == tup2.1 {
            tup1.1
        } else if tup1.1 == tup2.0 {
            tup1.0
        } else if tup1.1 == tup2.1 {
            tup1.0
        } else {
            panic!("No common one found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::grid_state::GridState;
    use crate::solver::structs::build_graph;
    use bitvec::{BitVec, BigEndian};


    #[test]
    fn test_gen_paths() {
        let data_str = include_str!("./test_data/treeTest1.json");

        //build a grid state
        let mut grid: GridState = serde_json::from_str(data_str).unwrap();

        grid.vans = grid.initial_van_list();

        let bg = build_graph(&grid);
        let gc = bg.0;
        let si = bg.1;

        let mut edge_list: EdgeList = Default::default();

        //add self loop edges for van starting locations
        let van_edge_indexes: Vec<EdgeIndex> = grid.vans.iter().map( |v| edge_list.insert_edge_if_needed(v.cell_index.0,v.cell_index.0)).collect();

        for (cell_index,_is_connected) in gc.is_connected.iter().enumerate() {
            for ai in si.adj_info[cell_index].iter().filter_map(|ai| ai.as_ref()) {
                edge_list.insert_edge_if_needed( cell_index, ai.cell_index.0);
            }
        }

        println!("{:?}.  Edges: {:?}", grid.tiles[0], edge_list.edges);

        //now create a tree
        let mut tree : TernaryTree<EdgeIndex> = TernaryTree::new( van_edge_indexes[0] );

        let mut used_edges: BitVec<BigEndian, u8> = bitvec![false; edge_list.len()];

        used_edges.set( van_edge_indexes[0] as usize, true);

        let mut stack = Vec::new();

        //edges in the current path
        stack.push( van_edge_indexes[0] );

        while let Some(cur_edge) = stack.pop()
        {
            //what is current cell
            let cell_index_tuple = edge_list.get_cell_indexes(cur_edge);

            let current_index = if cell_index_tuple.0 == cell_index_tuple.1 {
                //root node
                cell_index_tuple.0
            } else {
                //discount the common edge between the prev edge and this node
                let prev_edge = *stack.last().unwrap();
                let prev_cell_indexes = edge_list.get_cell_indexes(prev_edge);

                EdgeList::find_next_cell_index( cell_index_tuple, prev_cell_indexes )
            };

            //are any edges of the current edge not used?
            for
        }
    }
}