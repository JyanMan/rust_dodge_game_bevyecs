use bevy_ecs::prelude::*;
use crate::core::*;
use crate::components::*;

pub fn render_all_obb(world: &mut World, renderer: &mut Renderer) {
    let mut query = world.query::<&OBB>();

    for obb in query.iter(world) {
        // renderer.camera.set_target(trans.global);
        obb.draw(renderer);
    }
}
