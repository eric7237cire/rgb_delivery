
use crate::solver::grid_state::GridState;
use crate::solver::universe::Universe;
use crate::solver::structs::{Direction, ALL_DIRECTIONS, VanIndex};
use crate::solver::structs::Direction::*;

pub struct StackNode
{
    //0 to 3 direction, 4 for stopped
    current_van_direction: Direction,
    used_popper: bool,
    pub current_state: GridState,
}

impl Universe {
    fn handle_current_state_invalid(&mut self) {
        self.last_node = self.stack.pop();
        self.is_failure = self.last_node.is_none();


    }
    pub fn do_backtracking(&mut self)
    {
        if self.is_failure {
            return;
        }

        let cur_state = if let Some(node) = &self.last_node {
            &node.current_state
        } else {
            &self.initial_data
        };

        //invariant, anything on the stack actually happened
        if self.success_state.is_some() {
            return;
        }
        self.iter_count += 1;


        //self.current_calc_state = Some(cur_state.clone());

        //check success, where all warehouses are filled
        if cur_state.check_success() {
            log!("Success!");
            self.success_state = Some(cur_state.clone());
            return;
        }

        let current_tick = self.stack.len() / cur_state.vans.len();
        let current_van_index = self.stack.len() % cur_state.vans.len();

        log_trace!("\n\nLoop count: {} Tick: {} \
        Stack Size: {} Cur van index: {:?}  Row/Col: {:?}",
            self.iter_count,
            current_tick,
            self.stack.len(),
            cur_state.current_van_index,
            cur_state.vans[cur_state.current_van_index.0].cell_index.to_row_col(cur_state.width)
        );


        cur_state.check_bridges_and_buttons();



        //If we are stopped, we stay stopped
        if self.stack.len() > cur_state.vans.len() && self.stack[self.stack.len() - 1 - cur_state.vans.len()].current_van_direction == STOP {
            self.stack.push(
                StackNode {
                    current_van_direction: STOP,
                    current_state: cur_state.clone(),
                    used_popper: false

                }
            );
            return;
        }

        if self.iter_count % 10000 == 0 {
            log!("\n\nLoop count: {} \
             Queue Length: {} Current Tick: {} ",
                 self.iter_count, self.stack.len(), current_tick);
        }

        if !cur_state.check_graph_validity(current_tick) {
            log_trace!("Rejecting state");
            self.handle_current_state_invalid();
            return;
        }


        let van_cell_index = cur_state.vans[current_van_index].cell_index;

        //let (cur_row_index, cur_col_index) = van_cell_index.to_row_col(self.data.width);








        //now attempt to move

        log_trace!("Adj squares: {:?}", adj_square_indexes);
        let mut any_moved = false;

        let fixed_choice_opt = self.get_fixed_choice(&cur_state, current_tick,current_van_index);


        //Where could we move?  (looks at mask & grid)

        let mut skip_count = 0;
        let used_popper ;

        if let Some(StackNode { current_van_direction, used_popper: up,.. }) = &self.last_node {
            skip_count = *current_van_direction as usize + 1;
            used_popper = *up;
        } else {
            used_popper = false;
        }

        let mut next_used_popper = used_popper;

        let cell_index = cur_state.vans[current_van_index].cell_index;

        let mut next_dir = ALL_DIRECTIONS.iter().skip(skip_count).
            filter_map(|&dir| {
                if !cur_state.graph.is_connected(cell_index, dir) {
                    return None;
                }
                let adj_info = self.gc_static_info.adj_info[cell_index.0][dir as usize].as_ref();
                cur_state.filter_map_by_can_have_van(current_van_index,&fixed_choice_opt, adj_info.unwrap())
            }).next();

        if next_dir.is_none() {

            //see if van can stop
            if current_tick == 0 || cur_state.is_below_warehouse(current_van_index) {

                let mut node = StackNode {
                        current_van_direction: STOP,
                        current_state: cur_state.clone(),
                        used_popper
                    };
                if node.current_state.handle_post_move_actions(&self.analysis, &self.gc_static_info,
                current_van_index) {
                    self.stack.push(node);
                    return;
                }

                //continue down
            }

            log_trace!("see if we can pop");
            if !used_popper {
                next_used_popper=true;

            }

            if next_dir.is_none() {
                //couldn't use popper above
                self.handle_current_state_invalid();
                return;
            }
        }

        let adj_info = next_dir.unwrap();

        let mut next_state = cur_state.clone();

        //popper then move
        if !used_popper && next_used_popper {
        if let Some(mut next_state) = cur_state.handle_block_popper(VanIndex(current_van_index), current_tick) {
                    next_dir = ALL_DIRECTIONS.iter().
                        filter_map(|&dir| {
                            if !cur_state.graph.is_connected(cell_index, dir) {
                                return None;
                            }
                            let adj_info = self.gc_static_info.adj_info[cell_index.0][dir as usize].as_ref();
                            cur_state.filter_map_by_can_have_van(current_van_index, &fixed_choice_opt, adj_info.unwrap())
                        }).next();


                }


        next_state.handle_move(VanIndex(current_van_index),current_tick,van_cell_index, adj_info);

        let node = StackNode {
                    current_van_direction: adj_info.direction,
                    current_state: next_state,
                    used_popper
                };
        if next_state.handle_post_move_actions(&self.analysis, &self.gc_static_info, current_van_index) {
            self.stack.push(node);
        } else {
            //failed state
            self.last_node = Some(node);
        }
        return;
    }
}