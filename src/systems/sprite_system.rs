use std::collections::HashSet;
use std::any::Any;
use std::any::TypeId;
use crate::components::position::*;
use crate::core::renderer::*;
use crate::components::sprite::*;
use crate::ecs::system::*;
use crate::ecs::entity::*;
use crate::ecs::ecs::*;
use crate::math_helper::*;
use crate::managers::asset_manager::*;
use crate::core::renderer::*;

pub fn sprite_startup_system() -> SystemFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {

        let test = ecs.create_entity();
        ecs.add_component::<Sprite>(test, Sprite::new(
            renderer.asset_m.get_texture(TextureId::Player), TextureId::Player
        ));
        ecs.add_component::<Position>(test, Position { x: 10.0, y: 10.0 });
    })
}

pub fn sprite_draw_system() -> SystemFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        let entities = ecs.query_entities(&[
            TypeId::of::<Position>(),
            TypeId::of::<Sprite>(),
        ]);

        for e in entities {
            if let (Some(pos), Some(sprite)) = (
                ecs.get_component::<Position>(e),
                ecs.get_component::<Sprite>(e),
            ) {
                sprite.draw(renderer, &Vector2::new(pos.x, pos.y));
                // pos.x += vel.dx * dt;
                // pos.y += vel.dy * dt;
            }
        }
    })
}

// #[derive(Default)]
// pub struct SpriteSystem {
//     entities: HashSet<Entity>,
// }
// 
// impl System for SpriteSystem {
//     fn entities(&mut self) -> &mut HashSet<Entity> {
//         &mut self.entities 
//     }
// }
// 
// impl SpriteSystem {
//     pub fn init(&self, ecs: &mut ECS, asset_m: &AssetManager) {
//         for entity in self.entities.iter() {
//             let sprite = ecs.get_component_mut::<Sprite>(*entity).unwrap();
//             sprite.init(asset_m.get_texture(TextureId::Player), TextureId::Player);
//         }
//     }
// 
//     pub fn draw(&self, ecs: &mut ECS, renderer: &mut Renderer) {
//         for entity in self.entities.iter() {
//             if let (Some(position), Some(sprite)) = (
//                 ecs.get_component::<Position>(*entity),
//                 ecs.get_component::<Sprite>(*entity),
//             ) {
//                 sprite.draw(
//                     renderer, 
//                     &Vector2::new(position.x, position.y),
//                 );
//             }
//         }
//     }
// }
