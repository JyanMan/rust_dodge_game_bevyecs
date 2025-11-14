use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::entity::*;
use crate::components::Transform;
use crate::components::sprite::*;
use crate::components::Vector2;
// use crate::components::WeaponData;

pub fn sprite_system_draw(world: &mut World, renderer: &mut Renderer) {
    let mut query = world.query::<(&Transform, &Sprite)>();

    for (trans, sprite) in query.iter(world) {
        // renderer.camera.set_target(trans.global);
        if !sprite.visible {
            continue;
        }
        renderer.draw_to_cam(sprite, trans.global, 1.0);
    }
}
