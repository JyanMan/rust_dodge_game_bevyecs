use crate::components::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum CombatState {
    Idle,
    StartAttack,
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MovementState {
    Idle,
    Falling,
    Rising,
    Running,
    StartDodge,
    Dodging,
    DodgeLerping,
    DodgeEnd,
}
impl StateId for MovementState {
    fn usize(self) -> usize {
        self as usize
    }
    fn bit_mask(self) -> u32 {
        1u32 << (self as u32)
    }
}
