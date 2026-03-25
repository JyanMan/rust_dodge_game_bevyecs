use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use std::marker::PhantomData;
// use std::vec::Vec;

/// a container for a set of states
/// this is made to avoid the use of Vec<State> and O(n) lookups
/// HashMaps are slower since you typically won't have large state_sets
#[derive(Clone)]
pub struct StateConditions <S: StateId> {
    conds: u32,
    _phantom: PhantomData<S>
}

impl <S: StateId> StateConditions <S> {
    /// creates StateCondition from a state ids flat array
    /// NOTE: avoid using mulitple times, use only once per entity spawn
    /// otherwise defeats the purpose of using bit ops for performance
    pub fn new(conditions: &[S]) -> Self {
        let mut conds = 0u32;
        for state_id in conditions {
            conds |= state_id.clone().bit_mask();
        }
        Self { conds, _phantom: PhantomData::default() }
    }

    // when a state can exit to any other state
    pub fn accept_all() -> Self { Self { conds: u32::MAX, _phantom: PhantomData::default() } }

    pub fn all_except(conditions: &[S]) -> Self {
       let mut conds = u32::MAX; 
       for state_id in conditions {
           conds &= !state_id.clone().bit_mask();
       }
       Self {conds, _phantom: PhantomData::default() }
    }

    pub fn contains(&self, state: &S) -> bool {
        let bit_state = state.clone().bit_mask();
        self.conds & bit_state != 0
    }
}

/// entries: states this state can come from
/// exits: states this state struct can exit to
/// duration: none if no automatic transition | time before transition
/// next_state: none if no automatic transition | next state to auto transition to
/// id: the state id
#[derive(Clone)]
pub struct State<S: StateId> {
    pub entries: StateConditions<S>,
    pub exits: StateConditions<S>,
    pub duration: Option<f32>,
    pub next_state: Option<S>,
    pub id: S
}

#[allow(clippy::manual_map)]
impl <S: StateId> State <S> {
    pub fn accept_all(id: S) -> Self {
        Self {
            id,
            entries: StateConditions::accept_all(),
            exits: StateConditions::accept_all(),
            duration: None,
            next_state: None,
        }
    }
    pub fn exits(&self) -> &StateConditions<S> {
        &self.exits
    }
    pub fn entries(&self) -> &StateConditions<S> {
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

    pub fn next_state(&self) -> Option<S> {
        if let Some(next_state_id) = self.next_state.as_ref() {
            Some(next_state_id.clone())
        }
        else {
            None
        }
    }
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum StateId {
//     Idle,
//     Chasing,
//     Running,
//     Attacking,
//     StopAttacking,
//     StartDodge,
//     Dodging,
//     Falling,
//     Rising,
//     DodgeAttacking,
//     DodgeLerping,
//     DodgeEnd,
//     AirAttack,
//     Knocked,
// }

pub trait StateId: Clone + PartialEq {
    fn usize(self) -> usize;
    fn bit_mask(self) -> u32;
}

#[derive(Component)]
pub struct StateMachine <S: StateId + 'static> {
    state: State<S>,
    timer: f32,
    // states_set: HashMap<StateId, State>
    states_set: SparseSet<usize, State<S>>
}

/// the id is converted to usize manually in this code, no need to worry how it works to use
/// ```
/// // add the states first
/// let mut state_m = StateMachine::new(StateId::Idle, idle());
/// state_m.add_state(StateId::Running, State {..});
/// state_m.add_state(StateId::Falling, State {..});
/// if state_m.curr_state() == StateId::Running {
///   // running
/// }
/// if state_m.curr_state() == StateId::Idle{
///   // idling
///   // should be true since idle was made as initial state
/// }
/// match state_m.curr_state() {
///   StateId::Running => {..},
///   StateId::Idle => {..}
/// }
/// 
/// ```
impl <S: StateId + 'static> StateMachine <S> {
    // sets initial state and adds into set
    pub fn new(id: S, init_state: State<S>) -> Self {
        let mut states_set = SparseSet::new();
        states_set.insert(id.usize(), init_state.clone());
        Self {
            state: init_state,
            timer: 0.0,
            states_set
        }
    }

    pub fn add_state(&mut self, state: State<S>) {
        self.states_set.insert(state.id.clone().usize(), state);
    }

    pub fn curr_state(&self) -> S { self.state.id.clone() }

    // only allows new state that is connected to the current state
    pub fn set_state(&mut self, id: S) {
        if self.state.id.clone().usize() == id.clone().usize() {
            return;
        }

        // check if next state can be transitioned to
        if self.state.exits().contains(&id)
        {
            let next_state = self.states_set.get(id.clone().usize())
                .expect("state does not exist in state machine");
            if next_state.entries().contains(&self.state.id) {
                // set to next state
                self.state = next_state.clone();
                self.timer = 0.0;
            }
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
