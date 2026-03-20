use bevy_ecs::prelude::*;
use std::vec::Vec;

pub struct StateConditions {
    conds: u32,
}
impl StateConditions {
    // use when a state can only exit to some specific states
    pub fn new(conditions: &[StateId]) -> Self {
        let mut conds = 0u32;
        for state_id in conditions {
            conds |= 1u32 << (state_id.clone() as u32);
        }
        Self { conds  }
    }

    // when a state can exit to any other state
    pub fn accept_all() -> Self { Self { conds: u32::MAX, } }

    pub fn contains(&self, state: &StateId) -> bool {
        let bit_state = 1u32 << (state.clone() as u32);
        // check if bit is within conditions
        self.conds as u32 & bit_state != 0
    }
}

// no need for entries, each exit connects already
pub struct State {
    pub exits: fn() -> StateConditions,
    pub duration: Option<f32>,
    pub next_state: Option<fn() -> State>,
    pub id: StateId
}

impl State {
    pub fn exits(&self) -> StateConditions {
        let exit_fn = self.exits;
        exit_fn()
    }

    pub fn duration(&self) -> Option<f32> {
        if let Some(duration) = self.duration {
            Some(duration)
        }
        else {
            None
        }
    }

    pub fn next_state(&self) -> Option<State> {
        if let Some(next_state_fn) = self.next_state {
            Some(next_state_fn())
        }
        else {
            None
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum StateId {
    Idle,
    Chasing,
    Running,
    Attacking,
    StopAttacking,
    StartDodge,
    Dodging,
    Falling,
    Rising,
    DodgeAttacking,
    DodgeLerping,
    AirAttack,
    Knocked,
}

#[derive(Component)]
pub struct StateMachine {
    state: State,
    timer: f32,
}

impl StateMachine {
    pub fn new(init_state: State) -> Self {
        Self {
            state: init_state,
            timer: 0.0
        }
    }

    pub fn curr_state(&self) -> StateId { self.state.id.clone() }

    // only allows new state that is connected to the current state
    pub fn set_state(&mut self, new_state: State) {
        if self.state.exits().contains(&new_state.id) {
            self.state = new_state
        }
    }

    pub fn update_next_state_timer(&mut self, delta_time: f32) {
        let duration = {
            if let Some(duration) = self.state.duration() {
                duration
            }
            else {
                return;
            }
        };

        self.timer += delta_time;
        if self.timer >= duration {
            let next_state = self.state.next_state()
                .expect("unexpected next_state not defined when duration was set...");
            self.state = next_state;
        }
    }
}
