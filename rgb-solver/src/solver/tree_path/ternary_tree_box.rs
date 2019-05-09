use crate::solver::tree_path::edge_list::{EdgeIndex, EdgeList};
use arrayvec::ArrayVec;

#[derive(Default)]
pub struct TreeNode {
    pub all_children_count: usize,
    pub nodes: ArrayVec<[Box<TreeNode>; 3]>,
    pub edge_index: EdgeIndex,
}

impl TreeNode {

    pub fn print_up_to_depth(&self, current_depth: usize,
                             allowed_depth: usize,
                             edge_list: &EdgeList)
    {
        if allowed_depth == 0 {
            return;
        }

        let last_edge = self.edge_index;
        let this_edge = if self.nodes.len() > 0 {
            self.nodes[0].edge_index
        } else {
            self.edge_index
        };

        println!("{} depth {} count is {}.  Cell: {}",
            "..".to_string().repeat(current_depth),
current_depth, self.all_children_count,
        edge_list.get_edge_str(last_edge, this_edge)
        );

        for n in self.nodes.iter() {
            n.print_up_to_depth(current_depth+1, allowed_depth -1, edge_list);
        }
    }

    pub fn add_path(&mut self, path: &Vec<EdgeIndex>) {
        let mut current_node = self;

        assert_eq!(current_node.edge_index, path[0]);

        for (idx,path_edge_index) in path.iter().enumerate().skip(1) {

            if let Some(pos) = current_node
                .nodes
                .iter()
                .position(|c| c.edge_index == *path_edge_index)
            {
                //found it
                current_node = current_node.nodes[pos].as_mut();
                current_node.all_children_count += 1;
                continue;
            }

            /*println!("Creating new tree node path idx: {} edge: {}",
                     idx, path_edge_index);*/

            let new_tree_node = Box::new(TreeNode {
                edge_index: *path_edge_index,
                ..Default::default()
            });

            if current_node.nodes.len() >= 3 {
                println!("Too many nodes idx: {} edge: {}.  Hmm {}", idx, path_edge_index, current_node.nodes[0].edge_index);
            }

            current_node.nodes.push(new_tree_node);

            let last_index = current_node.nodes.len() - 1;
            current_node = current_node.nodes[last_index].as_mut();

            current_node.all_children_count += 1;
        }
    }
}
