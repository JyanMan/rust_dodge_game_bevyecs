use crate::components::walker_state::*;

#[derive(Clone)]
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

