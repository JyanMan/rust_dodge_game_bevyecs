use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn update(query: Query<&Transform, With<PlayerTag>>, mut camera: ResMut<Camera>) {
    // let mut renderer = world.query_filtered::<&Transform, With<PlayerTag>>();
    // let mut query = world.query_filtered::<&Transform, With<PlayerTag>>();

    for trans in &query {
        camera.set_target(trans.pos);
        camera.update();
    }
}
