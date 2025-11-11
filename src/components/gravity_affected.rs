use bevy_ecs::prelude::*;

#[derive(Component)]
#[component(storage = "Table")]
pub struct GravityAffected(pub bool);
