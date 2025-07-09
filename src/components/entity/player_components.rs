//#[derive(Clone, Default)]
//pub struct PlayerState(u8);

//impl PlayerState {
//    pub fn has(&self, state: PState) -> bool {
//        self.0 & state.bit() != 0
//    }
//
//    pub fn set(&mut self, state: PState) {
//        self.0 |= state.bit();
//    }
//
//    pub fn clear(&mut self, state: PState) {
//        self.0 &= !state.bit();
//    }
//
//    pub fn raw(&self) -> u8 {
//        self.0 as u8
//    }
//}

#[repr(u8)]
#[derive(Clone, PartialEq)]
pub enum PlayerState {
    Rest,
    // Jumping,
    Dodging,
    Lerping
}

//impl PState {
//    pub fn bit(self) -> u8 {
//        1 << (self as u8)
//    }
//}

#[derive(Clone)]
pub struct PlayerData {
    pub state: PlayerState,

    pub run_dir: i32,

    pub can_jump: bool,
    pub can_jump_timer: f32,
    pub jump_delay: f32,

    pub can_dodge: bool,
    pub dodge_timer: f32,
    pub dodge_speed: f32,
    pub dodge_cd: f32,
    pub dodge_duration: f32,
    pub dodge_min: f32,
    pub dodge_max: f32,
    pub lerp_timer: f32,
    pub lerp_duration: f32,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            state: PlayerState::Rest,

            run_dir: 0,

            // jumping: false,
            can_jump: false,
            can_jump_timer: 0.0,
            jump_delay: 0.05,

            can_dodge: true,
            dodge_timer: 0.0,
            dodge_speed: 10.0,
            dodge_cd: 0.2,
            dodge_duration: 0.01,
            dodge_min: 40.0,
            dodge_max: 120.0,
            lerp_timer: 0.0,
            lerp_duration: 0.1,
        }
    }
}

#[derive(Default, Clone)]
pub struct PlayerInput {
    pub dodge: bool,
    pub jump: bool,
    pub left: bool,
    pub right: bool,
}
