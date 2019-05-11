use crate::solver::tree_path::edge_list::{EdgeIndex, EdgeList};
use arrayvec::ArrayVec;

#[derive(Serialize, Deserialize, Default)]
pub struct TreeNode {
    pub all_children_count: usize,
    pub nodes: ArrayVec<[Box<TreeNode>; 3]>,
    pub edge_index: EdgeIndex,
}

impl TreeNode {
    pub fn add_path_containing_cell(
        &self,
        edge_list: &EdgeList,
        cur_path: &mut Vec<EdgeIndex>,
        paths: &mut Vec<Vec<EdgeIndex>>,
        contains_target: bool,
        target_cell: usize,
    ) {
        //let has_target
        cur_path.push(self.edge_index);

        let cells = edge_list.get_cell_indexes(self.edge_index);
        let contains_target = contains_target || cells.0 == target_cell || cells.1 == target_cell;

        if self.nodes.len() == 0 && contains_target {
            paths.push(cur_path.clone());
            cur_path.pop();
            return;
        }

        for n in self.nodes.iter() {
            n.add_path_containing_cell(edge_list, cur_path, paths, contains_target, target_cell);
        }

        cur_path.pop();
        return;
    }

    pub fn print_up_to_depth(
        &self,
        current_depth: usize,
        allowed_depth: usize,
        edge_list: &EdgeList,
    ) {
        if allowed_depth == 0 {
            return;
        }

        let last_edge = self.edge_index;
        let this_edge = if self.nodes.len() > 0 {
            self.nodes[0].edge_index
        } else {
            self.edge_index
        };

        println!(
            "{} depth {} count is {}.  Cell: {}",
            "..".to_string().repeat(current_depth),
            current_depth,
            self.all_children_count,
            edge_list.get_edge_str(last_edge, this_edge)
        );

        for n in self.nodes.iter() {
            n.print_up_to_depth(current_depth + 1, allowed_depth - 1, edge_list);
        }
    }

    pub fn add_path(&mut self, path: &Vec<EdgeIndex>) {
        let mut current_node = self;

        assert_eq!(current_node.edge_index, path[0]);

        for (idx, path_edge_index) in path.iter().enumerate().skip(1) {
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
                println!(
                    "Too many nodes idx: {} edge: {}.  Hmm {}",
                    idx, path_edge_index, current_node.nodes[0].edge_index
                );
            }

            current_node.nodes.push(new_tree_node);

            let last_index = current_node.nodes.len() - 1;
            current_node = current_node.nodes[last_index].as_mut();

            current_node.all_children_count += 1;
        }
    }
}
