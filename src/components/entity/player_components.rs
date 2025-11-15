use bevy_ecs::prelude::*;

#[repr(u8)]
#[derive(Clone, PartialEq)]
pub enum PlayerState {
    Rest,
    Dodging,
    Lerping
}

#[derive(Component, Clone)]
pub struct PlayerData {
    pub state: PlayerState,
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

            // jumping: false,
            can_jump: false,
            can_jump_timer: 0.0,
            jump_delay: 0.05,

            can_dodge: true,
            dodge_timer: 0.0,
            dodge_speed: 7.0,
            dodge_cd: 0.2,
            dodge_duration: 0.1,
            dodge_min: 40.0,
            dodge_max: 120.0,
            lerp_timer: 0.0,
            lerp_duration: 0.2,
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct PlayerInput {
    pub dodge: bool,
    pub jump: bool,
    pub left: bool,
    pub right: bool,
}
