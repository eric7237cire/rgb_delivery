use bitvec::BitVec;
use crate::solver::tree_path::ternary_tree_array::TreeArray;
use crate::solver::tree_path::edge_list::EdgeList;

//len cells == len edges + 1
struct GridPath {
    cells: Vec<usize>,
    edges: Vec<usize>,
    cell_mask: BitVec,
    edge_mask: BitVec
}

impl GridPath {
    fn create_grid_paths( tree_array: &TreeArray, edge_list: &EdgeList) -> Vec<GridPath>
    {
        let mut paths = Vec::new();

        for (idx,tree_node) in tree_array.iter().enumerate()
        {
            //only want leaf nodes
            if tree_node.num_children > 0 {
                continue;
            }

            let mut edges = Vec::new();

            let mut cur_index = idx;

            loop {
                let cur_node = &tree_array[cur_index];
                edges.push( cur_node.edge_index);

                if cur_node.parent_index == 0 {
                    break;
                }
            }

            edges.reverse();

            //indexes everything but first & last
            let mut cell_indexes : Vec<usize> = edges.windows(2).map( |
                (from_edge, to_edge) |
            {
edge_list.get_cell_index( to_edge, from_edge, )
            }).collect();

            cell_indexes.insert(0, edge_list.)
        }

        paths
    }
}