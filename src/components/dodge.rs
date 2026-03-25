use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct DodgeCD {
    timer: f32,
    dur: f32
}
impl DodgeCD {
    pub fn new(dur: f32) -> Self {
        Self {
            timer: 0.0,
            dur
        }
    }
}

#[derive(Component)]
pub struct LerpCD {
    timer: f32,
    dur: f32
}
impl LerpCD {
    pub fn new(dur: f32) -> Self {
        Self {
            timer: 0.0,
            dur
        }
    }
}
