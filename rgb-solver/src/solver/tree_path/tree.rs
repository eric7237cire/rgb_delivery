use std::collections::HashMap;
use crate::solver::grid_state::GridState;
use crate::solver::structs::GridConnectionsStaticInfo;
use bitvec::BitVec;
use std::u8;
use std::collections::vec_deque::VecDeque;

struct TernaryTree<T>
{
    //store Ternary tree in 3-ary array
    nodes: Vec<Option<T>>
}

impl<T> TernaryTree<T>
{
    fn new(parent_value: T) -> Self {
        let mut nodes: Vec<Option<T>> = Default::default();
        nodes.push(Some(parent_value));

        Self {
            nodes
        }
    }

    fn get_children(&self, index: usize) -> &[Option<T>]
    {
        //0
        //1 2 3
        //4 5 6   7 8 9   10 11 12

        //3k+1 3k+2 3k+3

        &self.nodes[3 * index + 1..3 * index + 4]
    }

    fn extend_to_size(&mut self, new_size: usize)
    {
        while self.nodes.len() < new_size {
            //let new_nodes: Vec<Option<T>> = vec![None; new_size - self.nodes.len()];
            //self.nodes.extend(new_nodes.iter());
            self.nodes.push(None);
        }
    }

    fn insert_child(&mut self, index: usize, val: T) -> bool {
        self.extend_to_size(3 * index + 4);

        for child_index in 3 * index + 1..=3 * index + 3 {
            if self.nodes[child_index].is_none() {
                self.nodes[child_index] = Some(val);
                return true;
            }
        }

        return false;
    }
}


type EdgeIndex = u8;
type CellIndexTuple = (usize, usize);

#[derive(Default)]
struct EdgeList
{
    edges: HashMap<CellIndexTuple, EdgeIndex>,
    edge_list: Vec<CellIndexTuple>,
}

impl EdgeList {
    fn get_edge_index(&self, cell_index_1: usize, cell_index_2: usize) -> EdgeIndex
    {
        if cell_index_1 < cell_index_2 {
            self.edges[&(cell_index_1, cell_index_2)]
        } else {
            self.edges[&(cell_index_2, cell_index_1)]
        }
    }

    fn get_cell_indexes(&self, edge_index: EdgeIndex) -> &CellIndexTuple
    {
        &self.edge_list[edge_index as usize]
    }

    fn insert_edge_if_needed(&mut self, cell_index_1: usize, cell_index_2: usize) -> EdgeIndex
    {
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

    fn find_next_cell_index(current_cells: &CellIndexTuple, prev_cells: &CellIndexTuple) -> usize
    {
        if current_cells.0 == prev_cells.0 || current_cells.0 == prev_cells.1 {
            current_cells.1
        } else if current_cells.1 == prev_cells.0 || current_cells.1 == prev_cells.1 {
            current_cells.0
        } else {
            panic!("No common one found")
        }
    }
}

fn min_distance(
    cur_index: usize, target_index: usize,
    edge_list: &EdgeList,
    used_edges: &BitVec,
    visited: &mut BitVec,
    distance: &mut Vec<u8>,
    static_info: &GridConnectionsStaticInfo,
)
    -> Option<u8>
{

    if cur_index == target_index {
        return Some(0);
    }

    //visited cells
    visited.set_all(false);

    let mut queue: VecDeque<usize> = VecDeque::new();

    queue.push_back(cur_index);
    distance[cur_index] = 0;
    visited.set(cur_index, true);

    while let Some(current_index) = queue.pop_front()
        {
            for ai in static_info
                .adj_info[current_index].iter().filter_map(|ai| ai.as_ref())
                {
                    let edge_index = edge_list.get_edge_index(current_index, ai.cell_index.0);

                    if used_edges[edge_index as usize] {
                        continue;
                    }

                    if visited[ai.cell_index.0] {
                        continue;
                    }

                    visited.set(ai.cell_index.0, true);

                    distance[ai.cell_index.0] = distance[current_index] + 1;

                    queue.push_back(ai.cell_index.0);

                    if ai.cell_index.0 == target_index {
                        return Some(distance[target_index]);
                    }
                }
        }


    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::grid_state::GridState;
    use crate::solver::structs::{build_graph, CellIndex};
    use bitvec::{BitVec, BigEndian};
    use std::iter::once;
    use std::time::{SystemTime, UNIX_EPOCH};


    #[test]
    fn test_gen_paths() {
        let data_str = include_str!("./test_data/treeTest2.json");

        //build a grid state
        let mut grid_state: GridState = serde_json::from_str(data_str).unwrap();

        grid_state.vans = grid_state.initial_van_list();

        let bg = build_graph(&grid_state);
        let (gc, si) = bg;
        //let gc = bg.0;
        let si = {
            let mut si = si;

            for (idx, ai_array) in si.adj_info.iter_mut().enumerate()
            {
                for dir_idx in 0..4 {
                    if gc.is_connected[idx] & 1 << dir_idx == 0 {
                        ai_array[dir_idx] = None;
                    }
                }
            }

            si
        };


        let edge_list: EdgeList = {
            let mut edge_list: EdgeList = Default::default();

            //add self loop edges for van starting locations
            let van_edge_indexes: Vec<EdgeIndex> = grid_state.vans.iter().map(|v| edge_list.insert_edge_if_needed(v.cell_index.0, v.cell_index.0)).collect();

            for (cell_index, _is_connected) in gc.is_connected.iter().enumerate() {
                for ai in si.adj_info[cell_index].iter().filter_map(|ai| ai.as_ref()) {
                    edge_list.insert_edge_if_needed(cell_index, ai.cell_index.0);
                }
            }

            edge_list
        };

        let van_edge_indexes: Vec<EdgeIndex> = grid_state.vans.iter().map(|v| edge_list.get_edge_index(v.cell_index.0, v.cell_index.0)).collect();

        //println!("{:?}.  Edges: {:?}", grid.tiles[0], edge_list.edges);

        //now create a tree
        let mut tree: TernaryTree<EdgeIndex> = TernaryTree::new(van_edge_indexes[0]);

        let mut used_edges: BitVec<BigEndian, u8> = bitvec![0; edge_list.len()];

        used_edges.set(van_edge_indexes[0] as usize, true);

        let mut edge_stack = Vec::new();
        let mut next_adj_index: Vec<usize> = Vec::new();

        //edges in the current path
        edge_stack.push(van_edge_indexes[0]);
        next_adj_index.push(0);

        let mut it_check: usize = 0usize;

        let mut path_count = 0usize;

        let target_index = 24;
        let max_distance = 25;

        //pre-allocate for min_distance
        let mut visited = bitvec![0; grid_state.width * grid_state.height];
        let mut distance = vec![u8::MAX; visited.len()];

        let start = SystemTime::now();

        'while_loop:
            while let Some(cur_edge) = edge_stack.last()
            {
                let cur_edge = *cur_edge;

                it_check += 1;
                if it_check > 4_000_000_000 {
                    break;
                }
                if it_check % 10_000 == 0 {
                    let time_check = SystemTime::now();
                    let since_the_epoch = time_check.duration_since(start).unwrap();
                    let in_ms = since_the_epoch.as_millis();
                    println!("Done with {}  path count {} μs per iteration {:.3}", it_check, path_count,
                    in_ms as f64 * 1000f64 / it_check as f64
                    );
                }
                let last_next_idx = *next_adj_index.last().unwrap();

                assert!(used_edges[cur_edge as usize]);

                //what is current cell
                let cell_index_tuple = edge_list.get_cell_indexes(cur_edge);

                let current_index = if cell_index_tuple.0 == cell_index_tuple.1 {
                    //root node
                    cell_index_tuple.0
                } else {
                    //discount the common edge between the prev edge and this node
                    let prev_edge = edge_stack[edge_stack.len() - 2];
                    let prev_cell_indexes = edge_list.get_cell_indexes(prev_edge);

                    EdgeList::find_next_cell_index(cell_index_tuple, prev_cell_indexes)
                };

                if let Some(remaining_dist) = min_distance(
                    current_index,
                    target_index,
                    &edge_list,
                    &used_edges,
                    &mut visited,
                    &mut distance,
                    &si,
                ) {
                    /*println!("Node cur edge {} index {} tuple {:?} cur row col {:?} \
                remaining distance {} \
                ", cur_edge, current_index, cell_index_tuple,
                             CellIndex(current_index).to_row_col(grid.width),
                             remaining_dist
                    );*/

                    if edge_stack.len() - 1 + remaining_dist as usize > max_distance {
                        edge_stack.pop();
                        next_adj_index.pop();
                        used_edges.set(cur_edge.into(), false);
                        continue;
                    }
                } else {
                    edge_stack.pop();
                    next_adj_index.pop();
                    used_edges.set(cur_edge.into(), false);
                    continue;
                }

                if current_index == target_index {

                    /*let mut row_col_strs = Vec::new();

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
                    path_count += 1;

                    edge_stack.pop();
                    next_adj_index.pop();
                    used_edges.set(cur_edge.into(), false);
                    continue;
                }

                //println!("Node cur edge {} index {} tuple {:?} cur row col {:?}", cur_edge, current_index, cell_index_tuple, CellIndex(current_index).to_row_col(grid.width));

                //are any edges of the current edge not used?
                for ai in si.adj_info[current_index].iter().enumerate().filter(|(idx, ai)| *idx >= last_next_idx && ai.is_some())
                    {
                        let adj_index = ai.0;
                        let ai = ai.1.as_ref().unwrap();
                        let edge_index = edge_list.get_edge_index(current_index, ai.cell_index.0);

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
                edge_stack.pop();
                next_adj_index.pop();
                used_edges.set(cur_edge.into(), false);
            }

        println!("Number of paths found: {}", path_count);
    }
}