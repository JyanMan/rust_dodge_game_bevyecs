use crate::components::*;

pub fn state_machine() -> StateMachine {
    let mut state_m = StateMachine::new(StateId::Idle, idle());
    state_m.add_state(StateId::Running, running());
    state_m.add_state(StateId::StartDodge, start_dodge());
    state_m.add_state(StateId::DodgeEnd, dodge_end());
    state_m.add_state(StateId::Dodging, dodging());
    state_m.add_state(StateId::DodgeLerping, lerping());
    state_m.add_state(StateId::Attacking, attacking());
    state_m.add_state(StateId::StopAttacking, stop_attacking());
    state_m
}

pub fn idle() -> State {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: StateId::Idle
    }
}

pub fn running() -> State {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: StateId::Running
    }
}

// change state transfer to air attack
pub fn falling() -> State {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: StateId::Falling
    }
}

pub fn rising() -> State {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: StateId::Rising
    }
}

pub fn start_dodge() -> State {
    // println!("wtf this happened\n");
    State {
        entries:  StateConditions::accept_all() ,
        exits: 
            StateConditions::new(&[
                StateId::DodgeAttacking, // hecking feature...
                StateId::Dodging,
            ])
        ,
        duration: None,
        next_state: None,
        id: StateId::StartDodge
    }
}

pub fn dodging() -> State {
    // println!("am now dodging");
    State {
        entries: 
            StateConditions::new(&[
                StateId::StartDodge   
            ])
        ,
       exits: 
            StateConditions::new(&[
                StateId::DodgeAttacking, // hecking feature...
                StateId::DodgeLerping
            ])
        ,
        duration: None,
        next_state: None,
        id: StateId::Dodging
    }
}

pub fn lerping() -> State {
    State {
        entries: 
            StateConditions::new(&[
                StateId::Dodging,
                StateId::DodgeAttacking
            ])
        ,
       exits: 
            StateConditions::new(&[
                StateId::Knocked,
                StateId::DodgeEnd,
                StateId::StartDodge
            ])
        ,
        duration: None,
        next_state: None,
        id: StateId::DodgeLerping
    }
}

pub fn dodge_end() -> State {
    State {
        entries: 
            StateConditions::new(&[
                StateId::DodgeLerping,
            ])
        ,
       exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: StateId::DodgeEnd
    }
}

pub fn attacking() -> State {
    State {
        entries: StateConditions::accept_all() ,
        exits: 
            StateConditions::new(&[
                StateId::StartDodge,
                StateId::StopAttacking,
                StateId::Knocked,
            ])
        ,
        // duration is dictated by weapon
        duration: None,
        next_state: None,
        id: StateId::Attacking
    }
}

pub fn stop_attacking() -> State {
    // println!("broda you're stoppin no?");
    State {
        entries: StateConditions::new(&[ StateId::Attacking ]) ,
        exits: 
            StateConditions::new(&[
                StateId::Knocked,
                StateId::Idle,
                StateId::Falling,
                StateId::Rising
            ]),
        // duration is dictated by weapon
        duration: None,
        next_state: None,
        id: StateId::StopAttacking
    }
}
