use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Health {
    current: i32,
    max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            current: max,
            max: max
        }
    }
}
