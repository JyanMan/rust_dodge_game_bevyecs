use crate::components::*;

pub fn idle() -> State {
    State {
        exits: || -> StateConditions { StateConditions::accept_all() },
        duration: None,
        next_state: None,
        id: StateId::Idle
    }
}

pub fn running() -> State {
    State {
        exits: || -> StateConditions { StateConditions::accept_all() },
        duration: None,
        next_state: None,
        id: StateId::Running
    }
}

// change state transfer to air attack
pub fn falling() -> State {
    State {
        exits: || -> StateConditions { StateConditions::accept_all() },
        duration: None,
        next_state: None,
        id: StateId::Falling
    }
}

pub fn rising() -> State {
    State {
        exits: || -> StateConditions { StateConditions::accept_all() },
        duration: None,
        next_state: None,
        id: StateId::Rising
    }
}

pub fn start_dodge() -> State {
    println!("wtf this happened\n");
    State {
        exits: || -> StateConditions {
            StateConditions::new(&[
                StateId::DodgeAttacking, // hecking feature...
                StateId::Dodging,
            ])
        },
        duration: None,
        next_state: None,
        id: StateId::StartDodge
    }
}

pub fn dodging() -> State {
    println!("am now dodging");
    State {
        exits: || -> StateConditions {
            StateConditions::new(&[
                StateId::DodgeAttacking, // hecking feature...
                StateId::DodgeLerping
            ])
        },
        duration: None,
        next_state: None,
        id: StateId::Dodging
    }
}

pub fn lerping() -> State {
    println!("LERPING");
    State {
        exits: || -> StateConditions {
            StateConditions::new(&[
                StateId::Knocked,
                StateId::Idle,
                StateId::Running,
                StateId::Falling,
                StateId::Rising,
            ])
        },
        duration: None,
        next_state: None,
        id: StateId::Dodging
    }
}

pub fn attacking() -> State {
    State {
        exits: || -> StateConditions {
            StateConditions::new(&[
                StateId::DodgeAttacking,
                StateId::Knocked,
            ])
        },
        // duration is dictated by weapon
        duration: None,
        next_state: None,
        id: StateId::Attacking
    }
}

pub fn stop_attacking() -> State {
    State {
        exits: || -> StateConditions {
            StateConditions::new(&[
                StateId::Knocked,
                StateId::Idle,
                StateId::Falling,
                StateId::Rising
            ])
        },
        // duration is dictated by weapon
        duration: None,
        next_state: None,
        id: StateId::StopAttacking
    }
}
