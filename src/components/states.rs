use crate::components::*;
use crate::config::KNOCK_TIME;
use strum_macros::EnumDiscriminants;

#[derive(Clone, PartialEq, EnumDiscriminants, Debug)]
pub enum CombatState {
    Idle,
    StartAttack,
    // let Attacking and Stop Attacking be set by weapon
    Attacking,
    StopAttacking,
    Knocked { dir: Vector2, force: f32 },
    KnockEnd,
}
impl StateId for CombatState{
    fn usize(self) -> usize {
        CombatStateDiscriminants::from(self) as usize
    }
    fn bit_mask(self) -> u32 {
        1u32 << (CombatStateDiscriminants::from(self) as u32)
    }
}
impl CombatState {

    pub fn default_knocked() -> Self {
        Self::Knocked { dir: Vector2::zero(), force: 0.0 }
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
            exits: StateConditions::new(&[ CombatState::KnockEnd]),
            duration: Some(KNOCK_TIME),
            next_state: Some(CombatState::KnockEnd),
            id: CombatState::default_knocked() 
        }
    }

    pub fn knock_end() -> State<CombatState> {
        State {
            entries: StateConditions::new(&[ CombatState::default_knocked() ]),
            exits: StateConditions::accept_all(),
            duration: None,
            next_state: None,
            id: CombatState::KnockEnd 
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

    pub fn idle() -> State<CombatState> {
        State {
            entries:  StateConditions::accept_all() ,
            exits:  StateConditions::accept_all() ,
            duration: None,
            next_state: None,
            id: CombatState::Idle
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MovementState {
    Idle,
    Falling,
    Rising,
    Running,
    StartDodge,
    Dodging,
    DodgeLerping,
    DodgeEnd,
    StartJump,
}
impl StateId for MovementState {
    fn usize(self) -> usize {
        self as usize
    }
    fn bit_mask(self) -> u32 {
        1u32 << (self as u32)
    }
}
impl MovementState {
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
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EnemyState {
    InAttackRange,
    Chasing,
    Idle
}
impl StateId for EnemyState {
    fn usize(self) -> usize {
        self as usize
    }
    fn bit_mask(self) -> u32 {
        1u32 << (self as u32)
    }
}

#[derive(Clone, Default, PartialEq, EnumDiscriminants)]
pub enum WeaponState {
    #[default]
    Unowned,
    Owned,
    Idle,
    StartAttack,
    Attacking,
    StartDodgeAttack,
    DodgeAttacking,
    StartPushAttack,
    PushAttacking,
    EndAttack,
    AfterEffectAttack,
}
impl StateId for WeaponState {
    fn usize(self) -> usize {
        WeaponStateDiscriminants::from(self) as usize
    }
    fn bit_mask(self) -> u32 {
        1u32 << (WeaponStateDiscriminants::from(self) as u32)
    }
}
impl WeaponState {
    pub fn idle() -> State<WeaponState> {
        State {
            entries:  StateConditions::accept_all() ,
            exits:  StateConditions::accept_all() ,
            duration: None,
            next_state: None,
            id: WeaponState::Idle
        }
    }
    pub fn start_attack() -> State<WeaponState> {
        State {
            entries:  StateConditions::accept_all() ,
            exits:  StateConditions::new(&[ WeaponState::Attacking ]),
            duration: None,
            next_state: None,
            id: WeaponState::StartAttack
        }
    }
    pub fn start_dodge_attack() -> State<WeaponState> {
        State {
            entries:  StateConditions::accept_all() ,
            exits:  StateConditions::new(&[ WeaponState::DodgeAttacking ]),
            duration: None,
            next_state: None,
            id: WeaponState::StartDodgeAttack
        }
    }
    pub fn start_push_attack() -> State<WeaponState> {
        State {
            entries:  StateConditions::accept_all() ,
            exits:  StateConditions::new(&[ WeaponState::PushAttacking ]),
            duration: None,
            next_state: None,
            id: WeaponState::StartPushAttack
        }
    }

    pub fn attacking() -> State<WeaponState> {
        State {
            entries:  StateConditions::new(&[ WeaponState::StartAttack ]) ,
            exits:  StateConditions::new(&[ WeaponState::AfterEffectAttack ]) ,
            duration: None,
            next_state: None,
            id: WeaponState::Attacking
        }
    }
    pub fn dodge_attacking() -> State<WeaponState> {
        State {
            entries:  StateConditions::new(&[ WeaponState::StartDodgeAttack ]) ,
            exits:  StateConditions::new(&[ WeaponState::AfterEffectAttack ]) ,
            duration: None,
            next_state: None,
            id: WeaponState::DodgeAttacking
        }
    }
    pub fn push_attacking() -> State<WeaponState> {
        State {
            entries:  StateConditions::new(&[ WeaponState::StartPushAttack ]) ,
            exits:  StateConditions::new(&[ WeaponState::AfterEffectAttack ]) ,
            duration: None,
            next_state: None,
            id: WeaponState::PushAttacking
        }
    }
    pub fn after_effect_attack() -> State<WeaponState> {
        State {
            entries:  StateConditions::new(&[
                WeaponState::Attacking,
                WeaponState::DodgeAttacking,
                WeaponState::PushAttacking,
            ]) ,
            exits:  StateConditions::new(&[ WeaponState::EndAttack ]) ,
            duration: None,
            next_state: None,
            id: WeaponState::AfterEffectAttack
        }
    }
    pub fn end_attack() -> State<WeaponState> {
        State {
            entries:  StateConditions::new(&[ WeaponState::AfterEffectAttack ]),
            exits:  StateConditions::accept_all() ,
            duration: None,
            next_state: None,
            id: WeaponState::EndAttack
        }
    }
}
