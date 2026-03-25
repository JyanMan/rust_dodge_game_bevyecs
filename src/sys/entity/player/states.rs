use crate::components::*;
use crate::components::states::*;

pub fn movement_state() -> StateMachine<MovementState> {
    let mut state_m = StateMachine::new(MovementState::Idle, idle());
    state_m.add_state(running());
    state_m.add_state(start_dodge());
    state_m.add_state(dodge_end());
    state_m.add_state(dodging());
    state_m.add_state(lerping());
    state_m.add_state(jump());
    state_m
}
pub fn combat_state() -> StateMachine<CombatState> {
    let mut state_m = StateMachine::new(CombatState::Idle, idle_combat());
    state_m.add_state(start_attack());
    state_m.add_state(attacking());
    state_m.add_state(knocked());
    state_m.add_state(stop_attacking());
    state_m
}
pub fn walker_anim() -> StateMachine<WalkerAnimState> {
    let mut state_m = StateMachine::new(WalkerAnimState::Idle, idle_anim());
    state_m.add_state(running_anim());
    state_m.add_state(rising_anim());
    state_m.add_state(falling_anim());
    state_m.add_state(attacking_anim());
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

pub fn jump() -> State<MovementState> {
    State {
        entries: 
            StateConditions::accept_all()
        ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: MovementState::StartJump
    }
}

pub fn start_attack() -> State<CombatState> {
    State {
        entries: StateConditions::accept_all() ,
        exits: 
            StateConditions::new(&[ CombatState::Attacking, ])
        ,
        // duration is dictated by weapon
        duration: None,
        next_state: None,
        id: CombatState::StartAttack
    }
}

pub fn attacking() -> State<CombatState> {
    State {
        entries: StateConditions::new(&[ CombatState::StartAttack ]) ,
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
            StateConditions::accept_all(),
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

pub fn idle_anim() -> State<WalkerAnimState> {
    State::accept_all(WalkerAnimState::Running)
}
pub fn running_anim() -> State<WalkerAnimState> {
    State::accept_all(WalkerAnimState::Running)
}
pub fn rising_anim() -> State<WalkerAnimState> {
    State::accept_all(WalkerAnimState::Rising)
}
pub fn falling_anim() -> State<WalkerAnimState> {
    State::accept_all(WalkerAnimState::Falling)
}
pub fn attacking_anim() -> State<WalkerAnimState> {
    State::accept_all(WalkerAnimState::Attacking)
}
