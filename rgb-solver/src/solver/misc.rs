use crate::solver::struct_defs::*;

use crate::solver::struct_defs::TileEnum::{TileWarehouse, TileRoad, TileBridge};

//use crate::solver::public_func::build_color_list;

//use crate::solver::utils::VAN_LABEL;



impl Directions {
    pub(crate) fn opposite(&self) -> Directions {
        match self {
            NORTH => SOUTH,
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST
        }
    }
}

pub(crate) fn opposite_dir_index(dir_index: usize) -> usize {
        match dir_index {
            0 => 2,
            1 => 3,
            2 => 0,
            3 => 1,
            _ => panic!("Not a valid dir index")
        }
    }


use crate::solver::struct_defs::Directions::*;

use crate::solver::struct_defs::Warehouse;
use crate::solver::grid_state::GridGraph;
use core::cmp;


pub (crate) const ALL_DIRECTIONS: [Directions; 4] = [NORTH, EAST, SOUTH, WEST];


impl TileEnum {
    pub(crate) fn mut_warehouse(&mut self) -> &mut Warehouse {
        match self {
            TileWarehouse(inner) => {
                return inner;
            }
            _ => panic!()
        }
    }
}


pub (crate) fn get_adjacent_index(square_index: CellIndex, grid_height: usize, grid_width: usize,  dir: Directions) -> Option<CellIndex> {

    let (cell_row_index, cell_col_index) = square_index.to_row_col(grid_width);

    match dir {
        NORTH => {
            if cell_row_index == 0 {
                None
            } else {
                Some(CellIndex(square_index.0 - grid_width))
            }
        }
        SOUTH => {
            if cell_row_index >= grid_height - 1 {
                None
            } else {
                Some(CellIndex(square_index.0 + grid_width))
            }
        }
        EAST => {
            if cell_col_index >= grid_width - 1 {
                None
            } else {
                Some(CellIndex(square_index.0 + 1))
            }
        }
        WEST => {
            if cell_col_index == 0 {
                None
            } else {
                Some(CellIndex(square_index.0 - 1))
            }
        }
    }
}

pub (crate) fn is_tile_navigable(tile: &TileEnum) -> bool {
    match tile {
        TileRoad(_) => true,
        TileBridge(_) => true,
        _ => false
    }
}

//Copyright © 2000–2017, Robert Sedgewick and Kevin Wayne.  (Modified to be in Rust...)
/******************************************************************************
 *  Compilation:  javac Bridge.java
 *  Execution:    java  Bridge V E
 *  Dependencies: Graph.java GraphGenerator.java
 *
 *  Identifies bridge edges and prints them out. This decomposes
 *  a directed graph into two-edge connected components.
 *  Runs in O(E + V) time.
 *
 *  Key quantity:  low[v] = minimum DFS preorder number of v
 *  and the set of vertices w for which there is a back edge (x, w)
 *  with x a descendant of v and w an ancestor of v.
 *
 *  Note: code assumes no parallel edges, e.g., two parallel edges
 *  would be (incorrectly) identified as bridges.
 *
 ******************************************************************************/

//https://stackoverflow.com/questions/11218746/bridges-in-a-connected-graph/11221469#11221469

#[derive(Default)]
pub(crate) struct GraphBridge {
    bridges: usize,      // number of bridges
    cnt: usize,          // counter
    pre: Vec<Option<usize>>,        // pre[v] = order in which dfs examines v
    low: Vec<Option<usize>>         // low[v] = lowest preorder of any vertex connected to v
}

impl GraphBridge {

    pub(crate) fn do_it(&mut self, graph: &GridGraph, grid_height: usize, grid_width: usize) {

        let n_vertices = graph.is_connected.len();
        self.low = vec![ None;n_vertices];
        self.pre = vec![ None;n_vertices];
        /*for (int v = 0; v < G.V(); v++)
            low[v] = -1;
        for (int v = 0; v < G.V(); v++)
            pre[v] = -1;*/

        for v in 0..n_vertices {
            if self.pre[v].is_none() {
                self.dfs(graph, v, v, grid_height, grid_width);
            }
        }
    }

    fn dfs(&mut self, graph: &GridGraph, u: usize, v: usize,   grid_height: usize, grid_width: usize) {
        self.pre[v] = Some(self.cnt);
        self.cnt += 1;
        self.low[v] = self.pre[v];


        let is_connected_mask = graph.is_connected[v];
        for (dir_idx, dir) in ALL_DIRECTIONS.iter().enumerate() {
            //not connected
            if is_connected_mask & (1 << dir_idx) == 0 {
                continue;
            }

            let w = get_adjacent_index(
                    CellIndex(v),
                    grid_height, grid_width, *dir).expect("Should not be connected if there is no adj cell");

            let w = w.0;

            if self.pre[w].is_none() {
                self.dfs(graph, v, w, grid_height, grid_width);
                self.low[v] = Some( cmp::min(self.low[v].unwrap(), self.low[w].unwrap() ) );
                if self.low[w] == self.pre[w] {
                    log_trace!("{} - {} is a bridge", v, w);
                    self.bridges+=1;
                }
            }

            // update low number - ignore reverse of edge leading to v
            else if w != u {
                self.low[v] = cmp::min(self.low[v], self.pre[w]);
            }
        }




    }

    // test client
    /*
    public static void main(String[] args) {
        int V = Integer.parseInt(args[0]);
        int E = Integer.parseInt(args[1]);
        Graph G = GraphGenerator.simple(V, E);
        StdOut.println(G);

        Bridge bridge = new Bridge(G);
        StdOut.println("Edge connected components = " + bridge.components());
    }*/


}

