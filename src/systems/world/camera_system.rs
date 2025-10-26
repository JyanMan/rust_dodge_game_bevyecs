use bevy_ecs::prelude::*;
use crate::components::Transform;
use crate::components::entity::*;
use crate::Renderer;

pub fn camera_system_update(world: &mut World, renderer: &mut Renderer) {
    let mut query = world.query_filtered::<&Transform, With<PlayerTag>>();

    for trans in query.iter(world) {
        renderer.camera.set_target(trans.global);
        renderer.camera.update();
    }
}
