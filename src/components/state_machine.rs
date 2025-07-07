
pub enum State {
    Chasing,
    Attacking,
    Patrolling,
    Knocked
}

pub struct StateMachine {
    curr_state: State,
}

impl StateMachine {
    pub fn set_state(&mut self, new_state: State) {
        self.curr_state = new_state;
    }
}
