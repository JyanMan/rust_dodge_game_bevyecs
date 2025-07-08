#[derive(Clone, Default)]
pub struct PlayerState(u8);

impl PlayerState {
    pub fn has(&self, state: PState) -> bool {
        self.0 & state.bit() != 0
    }

    pub fn set(&mut self, state: PState) {
        self.0 |= state.bit();
    }

    pub fn clear(&mut self, state: PState) {
        self.0 &= !state.bit();
    }
}

#[repr(u8)]
#[derive(Clone)]
pub enum PState {
    Jumping,
    CanJump,
    Dodging,
    Lerping
}

impl PState {
    pub fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

#[derive(Clone)]
pub struct PlayerData {
    pub state: PlayerState,

    pub run_dir: i32,

    //pub jumping: bool,
    //pub can_jump: bool,
    pub can_jump_timer: f32,
    pub jump_delay: f32,

    pub dodge_timer: f32,
    pub dodge_speed: f32,
    pub dodge_cd: f32,
    pub dodge_duration: f32,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            state: PlayerState::default(),

            run_dir: 0,

            // jumping: false,
            // can_jump: false,
            can_jump_timer: 0.0,
            jump_delay: 0.05,

            dodge_timer: 0.0,
            dodge_speed: 10.0,
            dodge_cd: 0.2,
            dodge_duration: 0.1,
        }
    }
}

