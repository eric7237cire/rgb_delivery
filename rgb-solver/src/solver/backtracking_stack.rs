use bitvec::BitVec;
use crate::solver::grid_state::GridState;
use crate::solver::universe::Universe;
use crate::solver::structs::{Direction, ALL_DIRECTIONS};
use crate::solver::structs::Direction::*;

pub struct StackNode
{
    //0 to 3 direction, 4 for stopped
    current_van_direction: Direction,
    used_popper: bool,
    current_state: GridState,
}

impl Universe {
    fn handle_current_state_invalid(&mut self) {
        self.last_node = self.stack.pop();
        self.is_failure = self.last_node.is_none();
    }
    fn do_backtracking(&mut self)
    {
        if self.is_failure {
            return;
        }

        let mut cur_state = &self.cur_stack_data;

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

        if current_van_index == 0 {
            match cur_state.toggle_bridges_and_buttons() {
                Err(_) => {
                    log_trace!("Van caught on open bridge");
                    self.handle_current_state_invalid();
                    return;
                }
                Ok(_) => {}
            };
            cur_state.check_bridges_and_buttons();
        } else {
            log_trace!("Tick did not advance");
        }

        //If we are stopped, we stay stopped
        if self.stack.len() > cur_state.vans.len() && self.stack[self.stack.len() - 1 - cur_state.vans.len()] == STOP {
            self.stack.push(
                StackNode {
                    current_van_direction: STOP,
                    current_state: cur_state.clone(),
                }
            );
            continue;
        }

        if self.iter_count % 10000 == 0 {
            log!("\n\nLoop count: {} \
             Queue Length: {} Current Tick: {} ",
                 self.iter_count, self.stack.len(), current_tick);
        }

        if !cur_state.check_graph_validity() {
            log_trace!("Rejecting state");
            self.handle_current_state_invalid();
            return;
        }


        let van_cell_index = cur_state.vans[current_van_index].cell_index;

        //let (cur_row_index, cur_col_index) = van_cell_index.to_row_col(self.data.width);

        match cur_state.pick_up_block_if_exists(&self.analysis) {
            Err(_) => {
                self.handle_current_state_invalid();
                return;
            }
            _ => ()
        };

        //check if we can drop a block off
        match cur_state.handle_warehouse_drop_off(&self.gc_static_info) {
            Ok(Some(next_state)) => {
                self.queue.push_front(next_state);
            }
            Err(_) => {
                self.handle_current_state_invalid();
                return;
            }
            _ => ()
        };

        match cur_state.handle_block_popper() {
            Ok(Some(mut next_state)) => {
                //reset to values as when it was just popped
                next_state.tick = save_for_toggle.0;
                next_state.current_van_index = save_for_toggle.1;
                self.queue.push_front(next_state);
            }
            Err(_) => continue,
            _ => ()
        };


        //now attempt to move

        log_trace!("Adj squares: {:?}", adj_square_indexes);
        let mut any_moved = false;

        let fixed_choice_opt = self.get_fixed_choice(&cur_state);


        //Where could we move?  (looks at mask & grid)

        let mut skip_count = 0;

        if let Some(StackNode { current_van_direction, .. }) = &self.last_node {
            skip_count = *current_van_direction as usize + 1;
        }

        let next_dir = ALL_DIRECTIONS.iter().skip(skip_count).
            filter_map(|&dir| {
                if !cur_state.graph.is_connected(cell_index, *dir) {
                    return None;
                }
                let adj_info = static_info.adj_info[cell_index.0][*dir as usize].as_ref();
                cur_state.filter_map_by_can_have_van(&fixed_choice_opt, adj_info)
            }).next();

        if next_dir.is_none() {

            //see if van can stop
            if current_tick == 0 {
                self.stack.push(
                    StackNode {
                        current_van_direction: STOP,
                        current_state: cur_state.clone(),
                    }
                );
                return;
            }

            self.handle_current_state_invalid();
            return;
        }

        let adj_info = next_dir.unwrap();

        let mut next_state = cur_state.clone();
        next_state.handle_move(van_cell_index, adj_info);


        next_state.press_button_if_exists();

        self.stack.push(
            StackNode {
                current_van_direction: adj_info.direction,
                current_state: next_state,
            }
        );
        return;
    }
}