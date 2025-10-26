use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct DeltaTimeRes {
    pub delta_time: f32,
}
