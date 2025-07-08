#[derive(Clone, Default, PartialEq)]
pub enum State {
    #[default]
    Idle,
    Chasing,
    Running,
    Attacking,
    Patrolling,
    Knocked,
    Aired,
}

#[derive(Clone, Default)]
pub struct StateMachine {
    curr_state: State,
}

impl StateMachine {
    pub fn set_state(&mut self, new_state: State) {
        self.curr_state = new_state;
    }

    pub fn state(&mut self) -> &State {
        &self.curr_state
    }
}
