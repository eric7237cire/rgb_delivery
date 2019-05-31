use crate::solver::structs::*;

use crate::solver::structs::TileEnum::TileWarehouse;

//use crate::solver::public_func::build_color_list;

//use crate::solver::utils::VAN_LABEL;

use crate::solver::structs::Warehouse;

impl TileEnum {
    pub(crate) fn mut_warehouse(&mut self) -> &mut Warehouse {
        match self {
            TileWarehouse(inner) => {
                return inner;
            }
            _ => panic!(),
        }
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

/*
#[derive(Default)]
pub(crate) struct GraphBridge {
    bridges: usize,      // number of bridges
    cnt: usize,          // counter
    pre: Vec<Option<usize>>,        // pre[v] = order in which dfs examines v
    low: Vec<Option<usize>>         // low[v] = lowest preorder of any vertex connected to v
}
*/

//impl GraphBridge {

/*
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




}*/

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

//}
