use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::position::*;
use crate::components::sprite::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::systems::player_system::*;

pub fn sprite_draw_system() -> DrawFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        let entities = ecs.query_entities(&[
            TypeId::of::<Position>(),
            TypeId::of::<Sprite>(),
            // TypeId::of::<PlayerTag>(),
        ]);

        for e in entities {
            if let (Some(pos), Some(sprite)) = (
                ecs.get_component::<Position>(e),
                ecs.get_component::<Sprite>(e),
            ) {
                // if let Some(_p_tag) = ecs.get_component::<PlayerTag>(e) {
                //     let p_pos = pos - renderer.camera.get_pos();
                //     renderer.draw(sprite, &p_pos, 1.0);
                // }
                // else {
                // }
                renderer.draw_to_cam(sprite, pos, 1.0);

                if let Some(_p_tag) = ecs.get_component::<PlayerTag>(e) {
                    renderer.camera.set_target(&pos);
                }
            }
        }
    })
}
