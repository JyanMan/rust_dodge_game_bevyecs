use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::entity::*;
use crate::components::Transform;
use crate::components::sprite::*;
use crate::components::Vector2;
// use crate::components::WeaponData;
use crate::ecs::ecs::*;

pub fn sprite_system_draw(world: &mut World, renderer: &mut Renderer) {
    let mut query = world.query_filtered::<(&Transform, &Sprite), With<PlayerTag>>();

    for (trans, sprite) in query.iter(world) {
        // renderer.camera.set_target(trans.global);
        if !sprite.visible {
            continue;
        }
        renderer.draw_to_cam(sprite, trans.global, 1.0);
    }
}

pub fn sprite_draw(ecs: &mut ECS, renderer: &mut Renderer) {
    let query = ecs.query_comp::<(&Transform, &Sprite)>();

    for (e, trans, sprite) in query {
        if !sprite.visible {
            continue;
        }

        // let half_size = Vector2::new(
        //     sprite.width as f32 / 2.0,
        //     sprite.height as f32 / 2.0
        // );
        renderer.draw_to_cam(sprite, trans.global, 1.0);
        if ecs.has_component::<PlayerData>(e) {
            renderer.camera.set_target(trans.global);
        }

    }

        // if let Some(_p_tag) = ecs.get_component::<PlayerTag>(e) {
        // }
    //let entities = query_entities!(ecs, Transform, Sprite);
    //for e in entities {
    //    if let (Some(pos), Some(sprite)) = 
    //        ecs.query_tuple::<(&Transform, &Sprite)>(e)
    //     {
    //        renderer.draw_to_cam(sprite, pos, 1.0);

    //        if let Some(_p_tag) = ecs.get_component::<PlayerTag>(e) {
    //            renderer.camera.set_target(&pos);
    //        }
    //    }
    //}
}
