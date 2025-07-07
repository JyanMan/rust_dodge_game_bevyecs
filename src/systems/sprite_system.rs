use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::position::*;
use crate::components::sprite::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::ecs::ecs_query::*;
use crate::systems::player_system::*;

pub fn sprite_draw(ecs: &mut ECS, renderer: &mut Renderer) {
    let query = ecs.query_comp::<(&Position, &Sprite)>();

    for (e, pos, sprite) in query {
        renderer.draw_to_cam(sprite, pos, 1.0);
        if ecs.has_component::<PlayerTag>(e) {
            renderer.camera.set_target(&pos);
        }
    }

    for (_e, _p_tag, pos) in ecs.query_comp::<(&PlayerTag, &Position)>() {
    }

        // if let Some(_p_tag) = ecs.get_component::<PlayerTag>(e) {
        // }
    //let entities = query_entities!(ecs, Position, Sprite);
    //for e in entities {
    //    if let (Some(pos), Some(sprite)) = 
    //        ecs.query_tuple::<(&Position, &Sprite)>(e)
    //     {
    //        renderer.draw_to_cam(sprite, pos, 1.0);

    //        if let Some(_p_tag) = ecs.get_component::<PlayerTag>(e) {
    //            renderer.camera.set_target(&pos);
    //        }
    //    }
    //}
}
