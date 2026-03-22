use bevy_ecs::prelude::*;
use std::collections::HashMap;
use bevy_ecs::storage::SparseSet;
// use std::vec::Vec;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct State {
    pub entries: StateConditions,
    pub exits: StateConditions,
    pub duration: Option<f32>,
    pub next_state: Option<StateId>,
    pub id: StateId
}

#[allow(clippy::manual_map)]
impl State {
    pub fn exits(&self) -> &StateConditions {
        &self.exits
    }
    pub fn entries(&self) -> &StateConditions {
        &self.entries
        
    }

    pub fn duration(&self) -> Option<f32> {
        if let Some(duration) = self.duration {
            Some(duration)
        }
        else {
            None
        }
    }

    pub fn next_state(&self) -> Option<StateId> {
        if let Some(next_state_id) = self.next_state.as_ref() {
            Some(next_state_id.clone())
        }
        else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
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
    DodgeEnd,
    AirAttack,
    Knocked,
}

impl StateId {
    pub fn usize(self) -> usize {
        self as usize
    }
}

#[derive(Component)]
pub struct StateMachine {
    state: State,
    timer: f32,
    // states_set: HashMap<StateId, State>
    states_set: SparseSet<usize, State>
}

impl StateMachine {
    pub fn new(init_state: State) -> Self {
        Self {
            state: init_state,
            timer: 0.0,
            states_set: SparseSet::new()
        }
    }

    pub fn add_state(&mut self, id: StateId, state: State) {
        self.states_set.insert(id.usize(), state);
    }

    pub fn curr_state(&self) -> StateId { self.state.id.clone() }

    // only allows new state that is connected to the current state
    pub fn set_state(&mut self, id: StateId) {
        if self.state.id == id {
            return;
        }
        let next_state = self.states_set.get(id.clone().usize()).expect("state does not exist");

        if self.state.exits().contains(&id)
            && next_state.entries().contains(&self.state.id)
        {
            self.state = next_state.clone();
            self.timer = 0.0;
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
            let next_state_id = self.state.next_state()
                .expect("unexpected next_state not defined when duration was set...");
            self.state = self.states_set.get(next_state_id.usize()).unwrap().clone();
        }
    }
}
