use crate::components::*;
use strum_macros::EnumDiscriminants;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct AttackData {
    pub attack_dir: Vector2,
}

#[derive(Clone, PartialEq, EnumDiscriminants, Debug)]
pub enum CombatState {
    Idle,
    StartAttack,
    // let Attacking and Stop Attacking be set by weapon
    Attacking,
    StopAttacking,
    Knocked,
}
impl StateId for CombatState{
    fn usize(self) -> usize {
        CombatStateDiscriminants::from(self) as usize
    }
    fn bit_mask(self) -> u32 {
        1u32 << (CombatStateDiscriminants::from(self) as u32)
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

#[derive(Clone, PartialEq, Debug)]
pub enum WalkerAnimState {
    Idle,
    Falling,
    Rising,
    Running,
    Attacking,
}
impl StateId for WalkerAnimState {
    fn usize(self) -> usize {
        self as usize
    }
    fn bit_mask(self) -> u32 {
        1u32 << (self as u32)
    }
}
impl WalkerAnimState {
    fn count() -> usize {
        Self::Attacking.usize() + 1
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
