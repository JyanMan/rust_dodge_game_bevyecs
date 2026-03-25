use crate::components::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
        self as usize
    }
    fn bit_mask(self) -> u32 {
        1u32 << (self as u32)
    }
}

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq)]
pub enum WalkerAnimState{
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

#[derive(Clone, PartialEq, Eq, Hash)]
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
