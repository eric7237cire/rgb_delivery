use crate::solver::tree_path::edge_list::{EdgeIndex, EdgeList};
use arrayvec::ArrayVec;
use std::collections::vec_deque::VecDeque;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TreeArrayNode {
    pub all_children_count: usize,
    pub parent_index: usize,
    pub child_start_index: usize,
    pub num_children: u8,
    pub depth: u8,
    pub edge_index: EdgeIndex,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TreeArray {
    pub nodes: Vec<TreeArrayNode>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TreeNode {
    pub all_children_count: usize,
    pub nodes: ArrayVec<[Box<TreeNode>; 3]>,
    pub edge_index: EdgeIndex,
}

impl TreeArray {
    pub fn print_up_to_depth(
        &self,
        current_index: usize,
        allowed_depth: usize,
        edge_list: &EdgeList,
    ) {
        if allowed_depth == 0 {
            return;
        }

        let cur_node = &self.nodes[current_index];

        let last_edge = self.nodes[cur_node.parent_index].edge_index;
        let this_edge = cur_node.edge_index;

        println!(
            "{} depth {} count is {}.  Cell: {} ",
            "..".to_string().repeat(cur_node.depth.into()),
            cur_node.depth,
            cur_node.all_children_count,
            edge_list.get_edge_str_2( this_edge, last_edge),
        );

        for i in
            cur_node.child_start_index..cur_node.child_start_index + cur_node.num_children as usize
        {
            self.print_up_to_depth(i, allowed_depth - 1, edge_list);
        }
    }
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

    #[allow(dead_code)]
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

    pub fn convert_to_array(&self) -> TreeArray {
        let root_node = self;

        let mut tree_array: TreeArray = Default::default();

        let mut queue: VecDeque<(&TreeNode, usize)> = VecDeque::new();

        queue.push_back((root_node, 0));

        while let Some((tree_node, parent_index)) = queue.pop_front() {
            let current_tree_index = tree_array.nodes.len();

            let depth = if tree_array.nodes.len() > 0 {
                //return depth
                tree_array.nodes[parent_index].depth + 1
            } else {
                //root node
                0
            };

            if tree_array.nodes.len() > 0 && tree_array.nodes[parent_index].child_start_index == 0 {
                //set start index
                tree_array.nodes[parent_index].child_start_index = current_tree_index;
            }

            tree_array.nodes.push(TreeArrayNode {
                edge_index: tree_node.edge_index,
                num_children: tree_node.nodes.len() as u8,
                all_children_count: tree_node.all_children_count,
                parent_index,
                depth,
                ..Default::default()
            });

            queue.extend(
                tree_node
                    .nodes
                    .iter()
                    .map(|tn| (tn.as_ref(), current_tree_index)),
            );
        }

        tree_array
    }
}
