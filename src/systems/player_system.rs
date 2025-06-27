// use std::collections::HashSet; use std::any::Any;
// use crate::core::renderer::*;
// use crate::components::position::*;
// use crate::components::sprite::*;
// use crate::ecs::system::*;
// use crate::ecs::entity::*;
// use crate::ecs::ecs::*;
// use crate::managers::asset_manager::*;
// use crate::math_helper::*;
// 
// #[derive(Default)]
// pub struct PlayerTag {}
// 
// #[derive(Default)]
// pub struct PlayerSystem {
//     entities: HashSet<Entity>,
// }
// 
// impl System for PlayerSystem {
//     fn entities(&mut self) -> &mut HashSet<Entity> {
//         &mut self.entities 
//     }
// }
// 
// impl PlayerSystem {
//     pub fn init(&mut self, ecs: &mut ECS, asset_m: &AssetManager) {
//         let player = ecs.create_entity();
//         let mut sprite = Sprite::new(
//             asset_m.get_texture(TextureId::Player),
//             TextureId::Player
//         );
//         sprite.set_sprite_sheet(6, 6);
//         // ecs.register_component::<PlayerTag>();
// 
//         println!("pos");
//         ecs.add_component::<Position>(player, Position { x: 10.0, y: 10.0 });
//         println!("sprite");
//         ecs.add_component::<Sprite>(player, sprite);
//         println!("player");
//         ecs.add_component::<PlayerTag>(player, PlayerTag{});
//         println!("none");
//     }
//     pub fn update(&mut self, ecs: &mut ECS, delta_time: f32) {
//     //     for entity in self.entities.iter() {
//     //         if let (Some(position), Some(sprite), Some(_player_tag)) = (
//     //             ecs.get_component::<Position>(*entity),
//     //             ecs.get_component::<Sprite>(*entity),
//     //             ecs.get_component::<PlayerTag>(*entity),
//     //         ) {
//     //             sprite.draw(
//     //                 renderer, 
//     //                 &Vector2::new(position.x, position.y),
//     //             );
//     //         }
//     //     }
//     }
// 
//     // pub fn draw(&self, ecs: &mut ECS, renderer: &mut Renderer) {
//     //     for entity in self.entities.iter() {
//     //         if let (Some(position), Some(sprite), Some(_player_tag)) = (
//     //             ecs.get_component::<Position>(*entity),
//     //             ecs.get_component::<Sprite>(*entity),
//     //             ecs.get_component::<PlayerTag>(*entity),
//     //         ) {
//     //             sprite.draw(
//     //                 renderer, 
//     //                 &Vector2::new(position.x, position.y),
//     //             );
//     //         }
//     //     }
//     // }
// }
