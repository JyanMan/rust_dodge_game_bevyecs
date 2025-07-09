#[derive(Clone, Default, PartialEq)]
pub enum WalkerState {
    #[default]
    Idle,
    Running,
    Aired,
}
