use crate::solver::grid_state::GridState;
use crate::solver::structs::{build_graph, CellIndex, GridConnections, GridConnectionsStaticInfo};
use crate::solver::tree_path::edge_list::{EdgeIndex, EdgeList};
use crate::solver::tree_path::ternary_tree_box::TreeNode;
use bitvec::{BigEndian, BitVec};

use std::time::SystemTime;
use std::u8;

struct PathCalc {
    gc: GridConnections,
    si: GridConnectionsStaticInfo,
    edge_list: EdgeList,
}

impl PathCalc {
    pub fn new(grid_state: &GridState) -> Self {
        let bg = build_graph(&grid_state);
        let (gc, mut si) = bg;
        //let gc = bg.0;

        for (idx, ai_array) in si.adj_info.iter_mut().enumerate() {
            for dir_idx in 0..4 {
                if gc.is_connected[idx] & 1 << dir_idx == 0
                //|| forbidden_squares[idx] {
                {
                    ai_array[dir_idx] = None;
                    continue;
                }

                /*
                if let Some(ai) = &ai_array[dir_idx] {
                    if forbidden_squares[ai.cell_index.0] {
                        ai_array[dir_idx] = None;
                    }
                }*/
            }
        }

        let edge_list: EdgeList = {
            let mut edge_list: EdgeList = Default::default();

            edge_list.grid_width = grid_state.width;

            //add self loop edges for van starting locations
            let _van_edge_indexes: Vec<EdgeIndex> = grid_state
                .vans
                .iter()
                .map(|v| edge_list.insert_edge_if_needed(v.cell_index.0, v.cell_index.0))
                .collect();

            for (cell_index, _is_connected) in gc.is_connected.iter().enumerate() {
                for ai in si.adj_info[cell_index]
                    .iter_mut()
                    .filter_map(|ai| ai.as_mut())
                {
                    let edge_index = edge_list.insert_edge_if_needed(cell_index, ai.cell_index.0);

                    ai.edge_index = edge_index;
                }
            }

            edge_list
        };

        Self { gc, si, edge_list }
    }
}

fn min_distance(
    cur_index: usize,
    target_index: usize,

    used_edges: &BitVec,
    visited: &mut BitVec,
    distance: &mut Vec<u8>,
    queue: &mut Vec<usize>,
    static_info: &GridConnectionsStaticInfo,
) -> Option<u8> {
    if cur_index == target_index {
        return Some(0);
    }

    //visited cells
    visited.set_all(false);

    let mut queue_back = 0;
    let mut queue_front = 0;

    //[queue_front, queue_back)

    queue[queue_back] = cur_index;
    queue_back += 1;
    distance[cur_index] = 0;
    visited.set(cur_index, true);

    while queue_front < queue_back {
        let current_index = queue[queue_front];
        queue_front += 1;
        for ai in static_info.adj_info[current_index]
            .iter()
            .filter_map(|ai| ai.as_ref())
        {
            let edge_index = ai.edge_index;

            if used_edges[edge_index as usize] {
                continue;
            }

            if visited[ai.cell_index.0] {
                continue;
            }

            visited.set(ai.cell_index.0, true);

            distance[ai.cell_index.0] = distance[current_index] + 1;

            queue[queue_back] = ai.cell_index.0;
            queue_back += 1;

            if ai.cell_index.0 == target_index {
                return Some(distance[target_index]);
            }
        }
    }

    None
}

impl PathCalc {
    pub fn calc_paths(&self, source_edge: EdgeIndex) ->
   // Vec<Vec<EdgeIndex>>
    TreeNode
    {
        //println!("{:?}.  Edges: {:?}", grid.tiles[0], edge_list.edges);

        //now create a tree
        let mut tree: TreeNode = TreeNode {
            edge_index: source_edge,
            ..Default::default()
        };

        //let mut path_list : Vec<Vec<EdgeIndex>> = Vec::new();

        let mut used_edges: BitVec<BigEndian, u8> = bitvec![0; self.edge_list.len()];

        used_edges.set(source_edge as usize, true);

        let mut edge_stack = Vec::new();
        let mut next_adj_index: Vec<usize> = Vec::new();

        //edges in the current path
        edge_stack.push(source_edge);
        next_adj_index.push(0);

        let mut it_check: usize = 0usize;

        let mut path_count = 0usize;

        let target_index = 24;
        let max_distance = 24;

        //pre-allocate for min_distance
        let mut visited = bitvec![0; self.gc.num_cols * self.gc.num_rows];
        let mut distance = vec![u8::MAX; visited.len()];
        let mut queue = vec![0usize; visited.len()];

        let start = SystemTime::now();

        'while_loop: while let Some(cur_edge) = edge_stack.last() {
            let cur_edge = *cur_edge;

            macro_rules! pop_stack {
                () => {
                    edge_stack.pop();
                    next_adj_index.pop();
                    used_edges.set(cur_edge.into(), false);
                    continue;
                };
            }

            it_check += 1;
            if it_check > 100_000_000 {
                break;
            }
            if it_check % 100_000 == 0 {
                let time_check = SystemTime::now();
                let since_the_epoch = time_check.duration_since(start).unwrap();
                let in_ms = since_the_epoch.as_millis();

                //tree.print_up_to_depth(0, 10, &edge_list);

                println!(
                    "Done with {}  path count {} μs per iteration {:.3}",
                    it_check,
                    path_count,
                    in_ms as f64 * 1000f64 / it_check as f64
                );
            }
            let last_next_idx = *next_adj_index.last().unwrap();

            assert!(used_edges[cur_edge as usize]);

            //what is current cell
            let cell_index_tuple = self.edge_list.get_cell_indexes(cur_edge);

            let current_index = if cell_index_tuple.0 == cell_index_tuple.1 {
                //root node
                cell_index_tuple.0
            } else {
                //discount the common edge between the prev edge and this node
                let prev_edge = edge_stack[edge_stack.len() - 2];
                let prev_cell_indexes = self.edge_list.get_cell_indexes(prev_edge);

                EdgeList::find_next_cell_index(cell_index_tuple, prev_cell_indexes)
            };

            if let Some(remaining_dist) = min_distance(
                current_index,
                target_index,
                &used_edges,
                &mut visited,
                &mut distance,
                &mut queue,
                &self.si,
            ) {
                /*println!("Node cur edge {} index {} tuple {:?} cur row col {:?} \
                remaining distance {} \
                ", cur_edge, current_index, cell_index_tuple,
                             CellIndex(current_index).to_row_col(grid.width),
                             remaining_dist
                    );*/

                if edge_stack.len() - 1 + remaining_dist as usize > max_distance {
                    pop_stack!();
                }
            } else {
                pop_stack!();
            }

            //got to target
            if current_index == target_index {
                if false {
                    let mut row_col_strs = Vec::new();

                    let start = self.edge_list.get_cell_indexes(edge_stack[0]).0;
                    let row_cols = CellIndex(start).to_row_col(self.gc.num_cols);

                    row_col_strs.push(format!("(r{:02}, c{:02})", row_cols.0, row_cols.1));

                    for ft in edge_stack.windows(2) {
                        let last_edge = ft[0];
                        let this_edge = ft[1];
                        let cell_index = EdgeList::find_next_cell_index(
                            self.edge_list.get_cell_indexes(this_edge),
                            self.edge_list.get_cell_indexes(last_edge),
                        );
                        let row_cols = CellIndex(cell_index).to_row_col(self.gc.num_cols);
                        row_col_strs.push(format!("(r{:02}, c{:02})", row_cols.0, row_cols.1));
                    }

                    println!("{}", row_col_strs.join(", "));
                }
                path_count += 1;
                tree.add_path(&edge_stack);
                //path_list.push(edge_stack.clone());

                pop_stack!();
            }

            //println!("Node cur edge {} index {} tuple {:?} cur row col {:?}", cur_edge, current_index, cell_index_tuple, CellIndex(current_index).to_row_col(grid.width));

            //are any edges of the current edge not used?
            for ai in self.si.adj_info[current_index]
                .iter()
                .enumerate()
                .filter(|(idx, ai)| *idx >= last_next_idx && ai.is_some())
            {
                let adj_index = ai.0;
                let ai = ai.1.as_ref().unwrap();
                let edge_index = ai.edge_index;
                //edge_list.get_edge_index(current_index, ai.cell_index.0);

                //println!("Current index {} adj index {} edge index {} is used {}", current_index, ai.cell_index.0, edge_index,used_edges[edge_index as usize]);

                if used_edges[edge_index as usize] {
                    continue;
                }

                //edge_stack.push(*cur_edge);
                edge_stack.push(edge_index);
                let last_idx = next_adj_index.len() - 1;
                next_adj_index[last_idx] = adj_index + 1;
                next_adj_index.push(0);

                assert_eq!(edge_stack.len(), next_adj_index.len());

                used_edges.set(edge_index.into(), true);

                continue 'while_loop;
            }

            //println!("No more choices");

            if last_next_idx == 0 {
                /*
                let mut row_col_strs = Vec::new();

                let start = edge_list.get_cell_indexes(edge_stack[0]).0;
                let row_cols = CellIndex(start).to_row_col(grid.width);

                row_col_strs.push(format!("(r{:02}, c{:02})", row_cols.0, row_cols.1  ));

                for ft in edge_stack.windows(2) {
                    let last_edge = ft[0];
                    let this_edge = ft[1];
                    let cell_index = EdgeList::find_next_cell_index( edge_list.get_cell_indexes(this_edge), edge_list.get_cell_indexes(last_edge) );
                    let row_cols = CellIndex(cell_index).to_row_col(grid.width);
                    row_col_strs.push(format!("(r{:02}, c{:02})", row_cols.0, row_cols.1  ));
                }

                println!("{}", row_col_strs.join(", "));
                */

                //path_count += 1;
            }

            //here we didn't find anything, done
            pop_stack!();
        }

        println!("Number of paths found: {}", path_count);

        tree

        //path_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::grid_state::GridState;
    use bincode::{deserialize_from, serialize_into};
    use std::fs::File;
    use crate::solver::structs::Road;
    use crate::solver::structs::tile::TileEnum::TileRoad;

    const VAN_0_TREE: &str =
        r"E:\git\rgb_delivery\rgb-solver\src\solver\tree_path\test_data\van0.tree";
    const VAN_0_INTERSECT_BASE: &str =
        r"E:\git\rgb_delivery\rgb-solver\src\solver\tree_path\test_data\van_0_intersect";

    #[test]
    fn test_gen_paths() {
        let data_str = include_str!("./test_data/treeTest2.json");

        //build a grid state
        let mut grid_state: GridState = serde_json::from_str(data_str).unwrap();

        grid_state.vans = grid_state.initial_van_list();

        let pc = PathCalc::new(&grid_state);

        let van_edge_indexes: Vec<EdgeIndex> = grid_state
            .vans
            .iter()
            .map(|v| pc.edge_list.get_edge_index(v.cell_index.0, v.cell_index.0))
            .collect();

        //26 28 30

        let mut forbidden_squares = bitvec![0; grid_state.height*grid_state.width];

        forbidden_squares.set(26, true);
        forbidden_squares.set(28, true);
        forbidden_squares.set(30, true);
        forbidden_squares.set(66, true);
        forbidden_squares.set(76, true);
        forbidden_squares.set(54, true);

        let popper_cell_indexes: Vec<usize> = grid_state
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(t_idx,t)|
                            if let TileRoad( Road{ has_popper:true,..}) = t
        { Some(t_idx) } else {None})
            .collect();

        let saving = true;

        let tree = if !saving {
            let mut tree_file = File::open(VAN_0_TREE).unwrap();
            //let tree: TreeNode = serde_cbor::from_reader(&mut tree_file).unwrap();

            println!("Loading...");
            let tree: TreeNode = deserialize_from(&mut tree_file).unwrap();
            //let tree: Vec<Vec<EdgeIndex>> = deserialize_from(&mut tree_file).unwrap();

            println!("Loaded");
            tree
        } else {
            let tree = pc.calc_paths(van_edge_indexes[0]);
            let mut tree_file = File::create(VAN_0_TREE).unwrap();

            //serde_cbor::to_writer(&mut tree_file, &tree).unwrap();
            serialize_into(&mut tree_file, &tree).unwrap();
            tree
        };

        tree.print_up_to_depth(0, 9, &pc.edge_list);

        for (idx,cell_index) in popper_cell_indexes.iter().enumerate() {

            let mut path_list: Vec<Vec<EdgeIndex>> = Vec::new();
            let mut cur_path: Vec<EdgeIndex> = Vec::new();
            tree.add_path_containing_cell(&pc.edge_list, &mut cur_path,
&mut path_list,false, *cell_index);

            println!("Paths intersecting cell {} = {}", cell_index, path_list.len());

            //VAN_0_INTERSECT_BASE
            let mut path_file = File::create(format!("{}{}.tree",
            VAN_0_INTERSECT_BASE, idx)
            ).unwrap();

            //serde_json::to_writer(&mut path_file, &path_list).unwrap();
            //serde_cbor::to_writer(&mut path_file, &path_list).unwrap();
            serialize_into(&mut path_file, &path_list).unwrap();

        }
    }
}
