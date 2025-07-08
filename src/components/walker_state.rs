#[derive(Clone, Default, PartialEq)]
pub enum WalkerState {
    #[default]
    Idle,
    Chasing,
    Running,
    Attacking,
    Patrolling,
    Knocked,
    Aired,
}
