use crate::core::renderer::*;
use crate::components::position::*;
use crate::components::sprite::*;
use crate::components::entity::PlayerData;
use crate::ecs::ecs::*;

pub fn sprite_draw(ecs: &mut ECS, renderer: &mut Renderer) {
    let query = ecs.query_comp::<(&Position, &Sprite)>();

    for (e, pos, sprite) in query {
        renderer.draw_to_cam(sprite, pos, 1.0);
        if ecs.has_component::<PlayerData>(e) {
            renderer.camera.set_target(&pos);
        }
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
