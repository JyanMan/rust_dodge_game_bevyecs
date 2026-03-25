use crate::components::*;
use crate::components::states::*;

pub fn movement_state() -> StateMachine<MovementState> {
    let mut state_m = StateMachine::new(MovementState::Idle, idle());
    // state_m.add_state(StateId::Running, running());
    // state_m.add_state(StateId::Attacking, attacking());
    // state_m.add_state(StateId::StopAttacking, stop_attacking());
    state_m
}
pub fn combat_state() -> StateMachine<CombatState> {
    let mut state_m = StateMachine::new(CombatState::Idle, idle_combat());
    state_m.add_state(knocked());
    state_m.add_state(attacking());
    state_m.add_state(stop_attacking());
    // state_m.add_state(StateId::Running, running());
    // state_m.add_state(StateId::Attacking, attacking());
    // state_m.add_state(StateId::StopAttacking, stop_attacking());
    state_m
}

pub fn idle() -> State<MovementState> {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: MovementState::Idle
    }
}

pub fn idle_combat() -> State<CombatState> {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: CombatState::Idle
    }
}

pub fn knocked() -> State<CombatState> {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: CombatState::Knocked
    }
}

pub fn attacking() -> State<CombatState> {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::new(&[ CombatState::StopAttacking]) ,
        duration: None,
        next_state: None,
        id: CombatState::Attacking
    }
}

pub fn stop_attacking() -> State<CombatState> {
    State {
        entries:  StateConditions::new(&[ CombatState::Attacking ]) ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: CombatState::StopAttacking
    }
}

// pub fn running() -> State {
//     State {
//         entries:  StateConditions::accept_all() ,
//         exits:  StateConditions::accept_all() ,
//         duration: None,
//         next_state: None,
//         id: StateId::Running
//     }
// }

// // change state transfer to air attack
// pub fn falling() -> State {
//     State {
//         entries:  StateConditions::accept_all() ,
//         exits:  StateConditions::accept_all() ,
//         duration: None,
//         next_state: None,
//         id: StateId::Falling
//     }
// }


// pub fn attacking() -> State {
//     State {
//         entries: StateConditions::accept_all() ,
//         exits: 
//             StateConditions::new(&[
//                 StateId::StartDodge,
//                 StateId::StopAttacking,
//                 StateId::Knocked,
//             ])
//         ,
//         // duration is dictated by weapon
//         duration: None,
//         next_state: None,
//         id: StateId::Attacking
//     }
// }

// pub fn stop_attacking() -> State {
//     // println!("broda you're stoppin no?");
//     State {
//         entries: StateConditions::new(&[ StateId::Attacking ]) ,
//         exits: 
//             StateConditions::new(&[
//                 StateId::Knocked,
//                 StateId::Idle,
//                 StateId::Falling,
//                 StateId::Rising
//             ]),
//         // duration is dictated by weapon
//         duration: None,
//         next_state: None,
//         id: StateId::StopAttacking
//     }
// }
