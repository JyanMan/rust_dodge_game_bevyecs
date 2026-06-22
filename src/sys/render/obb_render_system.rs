use bevy_ecs::prelude::*;
use sdl2::render::*;
use crate::core::*;
use crate::components::*;

// TODO
// pub fn render_all_obb(world: &mut World, canvas: &mut WindowCanvas, renderer: &mut Renderer) {
//     let mut query = world.query::<&OBB>();

//     for obb in query.iter(world) {
//         if obb.disabled {
//             continue;
//         }
//         // renderer.camera.set_target(trans.global);
//         obb.draw(canvas, renderer);
//     }
// }
