use bevy_ecs::prelude::*;
use crate::components::Transform;
use crate::components::entity::*;
use crate::Renderer;

pub fn camera_system_update(query: Query<&Transform, With<PlayerTag>>, mut renderer: NonSendMut<Renderer<'static>>) {
    // let mut renderer = world.query_filtered::<&Transform, With<PlayerTag>>();
    // let mut query = world.query_filtered::<&Transform, With<PlayerTag>>();

    for trans in &query {
        renderer.camera.set_target(trans.global);
        renderer.camera.update();
    }
}
