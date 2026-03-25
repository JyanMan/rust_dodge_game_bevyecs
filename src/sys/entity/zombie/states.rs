use crate::components::*;
use crate::components::states::*;

pub fn movement_state() -> StateMachine<MovementState> {
    let mut state_m = StateMachine::new(MovementState::Idle, idle());
    state_m.add_state(running());
    state_m
}
pub fn combat_state() -> StateMachine<CombatState> {
    let mut state_m = StateMachine::new(CombatState::Idle, idle_combat());
    state_m.add_state(knocked());
    state_m.add_state(start_attack());
    state_m.add_state(attacking());
    state_m.add_state(stop_attacking());
    // state_m.add_state(StateId::Running, running());
    // state_m.add_state(StateId::Attacking, attacking());
    // state_m.add_state(StateId::StopAttacking, stop_attacking());
    state_m
}

pub fn enemy_state() -> StateMachine<EnemyState> {
    let mut state_m = StateMachine::new(EnemyState::Idle, idle_enemy());
    state_m
}

pub fn idle_enemy() -> State<EnemyState> {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::accept_all() ,
        duration: None,
        next_state: None,
        id: EnemyState::Idle
    }
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

pub fn start_attack() -> State<CombatState> {
    State {
        entries:  StateConditions::accept_all() ,
        exits:  StateConditions::new(&[ CombatState::Attacking ]) ,
        duration: None,
        next_state: None,
        id: CombatState::StartAttack
    }
}

pub fn attacking() -> State<CombatState> {
    State {
        entries:  StateConditions::new(&[ CombatState::StartAttack ]) ,
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
