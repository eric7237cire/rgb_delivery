use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::SystemTime;
use std::u8;

use arrayvec::ArrayVec;
use bincode::{deserialize_from, serialize_into};
use bitvec::BitVec;
use serde::{Serialize};
use serde::de::DeserializeOwned;

use crate::solver::disjointset::DisjointSet;
use crate::solver::grid_state::GridState;
use crate::solver::structs::{
    build_graph, CellIndex, ColorIndex, Road,GridConnections, GridConnectionsStaticInfo, Warehouse,
};
use crate::solver::structs::tile::TileEnum::TileRoad;
use crate::solver::structs::tile::TileEnum::TileWarehouse;
use crate::solver::tree_path::edge_list::{EdgeIndex, EdgeList};
use crate::solver::tree_path::ternary_tree_box::TreeArray;
use crate::solver::tree_path::ternary_tree_box::TreeNode;

struct CellIndexConstraint {
    cell_index: usize,
    one_of_cell_index: ArrayVec<[usize; 3]>,
}

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

        si.num_cols = grid_state.width;

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
    pub fn calc_paths(
        &self,
        source_edge: EdgeIndex,
        constraints: &Vec<CellIndexConstraint>,
        target_index: usize,
        max_distance: usize,
    ) -> TreeNode {
        //println!("{:?}.  Edges: {:?}", grid.tiles[0], edge_list.edges);

        //now create a tree
        let mut tree: TreeNode = TreeNode {
            edge_index: source_edge,
            ..Default::default()
        };

        //let mut path_list : Vec<Vec<EdgeIndex>> = Vec::new();

        let mut used_edges: BitVec = bitvec![0; self.edge_list.len()];

        used_edges.set(source_edge as usize, true);

        let mut edge_stack = Vec::new();
        let mut next_adj_index: Vec<usize> = Vec::new();

        //edges in the current path
        edge_stack.push(source_edge);
        next_adj_index.push(0);

        let mut it_check: usize = 0usize;

        let mut path_count = 0usize;

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
            if it_check > 1_000_000_000 {
                //break;
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

            //Check constraints, using union find
            if !check_constraints(constraints, &self.si, &used_edges) {
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

fn check_constraints(
    constraints: &Vec<CellIndexConstraint>,
    si: &GridConnectionsStaticInfo,
    used_edges: &BitVec,
) -> bool {
    let mut ds = DisjointSet::new(si.adj_info.len());

    for (idx, ai_array) in si.adj_info.iter().enumerate() {
        for ai in ai_array.iter().filter_map(|ai| ai.as_ref()) {
            let edge_index = ai.edge_index;
            //edge_list.get_edge_index(current_index, ai.cell_index.0);

            //println!("Current index {} adj index {} edge index {} is used {}", current_index, ai.cell_index.0, edge_index,used_edges[edge_index as usize]);

            if used_edges[edge_index as usize] {
                continue;
            }

            /*
            let rc = CellIndex(idx).to_row_col(si.num_cols);
            let rc2 = CellIndex(ai.cell_index.0).to_row_col(si.num_cols);

            println!(
                "Cell index {} row {} col {} merge with \
                 index {} row {} col {}.  Edge = {}",
                idx, rc.0, rc.1, ai.cell_index.0, rc2.0, rc2.1, edge_index
            );*/

            ds.merge_sets(idx, ai.cell_index.0);

            /*
            println!(
                "comps {} and {}",
                ds.get_repr(0), ds.get_repr(24)
            );*/
        }
    }

    //now we can check for consistency
    for c in constraints.iter() {
        let comp1 = ds.get_repr(c.cell_index);

        if !c
            .one_of_cell_index
            .iter()
            .any(|ci| ds.get_repr(*ci) == comp1)
        {
            //couldn't find any match

            /*
            println!(
                "Cell index {} no matches in {:?}",
                c.cell_index, c.one_of_cell_index
            );*/

            /*
            for idx in 0..si.adj_info.len() {
                let rc = CellIndex(idx).to_row_col(si.num_cols);

                println!("Cell index {} row {} col {} is in comp {}",
                idx, rc.0, rc.1, ds.get_repr(idx)
                );
            }*/

            return false;
        }
    }

    return true;
}

fn build_constraints(grid_state: &GridState, ignore_warehouse: usize) -> Vec<CellIndexConstraint> {
    (1..=5)
        .map(|color_index| {
            let color_index = ColorIndex(color_index);

            //find warehouses
            grid_state
                .tiles
                .iter()
                .enumerate()
                .filter_map(move |(t_idx, t)| {
                    if t_idx + grid_state.width == ignore_warehouse {
                        return None;
                    }

                    if let TileWarehouse(Warehouse {
                        color: warehouse_color_index, ..
                    }) = t
                    {
                        if color_index == *warehouse_color_index {
                            //square directly below
                            return Some((color_index, t_idx + grid_state.width));
                        }
                    }

                    None

                })
        })
        .flatten()
        .map(|(color_index, warehouse_tile_index)| CellIndexConstraint {
            cell_index: warehouse_tile_index,
            one_of_cell_index: grid_state
                .vans
                .iter()
                .filter_map(|van| {
                    if van.color.is_white() || van.color == color_index {
                        Some(van.cell_index.0)
                    } else {
                        None
                    }
                })
                .collect(),
        })
        .collect()
}



fn get_tree_fn(van_index: usize) -> String {
    format!(
        r"E:\git\rgb_delivery\rgb-solver\src\solver\tree_path\test_data\van{}.tree",
        van_index
    )
}
fn get_array_tree_fn(van_index: usize) -> String {
    format!(
        r"E:\git\rgb_delivery\rgb-solver\src\solver\tree_path\test_data\van{}_array.tree",
        van_index
    )
}
fn read_from_file<T>(file_path: &Path) -> T
where
    T: DeserializeOwned,
{
    let file = File::open(file_path.to_str().unwrap()).unwrap();
    let mut reader = BufReader::new(file);

    let start = SystemTime::now();

    println!("Loading...{}", file_path.display());
    let obj: T = deserialize_from(&mut reader).unwrap();

    let time_check = SystemTime::now();
    let since_the_epoch = time_check.duration_since(start).unwrap();
    let in_ms = since_the_epoch.as_millis();

    println!("Loaded in {:.02} secs", in_ms as f64 / 1000f64);

    obj
}
fn write_to_file<T>(file_path: &Path, obj: &T)
where
    T: Serialize,
{
    let file = File::create(file_path.to_str().unwrap()).unwrap();
    let mut writer = BufWriter::new(file);

    serialize_into(&mut writer, obj).unwrap();
}
fn get_intersect_fn(van_index: usize, target_index: usize) -> String {
    format!(
        r"E:\git\rgb_delivery\rgb-solver\src\solver\tree_path\test_data\van_{}_intersect_{}.path",
        van_index, target_index
    )
}

pub fn gen_paths() {
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

    let popper_cell_indexes: Vec<usize> = grid_state
        .tiles
        .iter()
        .enumerate()
        .filter_map(|(t_idx, t)| {
            if let TileRoad(Road {
                has_popper: true, ..
            }) = t
            {
                Some(t_idx)
            } else {
                None
            }
        })
        .collect();

    for (current_van_index, &target_cell) in [24, 26, 30, 28].iter().enumerate() {
   // for (current_van_index, &target_cell) in [24, 30, 26, 28].iter().enumerate() {
        let constraints = build_constraints(&grid_state, target_cell);

        let tree_file_name = get_tree_fn(current_van_index);
        let array_tree_file_name = get_array_tree_fn(current_van_index);

        let tree_file = Path::new(&tree_file_name);
        let array_tree_file = Path::new(&array_tree_file_name);

        let (tree, tree_array) = if tree_file.exists() && array_tree_file.exists() {
            let tree: TreeNode = read_from_file(&tree_file);
            let tree_array: TreeArray = read_from_file(&array_tree_file);

            (tree, tree_array)
        } else {
            let tree = pc.calc_paths(
                van_edge_indexes[current_van_index],
                &constraints,
                target_cell,
                24,
            );

            write_to_file(&tree_file, &tree);

            println!("Done calcing tree");
            //tree.print_up_to_depth(0, 14, &pc.edge_list);

            let tree_array = tree.convert_to_array();

            write_to_file(&array_tree_file, &tree_array);

            println!("Done serializing");

            (tree, tree_array)
        };

        tree_array.print_up_to_depth(0, 12, &pc.edge_list);

        if false {
            for (idx, cell_index) in popper_cell_indexes.iter().enumerate() {
                let mut path_list: Vec<Vec<EdgeIndex>> = Vec::new();
                let mut cur_path: Vec<EdgeIndex> = Vec::new();
                tree.add_path_containing_cell(
                    &pc.edge_list,
                    &mut cur_path,
                    &mut path_list,
                    false,
                    *cell_index,
                );

                println!(
                    "Paths intersecting cell {} = {}",
                    cell_index,
                    path_list.len()
                );

                let file_path_sting = get_intersect_fn(current_van_index, idx);
                let file_path = Path::new(&file_path_sting);

                write_to_file(file_path, &path_list);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_paths() {
        gen_paths()
    }

}
