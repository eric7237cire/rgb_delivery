use bitvec::BitVec;
use crate::solver::tree_path::ternary_tree_array::TreeArray;
use crate::solver::tree_path::edge_list::{EdgeList, EdgeIndex};

//len cells == len edges + 1
struct GridPath {
    cells: Vec<usize>,
    edges: Vec<EdgeIndex>,
    cell_mask: BitVec,
    edge_mask: BitVec
}

impl GridPath {
    fn create_grid_paths( tree_array: &TreeArray, edge_list: &EdgeList) -> Vec<GridPath>
    {
        let mut paths = Vec::new();

        for (idx,tree_node) in tree_array.nodes.iter().enumerate()
        {
            //only want leaf nodes
            if tree_node.num_children > 0 {
                continue;
            }

            let mut edges = Vec::new();

            let mut cur_index = idx;

            loop {
                let cur_node = &tree_array.nodes[cur_index];
                edges.push( cur_node.edge_index);

                if cur_node.parent_index == 0 {
                    break;
                }
            }

            edges.reverse();

            //indexes everything but first & last
            let mut cell_indexes : Vec<usize> = edges.windows(2).map( |
                window_slice |
            {
                let from_edge = window_slice[0];
                let to_edge = window_slice[1];

edge_list.get_cell_index( to_edge, from_edge, )
            }).collect();


            cell_indexes.insert(0,
                                edge_list.get_extreme_cell_index(edges[0], cell_indexes[0]));
            cell_indexes.push(
                                edge_list.get_extreme_cell_index(*edges.last().unwrap(), *cell_indexes.last().unwrap()));


            let mut cell_mask = bitvec![0; edge_list.grid_height * edge_list.grid_width];
            let mut edge_mask = bitvec![0; edge_list.edges.len()]


            for edge_index in edges.iter() {
                edge_mask.set(edge_index, true);
            }

            for cell_index in cell_indexes.iter() {
                cell_mask.set(cell_index, true);
            }

            paths.push(GridPath {
                edge_mask,
                cell_mask,
                edges,
                cells: cell_indexes
            });

        }


        paths
    }
}