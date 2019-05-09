//too much memory

struct TernaryTree<T> {
    //store Ternary tree in 3-ary array
    nodes: Vec<Option<T>>,
}

impl<T> TernaryTree<T>
where T: PartialEq +
std::fmt::Debug + Copy,
{
    fn new(parent_value: T) -> Self {
        let mut nodes: Vec<Option<T>> = Default::default();
        nodes.push(Some(parent_value));

        Self { nodes }
    }

    pub fn add_path(&mut self, path: &Vec<T>)
    {
        let mut current_node = 0;

        assert_eq!( self.nodes[0].as_ref(), Some(&path[0]) ) ;

        for path_edge_index in path.iter().skip(1) {
            if self.get_children(current_node).iter().any(
                |c| Some(path_edge_index) == c.as_ref()
            ) {
                continue;
            }

            current_node = self.insert_child(current_node, *path_edge_index).unwrap();
        }
    }

    fn get_children(&self, index: usize) -> &[Option<T>] {

        if self.nodes.len() < 3 * index + 4 {
            return &[];
        }

        //0
        //1 2 3
        //4 5 6   7 8 9   10 11 12

        //3k+1 3k+2 3k+3

        &self.nodes[3 * index + 1..3 * index + 4]
    }

    fn extend_to_size(&mut self, new_size: usize) {
        while self.nodes.len() < new_size {
            //let new_nodes: Vec<Option<T>> = vec![None; new_size - self.nodes.len()];
            //self.nodes.extend(new_nodes.iter());
            self.nodes.push(None);
        }
    }

    fn insert_child(&mut self, index: usize, val: T) -> Option<usize> {
        self.extend_to_size(3 * index + 4);

        for child_index in 3 * index + 1..=3 * index + 3 {
            if self.nodes[child_index].is_none() {
                self.nodes[child_index] = Some(val);
                return Some(child_index);
            }
        }

        None
    }
}
