use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::position::*;
use crate::components::sprite::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::ecs::ecs_query::*;
use crate::systems::player_system::*;

pub fn sprite_draw_system() -> DrawFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        let query = ecs.query_components_2::<Position, Sprite>();
        for (pos, sprite) in query {
            renderer.draw_to_cam(sprite, pos, 1.0);
            renderer.camera.set_target(&pos);

            // if let Some(_p_tag) = ecs.get_component::<PlayerTag>(e) {
            // }
        }
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
    })
}
