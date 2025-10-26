use bevy_ecs::prelude::*;

#[derive(Component, Clone)]
pub struct WalkerData {
    pub run_speed: f32,
    pub accel: f32,
    pub jump_force: f32,
    pub grounded: bool,
    pub state: WalkerState
}

impl Default for WalkerData {
    fn default() -> Self {
        Self {
            run_speed: 200.0,
            accel: 50.0,
            jump_force: 300.0,
            grounded: false,
            state: WalkerState::default()
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum WalkerState {
    #[default]
    Idle,
    Running,
    Aired,
}

#[derive(Default, Clone)]
#[repr(usize)]
pub enum WalkerAnim {
    #[default]
    Idle,
    Run,
    Rise,
    Fall
}

impl WalkerAnim {
    pub const COUNT: usize = 4;
    pub fn usize(self) -> usize {
        self as usize
    }
}
