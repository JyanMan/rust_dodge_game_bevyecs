use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct RotationOffset(f32, f32, f32, f32);

impl RotationOffset {
    pub fn zero() -> Self {
        Self(1.0, 0.0, 0.0, 1.0)
    }
}
