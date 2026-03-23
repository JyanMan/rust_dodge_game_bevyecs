use crate::components::*;
use crate::components::states::*;

pub fn movement_state() -> StateMachine<MovementState> {
    let mut state_m = StateMachine::new(MovementState::Idle, idle());
    state_m.add_state(MovementState::Running, running());
    state_m.add_state(MovementState::StartDodge, start_dodge());
    state_m.add_state(MovementState::DodgeEnd, dodge_end());
    state_m.add_state(MovementState::Dodging, dodging());
    state_m.add_state(MovementState::DodgeLerping, lerping());
    state_m
}
pub fn combat_state() -> StateMachine<CombatState> {
    let mut state_m = StateMachine::new(CombatState::Idle, idle_combat());
    state_m.add_state(CombatState::Attacking, attacking());
    state_m.add_state(CombatState::Knocked, knocked());
    state_m.add_state(CombatState::StopAttacking, stop_attacking());
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

pub fn running() -> State<MovementState> {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: MovementState::Running
    }
}

// change state transfer to air attack
pub fn falling() -> State<MovementState> {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: MovementState::Falling
    }
}

pub fn rising() -> State<MovementState> {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: MovementState::Rising
    }
}

pub fn start_dodge() -> State<MovementState> {
    // println!("wtf this happened\n");
    State {
        entries:  StateConditions::accept_all() ,
        exits: 
            StateConditions::new(&[
                MovementState::Dodging,
            ])
        ,
        duration: None,
        next_state: None,
        id: MovementState::StartDodge
    }
}

pub fn dodging() -> State<MovementState> {
    // println!("am now dodging");
    State {
        entries: 
            StateConditions::new(&[
                MovementState::StartDodge   
            ])
        ,
       exits: 
            StateConditions::new(&[
                MovementState::DodgeLerping
            ])
        ,
        duration: None,
        next_state: None,
        id: MovementState::Dodging
    }
}

pub fn lerping() -> State<MovementState> {
    State {
        entries: 
            StateConditions::new(&[
                MovementState::Dodging,
            ])
        ,
       exits: 
            StateConditions::new(&[
                MovementState::DodgeEnd,
                MovementState::StartDodge
            ])
        ,
        duration: None,
        next_state: None,
        id: MovementState::DodgeLerping
    }
}

pub fn dodge_end() -> State<MovementState> {
    State {
        entries: 
            StateConditions::new(&[
                MovementState::DodgeLerping,
            ])
        ,
       exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: MovementState::DodgeEnd
    }
}

pub fn attacking() -> State<CombatState> {
    State {
        entries: StateConditions::accept_all() ,
        exits: 
            StateConditions::new(&[ CombatState::StopAttacking, ])
        ,
        // duration is dictated by weapon
        duration: None,
        next_state: None,
        id: CombatState::Attacking
    }
}

pub fn knocked() -> State<CombatState> {
    State {
        entries: StateConditions::accept_all(),
        exits: StateConditions::new(&[ CombatState::Idle ]),
        duration: None,
        next_state: None,
        id: CombatState::Knocked
    }
}

pub fn stop_attacking() -> State<CombatState> {
    // println!("broda you're stoppin no?");
    State {
        entries: StateConditions::new(&[ CombatState::Attacking ]) ,
        exits: 
            StateConditions::new(&[
                CombatState::Knocked,
                CombatState::Idle,
            ]),
        // duration is dictated by weapon
        duration: None,
        next_state: None,
        id: CombatState::StopAttacking
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
